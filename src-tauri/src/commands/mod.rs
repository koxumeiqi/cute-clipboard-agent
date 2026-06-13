use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

use crate::{
    clipboard::{
        ClipboardProcessOutcome, ClipboardRawContent, ClipboardRecorder, ClipboardRecorderStore,
        ClipboardRecordingSettings, SuppressNextClipboardHashRequest,
        UpdateClipboardRecordingSettingsRequest,
    },
    events,
    history::{
        ClipboardHistoryItemRequest, ClipboardHistorySnapshot, ClipboardHistoryStore,
        UpdateClipboardHistorySettingsRequest,
    },
    settings::{
        AppPreferencesStore, AppSettings, PetPosition, PetSettings, PetSettingsStore,
        UpdateAppSettingsRequest, UpdatePetBehaviorSettingsRequest,
    },
    windows,
};

#[derive(Debug, serde::Deserialize)]
pub struct SetRecordingPausedRequest {
    pub paused: bool,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugProcessClipboardTextRequest {
    pub text: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovePetWindowByRequest {
    pub delta_x: i32,
    pub delta_y: i32,
}

#[tauri::command]
pub fn get_pet_settings(
    app: AppHandle,
    store: State<'_, PetSettingsStore>,
) -> Result<PetSettings, String> {
    store.load(&app).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn move_pet_window_by(
    app: AppHandle,
    request: MovePetWindowByRequest,
) -> Result<PetPosition, String> {
    let window = app
        .get_webview_window("pet")
        .ok_or_else(|| "window_operation_failed".to_string())?;
    let position = window
        .outer_position()
        .map_err(|_| "window_operation_failed".to_string())?;
    let next_position = PetPosition {
        x: f64::from(position.x + request.delta_x),
        y: f64::from(position.y + request.delta_y),
    };
    window
        .set_position(tauri::PhysicalPosition::new(
            next_position.x as i32,
            next_position.y as i32,
        ))
        .map_err(|_| "window_operation_failed".to_string())?;
    Ok(next_position)
}

#[tauri::command]
pub fn save_pet_position(
    app: AppHandle,
    store: State<'_, PetSettingsStore>,
    position: PetPosition,
) -> Result<PetSettings, String> {
    let settings = store
        .save_position(&app, position)
        .map_err(|error| error.to_string())?;
    events::emit_pet_drag_ended(&app, settings.position);
    Ok(settings)
}

#[tauri::command]
pub fn update_pet_behavior_settings(
    app: AppHandle,
    store: State<'_, PetSettingsStore>,
    input: UpdatePetBehaviorSettingsRequest,
) -> Result<PetSettings, String> {
    let settings = store
        .update_behavior(&app, input)
        .map_err(|error| error.to_string())?;
    if let Some(window) = app.get_webview_window("pet") {
        window
            .set_always_on_top(settings.always_on_top)
            .map_err(|_| "window_operation_failed".to_string())?;
    }
    Ok(settings)
}

#[tauri::command]
pub fn get_app_settings(
    app: AppHandle,
    pet_store: State<'_, PetSettingsStore>,
    clipboard_store: State<'_, ClipboardRecorderStore>,
    history_store: State<'_, ClipboardHistoryStore>,
    preferences_store: State<'_, AppPreferencesStore>,
) -> Result<AppSettings, String> {
    crate::settings::load_app_settings(
        &app,
        &pet_store,
        &clipboard_store,
        &history_store,
        &preferences_store,
    )
    .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn update_app_settings(
    app: AppHandle,
    pet_store: State<'_, PetSettingsStore>,
    clipboard_store: State<'_, ClipboardRecorderStore>,
    history_store: State<'_, ClipboardHistoryStore>,
    preferences_store: State<'_, AppPreferencesStore>,
    request: UpdateAppSettingsRequest,
) -> Result<AppSettings, String> {
    let previous = crate::settings::load_app_settings(
        &app,
        &pet_store,
        &clipboard_store,
        &history_store,
        &preferences_store,
    )
    .map_err(|error| error.to_string())?;
    let settings = crate::settings::update_app_settings(
        &app,
        &pet_store,
        &clipboard_store,
        &history_store,
        &preferences_store,
        request,
    )
    .map_err(|error| error.to_string())?;
    if previous.recording_paused != settings.recording_paused {
        if settings.recording_paused {
            events::emit_recording_paused(&app);
        } else {
            events::emit_recording_resumed(&app);
        }
    }
    events::emit_settings_updated(&app, settings.clone());
    Ok(settings)
}

#[tauri::command]
pub fn open_history_panel(app: AppHandle) -> Result<(), String> {
    events::emit_pet_double_clicked(&app);
    windows::open_history_panel(&app).map_err(|_| "window_operation_failed".to_string())
}

#[tauri::command]
pub fn close_history_panel(app: AppHandle) -> Result<(), String> {
    windows::close_history_panel(&app).map_err(|_| "window_operation_failed".to_string())
}

#[tauri::command]
pub fn open_settings_window(app: AppHandle) -> Result<(), String> {
    windows::open_settings_window(&app).map_err(|_| "window_operation_failed".to_string())
}

#[tauri::command]
pub fn get_clipboard_recording_settings(
    app: AppHandle,
    store: State<'_, ClipboardRecorderStore>,
) -> Result<ClipboardRecordingSettings, String> {
    store.load(&app).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn update_clipboard_recording_settings(
    app: AppHandle,
    store: State<'_, ClipboardRecorderStore>,
    input: UpdateClipboardRecordingSettingsRequest,
) -> Result<ClipboardRecordingSettings, String> {
    store.update(&app, input).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn set_recording_paused(
    app: AppHandle,
    store: State<'_, ClipboardRecorderStore>,
    request: SetRecordingPausedRequest,
) -> Result<ClipboardRecordingSettings, String> {
    store
        .set_paused(&app, request.paused)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn suppress_next_clipboard_hash(
    recorder: State<'_, Mutex<ClipboardRecorder>>,
    request: SuppressNextClipboardHashRequest,
) -> Result<(), String> {
    let mut recorder = recorder
        .lock()
        .map_err(|_| "clipboard_recorder_lock_failed".to_string())?;
    recorder.suppress_next_hash(request.hash);
    Ok(())
}

#[tauri::command]
pub fn list_clipboard_history(
    store: State<'_, ClipboardHistoryStore>,
) -> Result<ClipboardHistorySnapshot, String> {
    store.list().map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_clipboard_history_item(
    store: State<'_, ClipboardHistoryStore>,
    request: ClipboardHistoryItemRequest,
) -> Result<crate::clipboard::ClipboardItem, String> {
    store.get(&request.id).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn restore_clipboard_history_item(
    recorder: State<'_, Mutex<ClipboardRecorder>>,
    store: State<'_, ClipboardHistoryStore>,
    request: ClipboardHistoryItemRequest,
) -> Result<(), String> {
    let item = store.get(&request.id).map_err(|error| error.to_string())?;
    crate::clipboard::write_item_to_clipboard(&item).map_err(|error| error.to_string())?;

    let mut recorder = recorder
        .lock()
        .map_err(|_| "clipboard_recorder_lock_failed".to_string())?;
    recorder.suppress_next_hash(item.hash);
    Ok(())
}

#[tauri::command]
pub fn delete_clipboard_history_item(
    app: AppHandle,
    store: State<'_, ClipboardHistoryStore>,
    request: ClipboardHistoryItemRequest,
) -> Result<ClipboardHistorySnapshot, String> {
    let snapshot = store
        .delete(&request.id)
        .map_err(|error| error.to_string())?;
    events::emit_clipboard_deleted(&app, request.id);
    Ok(snapshot)
}

#[tauri::command]
pub fn clear_clipboard_history(
    app: AppHandle,
    store: State<'_, ClipboardHistoryStore>,
) -> Result<usize, String> {
    let deleted_count = store.clear().map_err(|error| error.to_string())?;
    events::emit_history_cleared(&app, deleted_count);
    Ok(deleted_count)
}

#[tauri::command]
pub fn update_clipboard_history_settings(
    store: State<'_, ClipboardHistoryStore>,
    request: UpdateClipboardHistorySettingsRequest,
) -> Result<ClipboardHistorySnapshot, String> {
    store
        .update_settings(request)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn debug_process_clipboard_text(
    app: AppHandle,
    settings_store: State<'_, ClipboardRecorderStore>,
    recorder: State<'_, Mutex<ClipboardRecorder>>,
    history_store: State<'_, ClipboardHistoryStore>,
    request: DebugProcessClipboardTextRequest,
) -> Result<Option<crate::clipboard::ClipboardItem>, String> {
    events::emit_clipboard_changed(&app);
    let settings = settings_store
        .load(&app)
        .map_err(|error| error.to_string())?;
    let mut recorder = recorder
        .lock()
        .map_err(|_| "clipboard_recorder_lock_failed".to_string())?;
    match recorder.process(ClipboardRawContent::Text(request.text), &settings) {
        ClipboardProcessOutcome::Created(item) => {
            let item = history_store
                .push(item)
                .map_err(|error| error.to_string())?;
            events::emit_clipboard_created(&app, &item);
            Ok(Some(item))
        }
        ClipboardProcessOutcome::Duplicate { hash } => {
            events::emit_clipboard_duplicated(&app, hash);
            Ok(None)
        }
        ClipboardProcessOutcome::Ignored(reason) => {
            events::emit_clipboard_read_failed(&app, reason);
            Ok(None)
        }
    }
}

#[tauri::command]
pub fn show_pet_context_menu() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn quit_app(app: AppHandle) -> Result<(), String> {
    app.exit(0);
    Ok(())
}
