mod codex_usage;

use codex_usage::{CodexUsage, FetchError};
use serde::Serialize;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tauri::{
    image::Image,
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, LogicalSize, Manager, PhysicalPosition, Rect, WindowEvent,
};

const POPOVER_LABEL: &str = "main";
const AUTO_REFRESH_INTERVAL: Duration = Duration::from_secs(5 * 60);
const STALE_AFTER: Duration = Duration::from_secs(10 * 60);

#[derive(Clone, Copy, Debug)]
struct TrayAnchor {
    center_x: f64,
    top_y: f64,
}

#[derive(Default)]
struct PopoverState {
    anchor: Mutex<Option<TrayAnchor>>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "status", rename_all = "camelCase")]
enum UsageStatus {
    Loading,
    Available { data: CodexUsage },
    Unavailable { message: String },
    Error { message: String },
    Stale { data: CodexUsage, message: String },
}

impl UsageStatus {
    fn data(&self) -> Option<&CodexUsage> {
        match self {
            Self::Available { data } | Self::Stale { data, .. } => Some(data),
            _ => None,
        }
    }
}

struct UsageStore {
    current: Mutex<UsageStatus>,
    refresh_in_progress: AtomicBool,
    next_refresh_at: Mutex<Instant>,
    tray_language: Mutex<TrayLanguage>,
    tray_mode: Mutex<TrayMode>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrayLanguage {
    English,
    Italian,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrayMode {
    Auto,
    FiveHours,
    Weekly,
}

impl TrayMode {
    fn from_mode(mode: &str) -> Self {
        match mode {
            "fiveHours" => Self::FiveHours,
            "weekly" => Self::Weekly,
            _ => Self::Auto,
        }
    }
}

impl TrayLanguage {
    fn from_language(language: &str) -> Self {
        if language.eq_ignore_ascii_case("it") || language.starts_with("it-") {
            Self::Italian
        } else {
            Self::English
        }
    }
}

impl Default for UsageStore {
    fn default() -> Self {
        Self {
            current: Mutex::new(UsageStatus::Loading),
            refresh_in_progress: AtomicBool::new(false),
            next_refresh_at: Mutex::new(Instant::now()),
            tray_language: Mutex::new(TrayLanguage::English),
            tray_mode: Mutex::new(TrayMode::Auto),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct PopoverLayout {
    x: f64,
    y: f64,
    pointer_x: f64,
}

#[derive(Clone, Copy)]
struct WorkArea {
    left: f64,
    top: f64,
    width: f64,
    height: f64,
}

fn calculate_popover_layout(
    anchor: TrayAnchor,
    window_width: f64,
    window_height: f64,
    work_area: WorkArea,
    scale_factor: f64,
) -> PopoverLayout {
    let edge_margin = 8.0 * scale_factor;
    let tray_gap = -4.0 * scale_factor;
    let min_x = work_area.left + edge_margin;
    let max_x = (work_area.left + work_area.width - window_width - edge_margin).max(min_x);
    let x = (anchor.center_x - window_width / 2.0).clamp(min_x, max_x);
    let min_y = work_area.top + edge_margin;
    let max_y = (work_area.top + work_area.height - window_height - edge_margin).max(min_y);
    let y = (anchor.top_y - window_height - tray_gap).clamp(min_y, max_y);
    let pointer_x =
        ((anchor.center_x - x) / scale_factor).clamp(31.0, window_width / scale_factor - 31.0);
    PopoverLayout { x, y, pointer_x }
}

fn update_tray_anchor(app: &AppHandle, pointer_x: f64, pointer_y: f64, rect: Rect) {
    let scale_factor = app
        .monitor_from_point(pointer_x, pointer_y)
        .ok()
        .flatten()
        .map(|monitor| monitor.scale_factor())
        .unwrap_or(1.0);
    let position = rect.position.to_physical::<f64>(scale_factor);
    let size = rect.size.to_physical::<f64>(scale_factor);
    let anchor = if size.width > 0.0 && size.height > 0.0 {
        TrayAnchor {
            center_x: position.x + size.width / 2.0,
            top_y: position.y,
        }
    } else {
        TrayAnchor {
            center_x: pointer_x,
            top_y: pointer_y - 12.0 * scale_factor,
        }
    };
    if let Ok(mut stored_anchor) = app.state::<PopoverState>().anchor.lock() {
        *stored_anchor = Some(anchor);
    }
}

fn position_popover(app: &AppHandle) -> Option<f64> {
    let window = app.get_webview_window(POPOVER_LABEL)?;
    let anchor = app
        .state::<PopoverState>()
        .anchor
        .lock()
        .ok()
        .and_then(|anchor| *anchor)?;
    let monitor = app
        .monitor_from_point(anchor.center_x, anchor.top_y)
        .ok()
        .flatten()
        .or_else(|| window.current_monitor().ok().flatten())?;
    let scale_factor = monitor.scale_factor();
    let size = window.outer_size().ok()?;
    let work = monitor.work_area();
    let layout = calculate_popover_layout(
        anchor,
        size.width as f64,
        size.height as f64,
        WorkArea {
            left: work.position.x as f64,
            top: work.position.y as f64,
            width: work.size.width as f64,
            height: work.size.height as f64,
        },
        scale_factor,
    );
    let _ = window.set_position(PhysicalPosition::new(layout.x.round(), layout.y.round()));
    let _ = window.emit("popover-pointer", layout.pointer_x);
    Some(layout.pointer_x)
}

#[tauri::command]
fn hide_popover(app: AppHandle) -> Result<(), String> {
    app.get_webview_window(POPOVER_LABEL)
        .ok_or_else(|| "popover window unavailable".to_string())?
        .hide()
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn sync_popover_geometry(app: AppHandle, width: f64, height: f64) -> Result<f64, String> {
    if !(340.0..=420.0).contains(&width) || !(300.0..=760.0).contains(&height) {
        return Err("popover dimensions outside safe bounds".to_string());
    }
    let window = app
        .get_webview_window(POPOVER_LABEL)
        .ok_or_else(|| "popover window unavailable".to_string())?;
    window
        .set_size(LogicalSize::new(width.round(), height.round()))
        .map_err(|error| error.to_string())?;
    let fallback = width / 2.0;
    Ok(position_popover(&app).unwrap_or(fallback))
}

#[tauri::command]
fn get_usage_state(app: AppHandle) -> UsageStatus {
    app.state::<UsageStore>()
        .current
        .lock()
        .map(|state| state.clone())
        .unwrap_or_else(|_| UsageStatus::Error {
            message: "MEROA usage state is unavailable.".into(),
        })
}

#[tauri::command]
async fn refresh_usage(app: AppHandle) -> UsageStatus {
    let app_for_refresh = app.clone();
    match tauri::async_runtime::spawn_blocking(move || refresh_usage_now(&app_for_refresh)).await {
        Ok(status) => status,
        Err(error) => {
            let status = UsageStatus::Error {
                message: format!("Could not refresh Codex usage: {error}"),
            };
            publish_usage_status(&app, status.clone());
            status
        }
    }
}

#[tauri::command]
fn set_tray_language(app: AppHandle, language: String) {
    if let Ok(mut tray_language) = app.state::<UsageStore>().tray_language.lock() {
        *tray_language = TrayLanguage::from_language(&language);
    }
    let status = get_usage_state(app.clone());
    update_tray_usage(&app, &status);
}

#[tauri::command]
fn set_tray_mode(app: AppHandle, mode: String) {
    if let Ok(mut tray_mode) = app.state::<UsageStore>().tray_mode.lock() {
        *tray_mode = TrayMode::from_mode(&mode);
    }
    let status = get_usage_state(app.clone());
    update_tray_usage(&app, &status);
}

fn toggle_popover(app: &AppHandle) {
    let Some(window) = app.get_webview_window(POPOVER_LABEL) else {
        return;
    };

    if window.is_visible().unwrap_or(false) {
        let _ = window.hide();
        return;
    }

    let _ = position_popover(app);
    let _ = window.show();
    let _ = window.set_focus();
}

fn draw_pixel(buffer: &mut [u8], width: usize, x: i32, y: i32, color: [u8; 4]) {
    if x < 0 || y < 0 || x >= width as i32 || y >= width as i32 {
        return;
    }
    let index = (y as usize * width + x as usize) * 4;
    buffer[index..index + 4].copy_from_slice(&color);
}

const GLYPHS_3X5: [[u8; 15]; 10] = [
    [1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1],
    [0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1],
    [1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1],
    [1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1],
    [1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1],
    [1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1],
    [1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1],
    [1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0],
    [1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1],
    [1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1],
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrayIconSize {
    Px16,
    Px20,
    Px24,
    Px32,
}

impl TrayIconSize {
    fn pixels(self) -> usize {
        match self {
            Self::Px16 => 16,
            Self::Px20 => 20,
            Self::Px24 => 24,
            Self::Px32 => 32,
        }
    }

    fn for_scale_factor(scale_factor: f64) -> Self {
        if scale_factor <= 1.0 {
            Self::Px16
        } else if scale_factor <= 1.25 {
            Self::Px20
        } else if scale_factor <= 1.5 {
            Self::Px24
        } else {
            Self::Px32
        }
    }
}

struct TrayRaster {
    rgba: Vec<u8>,
    size: usize,
}

fn set_mask_pixel(mask: &mut [bool], size: usize, x: i32, y: i32) {
    if x >= 0 && y >= 0 && x < size as i32 && y < size as i32 {
        mask[y as usize * size + x as usize] = true;
    }
}

fn stamp_digit(
    mask: &mut [bool],
    size: usize,
    digit: u8,
    origin_x: i32,
    origin_y: i32,
    scale_x: i32,
    scale_y: i32,
) {
    for row in 0..5 {
        for column in 0..3 {
            if GLYPHS_3X5[digit as usize][row * 3 + column] == 0 {
                continue;
            }
            for offset_y in 0..scale_y {
                for offset_x in 0..scale_x {
                    set_mask_pixel(
                        mask,
                        size,
                        origin_x + column as i32 * scale_x + offset_x,
                        origin_y + row as i32 * scale_y + offset_y,
                    );
                }
            }
        }
    }
}

fn paint_number(buffer: &mut [u8], mask: &[bool], size: usize) {
    let outline = [0, 0, 0, 245];
    let white = [255, 255, 255, 255];
    for (index, is_number) in mask.iter().enumerate() {
        if !is_number {
            continue;
        }
        let x = (index % size) as i32;
        let y = (index / size) as i32;
        for offset_y in -1..=1 {
            for offset_x in -1..=1 {
                draw_pixel(buffer, size, x + offset_x, y + offset_y, outline);
            }
        }
    }
    for (index, is_number) in mask.iter().enumerate() {
        if *is_number {
            draw_pixel(
                buffer,
                size,
                (index % size) as i32,
                (index / size) as i32,
                white,
            );
        }
    }
}

fn stamp_single_tray_value(mask: &mut [bool], size: usize, value: u8) {
    let digits: Vec<u8> = if value == 100 {
        vec![1, 0, 0]
    } else if value >= 10 {
        vec![value / 10, value % 10]
    } else {
        vec![value]
    };
    let (scale_x, scale_y) = match (size, digits.len()) {
        (16, 1) => (3, 3),
        (16, 2) => (2, 2),
        (16, 3) => (1, 2),
        (20, 1) => (3, 3),
        (20, 2) => (2, 3),
        (20, 3) => (1, 3),
        (24, 1) => (4, 4),
        (24, 2) => (3, 3),
        (24, 3) => (2, 3),
        (32, 1) => (5, 5),
        (32, 2) => (4, 4),
        (32, 3) => (3, 4),
        _ => (1, 1),
    };
    let gap = 1;
    let total_width = digits.len() as i32 * 3 * scale_x + (digits.len() as i32 - 1) * gap;
    let total_height = 5 * scale_y;
    let origin_x = (size as i32 - total_width) / 2;
    let origin_y = (size as i32 - total_height) / 2;
    for (index, digit) in digits.into_iter().enumerate() {
        stamp_digit(
            mask,
            size,
            digit,
            origin_x + index as i32 * (3 * scale_x + gap),
            origin_y,
            scale_x,
            scale_y,
        );
    }
}

fn render_tray_raster(value: u8, icon_size: TrayIconSize) -> TrayRaster {
    let size = icon_size.pixels();
    let mut rgba = vec![0_u8; size * size * 4];
    let mut number_mask = vec![false; size * size];
    stamp_single_tray_value(&mut number_mask, size, value.min(100));
    paint_number(&mut rgba, &number_mask, size);
    TrayRaster { rgba, size }
}

fn render_tray_icon(value: u8, icon_size: TrayIconSize) -> Image<'static> {
    let raster = render_tray_raster(value, icon_size);
    Image::new_owned(raster.rgba, raster.size as u32, raster.size as u32)
}

fn render_unavailable_tray_icon(icon_size: TrayIconSize) -> Image<'static> {
    let size = icon_size.pixels();
    let mut rgba = vec![0_u8; size * size * 4];
    let mut mask = vec![false; size * size];
    let scale = match size {
        16 | 20 => 1,
        24 => 2,
        32 => 3,
        _ => 1,
    };
    let dash_width = 4 * scale;
    let gap = scale + 1;
    let total_width = dash_width * 2 + gap;
    let origin_x = (size as i32 - total_width) / 2;
    let origin_y = size as i32 / 2 - scale / 2;
    for dash in 0..2 {
        for y in 0..scale {
            for x in 0..dash_width {
                set_mask_pixel(
                    &mut mask,
                    size,
                    origin_x + dash * (dash_width + gap) + x,
                    origin_y + y,
                );
            }
        }
    }
    paint_number(&mut rgba, &mask, size);
    Image::new_owned(rgba, size as u32, size as u32)
}

fn tray_icon_size(app: &AppHandle) -> TrayIconSize {
    app.primary_monitor()
        .ok()
        .flatten()
        .map(|monitor| TrayIconSize::for_scale_factor(monitor.scale_factor()))
        .unwrap_or(TrayIconSize::Px16)
}

fn tray_values(status: &UsageStatus) -> Option<(u8, u8)> {
    let data = status.data()?;
    Some((
        data.weekly.as_ref()?.remaining_percent,
        data.five_hours.as_ref()?.remaining_percent,
    ))
}

fn tray_value(data: &CodexUsage, mode: TrayMode) -> Option<u8> {
    match mode {
        TrayMode::Auto => Some(data.limiting_window.remaining_percent),
        TrayMode::FiveHours => data
            .five_hours
            .as_ref()
            .map(|window| window.remaining_percent),
        TrayMode::Weekly => data.weekly.as_ref().map(|window| window.remaining_percent),
    }
}

fn update_tray_usage(app: &AppHandle, status: &UsageStatus) {
    let Some(tray) = app.tray_by_id("meroa-tray") else {
        return;
    };
    let size = tray_icon_size(app);
    if let (Some(data), Some((weekly, five_hours))) = (status.data(), tray_values(status)) {
        let language = app
            .state::<UsageStore>()
            .tray_language
            .lock()
            .map(|value| *value)
            .unwrap_or(TrayLanguage::English);
        let mode = app
            .state::<UsageStore>()
            .tray_mode
            .lock()
            .map(|value| *value)
            .unwrap_or(TrayMode::Auto);
        let Some(value) = tray_value(data, mode) else {
            return;
        };
        let tooltip = match language {
            TrayLanguage::Italian => format!("MEROA\n5 ore: {five_hours}%\nSettimanale: {weekly}%"),
            TrayLanguage::English => format!("MEROA\n5 hours: {five_hours}%\nWeekly: {weekly}%"),
        };
        let _ = tray.set_icon(Some(render_tray_icon(value, size)));
        let _ = tray.set_tooltip(Some(tooltip));
    } else {
        let _ = tray.set_icon(Some(render_unavailable_tray_icon(size)));
        let _ = tray.set_tooltip(Some("MEROA — Codex usage unavailable"));
    }
}

fn publish_usage_status(app: &AppHandle, status: UsageStatus) {
    if let Ok(mut current) = app.state::<UsageStore>().current.lock() {
        *current = status.clone();
    }
    update_tray_usage(app, &status);
    let _ = app.emit("usage-updated", status);
}

fn snapshot_is_stale(fetched_at: i64, now: SystemTime) -> bool {
    let age = now
        .duration_since(UNIX_EPOCH + Duration::from_secs(fetched_at.max(0) as u64))
        .unwrap_or_default();
    age >= STALE_AFTER
}

fn reset_auto_refresh(store: &UsageStore) {
    if let Ok(mut deadline) = store.next_refresh_at.lock() {
        *deadline = Instant::now() + AUTO_REFRESH_INTERVAL;
    }
}

fn auto_refresh_due(store: &UsageStore, now: Instant) -> bool {
    store
        .next_refresh_at
        .lock()
        .map(|deadline| now >= *deadline)
        .unwrap_or(false)
}

fn refresh_usage_now(app: &AppHandle) -> UsageStatus {
    let store = app.state::<UsageStore>();
    if store.refresh_in_progress.swap(true, Ordering::AcqRel) {
        return store
            .current
            .lock()
            .map(|state| state.clone())
            .unwrap_or_else(|_| UsageStatus::Error {
                message: "MEROA usage state is unavailable.".into(),
            });
    }

    let previous = store
        .current
        .lock()
        .map(|state| state.clone())
        .unwrap_or(UsageStatus::Loading);
    let status = match codex_usage::fetch_usage() {
        Ok(data) => UsageStatus::Available { data },
        Err(error) => {
            if let Some(data) = previous.data().cloned() {
                if snapshot_is_stale(data.fetched_at, SystemTime::now()) {
                    UsageStatus::Stale {
                        data,
                        message: error.message().to_string(),
                    }
                } else {
                    // Keep the last valid snapshot live until it actually becomes stale.
                    UsageStatus::Available { data }
                }
            } else {
                match error {
                    FetchError::Unavailable(message) => UsageStatus::Unavailable { message },
                    FetchError::Source(message) => UsageStatus::Error { message },
                }
            }
        }
    };
    store.refresh_in_progress.store(false, Ordering::Release);
    reset_auto_refresh(&store);
    publish_usage_status(app, status.clone());
    status
}

fn start_usage_refresh(app: AppHandle) {
    thread::spawn(move || {
        // Startup always loads a real snapshot immediately; subsequent calls share
        // the same guarded refresh path as manual refreshes.
        refresh_usage_now(&app);
        loop {
            thread::sleep(Duration::from_secs(1));
            if auto_refresh_due(&app.state::<UsageStore>(), Instant::now()) {
                refresh_usage_now(&app);
            }
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(PopoverState::default())
        .manage(UsageStore::default())
        .invoke_handler(tauri::generate_handler![
            hide_popover,
            sync_popover_geometry,
            get_usage_state,
            refresh_usage,
            set_tray_language,
            set_tray_mode
        ])
        .setup(|app| {
            let tray_icon_size = tray_icon_size(app.handle());
            let open = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
            let refresh = MenuItem::with_id(app, "refresh", "Refresh", true, None::<&str>)?;
            let startup = CheckMenuItem::with_id(
                app,
                "startup",
                "Start with Windows",
                true,
                false,
                None::<&str>,
            )?;
            let settings = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
            let separator = PredefinedMenuItem::separator(app)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(
                app,
                &[&open, &refresh, &startup, &settings, &separator, &quit],
            )?;

            TrayIconBuilder::with_id("meroa-tray")
                .tooltip("MEROA — Loading Codex usage")
                .icon(render_unavailable_tray_icon(tray_icon_size))
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        position,
                        rect,
                        ..
                    } = event
                    {
                        update_tray_anchor(tray.app_handle(), position.x, position.y, rect);
                        toggle_popover(tray.app_handle());
                    } else if let TrayIconEvent::Click { position, rect, .. }
                    | TrayIconEvent::Enter { position, rect, .. }
                    | TrayIconEvent::Move { position, rect, .. } = event
                    {
                        update_tray_anchor(tray.app_handle(), position.x, position.y, rect);
                    }
                })
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "open" => toggle_popover(app),
                    "refresh" => {
                        let app = app.clone();
                        thread::spawn(move || {
                            refresh_usage_now(&app);
                        });
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;

            if let Some(window) = app.get_webview_window(POPOVER_LABEL) {
                let window_for_events = window.clone();
                window.on_window_event(move |event| match event {
                    WindowEvent::Focused(false) => {
                        let _ = window_for_events.hide();
                    }
                    WindowEvent::CloseRequested { api, .. } => {
                        api.prevent_close();
                        let _ = window_for_events.hide();
                    }
                    _ => {}
                });
            }
            start_usage_refresh(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running MEROA");
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::PathBuf,
        time::{Duration, Instant},
    };

    use super::{
        auto_refresh_due, calculate_popover_layout,
        codex_usage::{CodexUsage, LimitingWindow, WindowKind},
        render_tray_raster, snapshot_is_stale, tray_value, tray_values, TrayAnchor, TrayIconSize,
        TrayMode, UsageStatus, UsageStore, WorkArea,
    };

    #[test]
    fn tray_renderer_covers_values_and_native_sizes() {
        for size in [
            TrayIconSize::Px16,
            TrayIconSize::Px20,
            TrayIconSize::Px24,
            TrayIconSize::Px32,
        ] {
            for value in [0, 2, 35, 74, 87, 100] {
                let raster = render_tray_raster(value, size);
                assert_eq!(raster.rgba.len(), size.pixels() * size.pixels() * 4);
                let (pixels, remainder) = raster.rgba.as_chunks::<4>();
                assert!(remainder.is_empty());
                assert!(pixels.iter().any(|pixel| pixel[3] == 255));
                assert!(pixels.iter().any(|pixel| pixel[3] == 0));
            }
        }
    }

    #[test]
    fn tray_size_tracks_windows_dpi_scale() {
        assert_eq!(TrayIconSize::for_scale_factor(1.0), TrayIconSize::Px16);
        assert_eq!(TrayIconSize::for_scale_factor(1.25), TrayIconSize::Px20);
        assert_eq!(TrayIconSize::for_scale_factor(1.5), TrayIconSize::Px24);
        assert_eq!(TrayIconSize::for_scale_factor(2.0), TrayIconSize::Px32);
    }

    #[test]
    fn tray_values_keep_weekly_first_and_five_hours_second() {
        let status = UsageStatus::Available {
            data: CodexUsage {
                five_hours: Some(super::codex_usage::UsageWindow {
                    kind: WindowKind::FiveHours,
                    used_percent: 13.0,
                    remaining_percent: 87,
                    resets_at: None,
                }),
                weekly: Some(super::codex_usage::UsageWindow {
                    kind: WindowKind::Weekly,
                    used_percent: 65.0,
                    remaining_percent: 35,
                    resets_at: None,
                }),
                credits: None,
                limiting_window: LimitingWindow {
                    kind: WindowKind::Weekly,
                    remaining_percent: 23,
                    resets_at: None,
                },
                fetched_at: 123,
            },
        };
        assert_eq!(tray_values(&status), Some((35, 87)));
        let data = status.data().unwrap();
        assert_eq!(tray_value(data, TrayMode::Auto), Some(23));
        assert_eq!(tray_value(data, TrayMode::FiveHours), Some(87));
        assert_eq!(tray_value(data, TrayMode::Weekly), Some(35));
        assert_eq!(tray_values(&UsageStatus::Loading), None);
    }

    #[test]
    fn stale_state_starts_only_after_ten_minutes_without_a_valid_snapshot() {
        let fetched_at = 1_000;
        assert!(!snapshot_is_stale(
            fetched_at,
            std::time::UNIX_EPOCH + Duration::from_secs(1_599)
        ));
        assert!(snapshot_is_stale(
            fetched_at,
            std::time::UNIX_EPOCH + Duration::from_secs(1_600)
        ));
    }

    #[test]
    fn automatic_refresh_is_due_only_at_its_scheduled_deadline() {
        let store = UsageStore::default();
        assert!(auto_refresh_due(&store, Instant::now()));
        super::reset_auto_refresh(&store);
        assert!(!auto_refresh_due(&store, Instant::now()));
    }

    #[test]
    #[ignore = "manual visual QA artifact"]
    fn export_tray_preview_matrix() {
        const SCALE: usize = 3;
        const CELL_ICON_SIZE: usize = 32 * SCALE;
        const CELL_GAP: usize = 16;
        const CELL_WIDTH: usize = CELL_ICON_SIZE * 2 + CELL_GAP;
        const CELL_HEIGHT: usize = CELL_ICON_SIZE + CELL_GAP;
        let values = [0, 2, 35, 74, 87, 100];
        let sizes = [
            TrayIconSize::Px16,
            TrayIconSize::Px20,
            TrayIconSize::Px24,
            TrayIconSize::Px32,
        ];
        let width = CELL_WIDTH * values.len();
        let height = CELL_HEIGHT * sizes.len();
        let mut rgb = vec![28_u8; width * height * 3];

        for (row, size) in sizes.into_iter().enumerate() {
            for (column, value) in values.into_iter().enumerate() {
                let raster = render_tray_raster(value, size);
                let icon_extent = raster.size * SCALE;
                let top = row * CELL_HEIGHT + (CELL_ICON_SIZE - icon_extent) / 2;
                let left = column * CELL_WIDTH + (CELL_ICON_SIZE - icon_extent) / 2;
                for light_background in [false, true] {
                    let panel_left = left
                        + if light_background {
                            CELL_ICON_SIZE + CELL_GAP
                        } else {
                            0
                        };
                    let background = if light_background { 242_u8 } else { 28_u8 };
                    for y in 0..icon_extent {
                        for x in 0..icon_extent {
                            let source_x = x / SCALE;
                            let source_y = y / SCALE;
                            let source = (source_y * raster.size + source_x) * 4;
                            let alpha = raster.rgba[source + 3] as u16;
                            let destination = ((top + y) * width + panel_left + x) * 3;
                            for channel in 0..3 {
                                rgb[destination + channel] =
                                    ((raster.rgba[source + channel] as u16 * alpha
                                        + background as u16 * (255 - alpha))
                                        / 255) as u8;
                            }
                        }
                    }
                }
            }
        }

        let output = std::env::var_os("MEROA_TRAY_PREVIEW")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("target/tray-preview.bmp"));
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).expect("create tray preview directory");
        }
        let row_stride = (width * 3 + 3) & !3;
        let pixel_bytes = row_stride * height;
        let mut bmp = Vec::with_capacity(54 + pixel_bytes);
        bmp.extend(b"BM");
        bmp.extend(((54 + pixel_bytes) as u32).to_le_bytes());
        bmp.extend([0_u8; 4]);
        bmp.extend(54_u32.to_le_bytes());
        bmp.extend(40_u32.to_le_bytes());
        bmp.extend((width as i32).to_le_bytes());
        bmp.extend((height as i32).to_le_bytes());
        bmp.extend(1_u16.to_le_bytes());
        bmp.extend(24_u16.to_le_bytes());
        bmp.extend([0_u8; 24]);
        for row in (0..height).rev() {
            let start = row * width * 3;
            let (pixels, remainder) = rgb[start..start + width * 3].as_chunks::<3>();
            assert!(remainder.is_empty());
            for pixel in pixels {
                bmp.extend([pixel[2], pixel[1], pixel[0]]);
            }
            bmp.extend(std::iter::repeat_n(0, row_stride - width * 3));
        }
        fs::write(&output, bmp).expect("write tray preview");
        println!("{}", output.display());
    }

    #[test]
    fn popover_layout_tracks_anchor_across_supported_dpi_scales() {
        for scale in [1.0, 1.25, 1.5, 2.0] {
            let width = 388.0 * scale;
            let height = 592.0 * scale;
            let anchor = TrayAnchor {
                center_x: 1800.0 * scale,
                top_y: 1040.0 * scale,
            };
            let layout = calculate_popover_layout(
                anchor,
                width,
                height,
                WorkArea {
                    left: 0.0,
                    top: 0.0,
                    width: 1920.0 * scale,
                    height: 1040.0 * scale,
                },
                scale,
            );
            assert!((layout.pointer_x - 276.0).abs() < 0.01);
            assert!(layout.x + width <= 1912.0 * scale + 0.01);
            assert!(layout.y >= 8.0 * scale);
        }
    }
}
