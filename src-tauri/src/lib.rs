use tauri::{
    image::Image,
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, PhysicalPosition, WindowEvent,
};

const POPOVER_LABEL: &str = "main";

#[tauri::command]
fn hide_popover(app: AppHandle) -> Result<(), String> {
    app.get_webview_window(POPOVER_LABEL)
        .ok_or_else(|| "popover window unavailable".to_string())?
        .hide()
        .map_err(|error| error.to_string())
}

fn toggle_popover(app: &AppHandle, tray_x: f64, tray_y: f64) {
    let Some(window) = app.get_webview_window(POPOVER_LABEL) else {
        return;
    };

    if window.is_visible().unwrap_or(false) {
        let _ = window.hide();
        return;
    }

    if let Ok(size) = window.outer_size() {
        let x = (tray_x - size.width as f64 + 24.0).max(0.0);
        let y = (tray_y - size.height as f64 - 12.0).max(0.0);
        let _ = window.set_position(PhysicalPosition::new(x, y));
    }

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
                draw_pixel(
                    buffer,
                    width,
                    origin_x + column as i32,
                    origin_y + row as i32,
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
        draw_digit(&mut rgba, SIZE, 1, 10, 13);
        draw_digit(&mut rgba, SIZE, 0, 15, 13);
        draw_digit(&mut rgba, SIZE, 0, 20, 13);
    } else if display_value >= 10 {
        draw_digit(&mut rgba, SIZE, tens, 12, 13);
        draw_digit(&mut rgba, SIZE, ones, 17, 13);
    } else {
        draw_digit(&mut rgba, SIZE, ones, 15, 13);
    }
    Image::new_owned(rgba, SIZE as u32, SIZE as u32)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hide_popover])
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
                        ..
                    } = event
                    {
                        toggle_popover(tray.app_handle(), position.x, position.y);
                    }
                })
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "open" => toggle_popover(app, 0.0, 0.0),
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
    use super::render_tray_icon;

    #[test]
    fn tray_renderer_accepts_edge_percentages() {
        let _ = render_tray_icon(0);
        let _ = render_tray_icon(9);
        let _ = render_tray_icon(74);
        let _ = render_tray_icon(100);
    }
}
