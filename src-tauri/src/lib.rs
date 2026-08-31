use std::sync::Mutex;
use tauri::{
    image::Image,
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, LogicalSize, Manager, PhysicalPosition, Rect, WindowEvent,
};

const POPOVER_LABEL: &str = "main";

#[derive(Clone, Copy, Debug)]
struct TrayAnchor {
    center_x: f64,
    top_y: f64,
}

#[derive(Default)]
struct PopoverState {
    anchor: Mutex<Option<TrayAnchor>>,
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

fn draw_dot(buffer: &mut [u8], width: usize, x: i32, y: i32, color: [u8; 4]) {
    draw_pixel(buffer, width, x, y, color);
    draw_pixel(buffer, width, x + 1, y, color);
    draw_pixel(buffer, width, x, y + 1, color);
    draw_pixel(buffer, width, x + 1, y + 1, color);
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

fn stamp_standard_value(mask: &mut [bool], size: usize, value: u8) {
    let digits: Vec<u8> = if value >= 10 {
        vec![value / 10, value % 10]
    } else {
        vec![value]
    };
    let scale = match (size, digits.len()) {
        (16, 1) => 3,
        (16 | 20, 2) => 2,
        (20, 1) => 3,
        (24, 1) => 4,
        (24, 2) => 3,
        (32, 1) => 5,
        (32, 2) => 4,
        _ => 2,
    };
    let glyph_width = 3 * scale;
    let gap = 1;
    let total_width = glyph_width * digits.len() as i32 + gap * (digits.len() as i32 - 1);
    let total_height = 5 * scale;
    let origin_x = (size as i32 - total_width) / 2;
    let origin_y = (size as i32 - total_height) / 2;
    for (index, digit) in digits.into_iter().enumerate() {
        stamp_digit(
            mask,
            size,
            digit,
            origin_x + index as i32 * (glyph_width + gap),
            origin_y,
            scale,
            scale,
        );
    }
}

fn stamp_hundred(mask: &mut [bool], size: usize) {
    match size {
        16 => {
            // Dedicated micro-layout: a dominant 1 followed by two stacked zeroes.
            stamp_digit(mask, size, 1, 1, 3, 2, 2);
            stamp_digit(mask, size, 0, 11, 1, 1, 1);
            stamp_digit(mask, size, 0, 11, 10, 1, 1);
        }
        20 | 24 => {
            // 5x7-inspired 100: narrow horizontal pixels, doubled vertically.
            const ROWS: [&str; 7] = [
                "01110 01110 01110",
                "00110 10001 10001",
                "00110 10001 10001",
                "00110 10001 10001",
                "00110 10001 10001",
                "00110 10001 10001",
                "11111 01110 01110",
            ];
            let origin_x = (size as i32 - 17) / 2;
            let origin_y = (size as i32 - 14) / 2;
            for (row, pattern) in ROWS.iter().enumerate() {
                for (column, bit) in pattern.bytes().filter(|bit| *bit != b' ').enumerate() {
                    if bit == b'1' {
                        set_mask_pixel(
                            mask,
                            size,
                            origin_x + column as i32,
                            origin_y + row as i32 * 2,
                        );
                        set_mask_pixel(
                            mask,
                            size,
                            origin_x + column as i32,
                            origin_y + row as i32 * 2 + 1,
                        );
                    }
                }
            }
        }
        32 => {
            for (index, digit) in [1_u8, 0, 0].into_iter().enumerate() {
                stamp_digit(mask, size, digit, 1 + index as i32 * 10, 8, 3, 3);
            }
        }
        _ => unreachable!("unsupported tray icon size"),
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

fn paint_ring(buffer: &mut [u8], size: usize, value: u8) {
    let dot_count = match size {
        16 => return,
        20 => 4,
        24 => 12,
        32 => 20,
        _ => unreachable!("unsupported tray icon size"),
    };
    let center = (size as f64 - 1.0) / 2.0;
    let radius = center - if size == 20 { 1.0 } else { 0.5 };
    let active = (value.min(100) as usize * dot_count + 50) / 100;
    let accent_count = if size >= 24 { 2 } else { 1 };
    for index in 0..dot_count {
        let angle =
            index as f64 / dot_count as f64 * std::f64::consts::TAU - std::f64::consts::FRAC_PI_2;
        let x = (center + angle.cos() * radius).round() as i32;
        let y = (center + angle.sin() * radius).round() as i32;
        let color = if index < active {
            if index + accent_count >= active {
                [89, 214, 111, 255]
            } else {
                [224, 224, 224, 255]
            }
        } else {
            [105, 105, 105, 235]
        };
        draw_pixel(buffer, size, x, y, [0, 0, 0, 230]);
        if size == 32 {
            draw_dot(buffer, size, x - 1, y - 1, color);
        } else {
            draw_pixel(buffer, size, x, y, color);
        }
    }
}

fn render_tray_raster(value: u8, icon_size: TrayIconSize) -> TrayRaster {
    let size = icon_size.pixels();
    let value = value.min(100);
    let mut rgba = vec![0_u8; size * size * 4];
    paint_ring(&mut rgba, size, value);
    let mut number_mask = vec![false; size * size];
    if value == 100 {
        stamp_hundred(&mut number_mask, size);
    } else {
        stamp_standard_value(&mut number_mask, size, value);
    }
    paint_number(&mut rgba, &number_mask, size);
    TrayRaster { rgba, size }
}

fn render_tray_icon(value: u8, icon_size: TrayIconSize) -> Image<'static> {
    let raster = render_tray_raster(value, icon_size);
    Image::new_owned(raster.rgba, raster.size as u32, raster.size as u32)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(PopoverState::default())
        .invoke_handler(tauri::generate_handler![
            hide_popover,
            sync_popover_geometry
        ])
        .setup(|app| {
            let tray_icon_size = app
                .primary_monitor()?
                .map(|monitor| TrayIconSize::for_scale_factor(monitor.scale_factor()))
                .unwrap_or(TrayIconSize::Px16);
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
                .tooltip("MEROA — 74% remaining")
                .icon(render_tray_icon(74, tray_icon_size))
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
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running MEROA");
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use super::{calculate_popover_layout, render_tray_raster, TrayAnchor, TrayIconSize, WorkArea};

    #[test]
    fn tray_renderer_covers_values_and_native_sizes() {
        for size in [
            TrayIconSize::Px16,
            TrayIconSize::Px20,
            TrayIconSize::Px24,
            TrayIconSize::Px32,
        ] {
            for value in [0, 9, 23, 74, 91, 100] {
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
    #[ignore = "manual visual QA artifact"]
    fn export_tray_preview_matrix() {
        const SCALE: usize = 3;
        const CELL_ICON_SIZE: usize = 32 * SCALE;
        const CELL_GAP: usize = 16;
        const CELL_WIDTH: usize = CELL_ICON_SIZE * 2 + CELL_GAP;
        const CELL_HEIGHT: usize = CELL_ICON_SIZE + CELL_GAP;
        let values = [0, 9, 23, 74, 91, 100];
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
