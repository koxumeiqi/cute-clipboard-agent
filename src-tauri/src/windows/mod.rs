use tauri::{AppHandle, Manager, PhysicalPosition, WebviewUrl, WebviewWindowBuilder};
#[cfg(windows)]
use windows_sys::Win32::{
    Foundation::{POINT, RECT},
    UI::{
        Input::KeyboardAndMouse::{GetAsyncKeyState, VK_LBUTTON},
        WindowsAndMessaging::{GetCursorPos, GetWindowRect},
    },
};

const HISTORY_WIDTH: f64 = 420.0;
const HISTORY_HEIGHT: f64 = 560.0;
const HISTORY_GAP: i32 = 10;

pub fn configure_pet_window(app: &AppHandle) -> tauri::Result<()> {
    if let Some(window) = app.get_webview_window("pet") {
        window.set_decorations(false)?;
        window.set_always_on_top(true)?;
        window.set_resizable(false)?;
        window.set_skip_taskbar(true)?;
        window.set_focusable(true)?;
        window.show()?;
        start_pet_drag_monitor(app.clone(), window);
    }
    Ok(())
}

pub fn open_history_panel(app: &AppHandle) -> tauri::Result<()> {
    let initial_position = history_position_next_to_pet(app)?;
    let window = open_or_focus_window(
        app,
        "history",
        "Clipboard History",
        "history.html",
        HISTORY_WIDTH,
        HISTORY_HEIGHT,
        initial_position,
    )?;
    place_history_next_to_pet(app, &window)?;
    if let Some(pet) = app.get_webview_window("pet") {
        pet.show()?;
        pet.set_always_on_top(true)?;
        pet.set_focus()?;
    } else {
        window.set_focus()?;
    }
    Ok(())
}

pub fn open_settings_window(app: &AppHandle) -> tauri::Result<()> {
    open_or_focus_window(
        app,
        "settings",
        "Settings",
        "index.html",
        520.0,
        460.0,
        None,
    )
    .map(|_| ())
}

pub fn close_history_panel(app: &AppHandle) -> tauri::Result<()> {
    if let Some(window) = app.get_webview_window("history") {
        window.destroy()?;
    }
    if let Some(window) = app.get_webview_window("pet") {
        window.set_always_on_top(true)?;
        window.set_focus()?;
    }
    Ok(())
}

pub fn move_pet_window_by(app: &AppHandle, delta_x: i32, delta_y: i32) -> tauri::Result<()> {
    if let Some(window) = app.get_webview_window("pet") {
        let position = window.outer_position()?;
        window.set_position(PhysicalPosition::new(
            position.x + delta_x,
            position.y + delta_y,
        ))?;
    }
    Ok(())
}

#[cfg(windows)]
fn start_pet_drag_monitor(app: AppHandle, window: tauri::WebviewWindow) {
    std::thread::spawn(move || {
        const DRAG_THRESHOLD_PX: i32 = 4;
        let mut active_drag: Option<(POINT, POINT)> = None;
        let mut was_left_button_down = false;

        loop {
            if app.get_webview_window("pet").is_none() {
                break;
            }

            let left_button_down =
                unsafe { (GetAsyncKeyState(VK_LBUTTON as i32) & 0x8000u16 as i16) != 0 };
            let mut cursor = POINT { x: 0, y: 0 };
            let has_cursor = unsafe { GetCursorPos(&mut cursor) != 0 };

            if !left_button_down || !has_cursor {
                active_drag = None;
                was_left_button_down = left_button_down;
                std::thread::sleep(std::time::Duration::from_millis(12));
                continue;
            }

            if !was_left_button_down && point_inside_window(&window, cursor) {
                active_drag = Some((cursor, cursor));
            }

            if let Some((start, last)) = active_drag.as_mut() {
                let total_delta_x = cursor.x - start.x;
                let total_delta_y = cursor.y - start.y;
                let distance_squared =
                    total_delta_x * total_delta_x + total_delta_y * total_delta_y;
                if distance_squared >= DRAG_THRESHOLD_PX * DRAG_THRESHOLD_PX {
                    let delta_x = cursor.x - last.x;
                    let delta_y = cursor.y - last.y;
                    if delta_x != 0 || delta_y != 0 {
                        if let Ok(position) = window.outer_position() {
                            let _ = window.set_position(PhysicalPosition::new(
                                position.x + delta_x,
                                position.y + delta_y,
                            ));
                        }
                        *last = cursor;
                    }
                }
            }

            was_left_button_down = left_button_down;
            std::thread::sleep(std::time::Duration::from_millis(12));
        }
    });
}

#[cfg(windows)]
fn point_inside_window(window: &tauri::WebviewWindow, point: POINT) -> bool {
    let hwnd = match window.hwnd() {
        Ok(handle) => handle,
        Err(_) => return false,
    };
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    unsafe {
        if GetWindowRect(hwnd.0, &mut rect) == 0 {
            return false;
        }
    }
    point.x >= rect.left && point.x <= rect.right && point.y >= rect.top && point.y <= rect.bottom
}

#[cfg(not(windows))]
fn start_pet_drag_monitor(_app: AppHandle, _window: tauri::WebviewWindow) {}

fn open_or_focus_window(
    app: &AppHandle,
    label: &str,
    title: &str,
    url: &str,
    width: f64,
    height: f64,
    initial_position: Option<PhysicalPosition<i32>>,
) -> tauri::Result<tauri::WebviewWindow> {
    if let Some(window) = app.get_webview_window(label) {
        window.show()?;
        window.set_focus()?;
        return Ok(window);
    }

    let mut builder = WebviewWindowBuilder::new(app, label, WebviewUrl::App(url.into()))
        .title(title)
        .inner_size(width, height)
        .resizable(true)
        .decorations(true)
        .focused(true);

    if let Some(position) = initial_position {
        builder = builder.position(position.x as f64, position.y as f64);
    }

    builder.build()
}

fn place_history_next_to_pet(app: &AppHandle, history: &tauri::WebviewWindow) -> tauri::Result<()> {
    if let Some(position) = history_position_next_to_pet(app)? {
        history.set_position(position)?;
    }
    if let Some(pet) = app.get_webview_window("pet") {
        pet.set_always_on_top(true)?;
    }
    Ok(())
}

fn history_position_next_to_pet(app: &AppHandle) -> tauri::Result<Option<PhysicalPosition<i32>>> {
    if let Some(pet) = app.get_webview_window("pet") {
        let pet_position = pet.outer_position()?;
        let pet_size = pet.outer_size()?;
        let mut history_x = pet_position.x + pet_size.width as i32 + HISTORY_GAP;
        if let Some(monitor) = pet.current_monitor()? {
            let monitor_position = monitor.position();
            let monitor_size = monitor.size();
            let monitor_right = monitor_position.x + monitor_size.width as i32;
            if history_x + HISTORY_WIDTH as i32 > monitor_right {
                history_x =
                    (pet_position.x - HISTORY_WIDTH as i32 - HISTORY_GAP).max(monitor_position.x);
            }
        }
        return Ok(Some(PhysicalPosition::new(history_x, pet_position.y)));
    }
    Ok(None)
}
