use tauri::{Manager, WindowEvent};

mod clipboard;
mod commands;
mod events;
mod history;
mod image;
mod settings;
mod storage;
mod windows;

pub fn run() {
    tauri::Builder::default()
        .manage(clipboard::ClipboardRecorderStore::default())
        .manage(history::ClipboardHistoryStore::default())
        .manage(std::sync::Mutex::new(
            clipboard::ClipboardRecorder::default(),
        ))
        .manage(settings::PetSettingsStore::default())
        .invoke_handler(tauri::generate_handler![
            commands::get_pet_settings,
            commands::move_pet_window_by,
            commands::save_pet_position,
            commands::update_pet_behavior_settings,
            commands::get_clipboard_recording_settings,
            commands::update_clipboard_recording_settings,
            commands::open_history_panel,
            commands::close_history_panel,
            commands::open_settings_window,
            commands::set_recording_paused,
            commands::suppress_next_clipboard_hash,
            commands::list_clipboard_history,
            commands::get_clipboard_history_item,
            commands::restore_clipboard_history_item,
            commands::delete_clipboard_history_item,
            commands::clear_clipboard_history,
            commands::update_clipboard_history_settings,
            commands::debug_process_clipboard_text,
            commands::show_pet_context_menu,
            commands::quit_app
        ])
        .setup(|app| {
            app.state::<history::ClipboardHistoryStore>()
                .initialize(storage::app_database_path(app.handle()))?;
            windows::configure_pet_window(app.handle())?;
            clipboard::start_clipboard_polling_listener(app.handle().clone());
            if std::env::args().any(|arg| arg == "--e2e-open-history") {
                windows::open_history_panel(app.handle())?;
            }
            if std::env::args().any(|arg| arg == "--e2e-move-pet-after-open-history") {
                windows::move_pet_window_by(app.handle(), 60, 35)?;
            }
            if std::env::args().any(|arg| arg == "--e2e-close-history-after-open") {
                let handle = app.handle().clone();
                std::thread::spawn(move || {
                    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(12);
                    while std::time::Instant::now() < deadline {
                        if handle.get_webview_window("history").is_some() {
                            std::thread::sleep(std::time::Duration::from_millis(800));
                            break;
                        }
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }
                    let _ = windows::close_history_panel(&handle);
                });
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() == "history" {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = window.destroy();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("failed to run Cute Clipboard Agent");
}
