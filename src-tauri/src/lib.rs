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

fn draw_digit(buffer: &mut [u8], width: usize, digit: u8, origin_x: i32, origin_y: i32) {
    const GLYPHS: [[u8; 15]; 10] = [
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
    let white = [242, 242, 242, 255];
    for row in 0..5 {
        for column in 0..3 {
            if GLYPHS[digit as usize][row * 3 + column] == 1 {
                draw_dot(
                    buffer,
                    width,
                    origin_x + column as i32 * 2,
                    origin_y + row as i32 * 2,
                    white,
                );
            }
        }
    }
}

fn render_tray_icon(value: u8) -> Image<'static> {
    const SIZE: usize = 32;
    const DOTS: usize = 24;
    let mut rgba = vec![0_u8; SIZE * SIZE * 4];
    let active = (value as usize * DOTS + 50) / 100;
    for index in 0..DOTS {
        let angle =
            index as f64 / DOTS as f64 * std::f64::consts::TAU - std::f64::consts::FRAC_PI_2;
        let x = (15.0 + angle.cos() * 13.0).round() as i32;
        let y = (15.0 + angle.sin() * 13.0).round() as i32;
        let color = if index < active {
            if index + 3 >= active {
                [89, 214, 111, 255]
            } else {
                [242, 242, 242, 255]
            }
        } else {
            [92, 92, 92, 220]
        };
        draw_dot(&mut rgba, SIZE, x, y, color);
    }

    let display_value = value.min(100);
    let tens = display_value / 10;
    let ones = display_value % 10;
    if display_value == 100 {
        draw_digit(&mut rgba, SIZE, 1, 6, 11);
        draw_digit(&mut rgba, SIZE, 0, 13, 11);
        draw_digit(&mut rgba, SIZE, 0, 20, 11);
    } else if display_value >= 10 {
        draw_digit(&mut rgba, SIZE, tens, 9, 11);
        draw_digit(&mut rgba, SIZE, ones, 17, 11);
    } else {
        draw_digit(&mut rgba, SIZE, ones, 13, 11);
    }
    Image::new_owned(rgba, SIZE as u32, SIZE as u32)
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
                .icon(render_tray_icon(74))
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
    use super::{calculate_popover_layout, render_tray_icon, TrayAnchor, WorkArea};

    #[test]
    fn tray_renderer_accepts_edge_percentages() {
        let _ = render_tray_icon(0);
        let _ = render_tray_icon(9);
        let _ = render_tray_icon(74);
        let _ = render_tray_icon(100);
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
