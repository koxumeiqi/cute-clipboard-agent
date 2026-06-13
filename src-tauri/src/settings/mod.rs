use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};
use tauri::{AppHandle, Manager};

use crate::{
    clipboard::{
        ClipboardRecorderStore, ClipboardRecordingSettings, UpdateClipboardRecordingSettingsRequest,
    },
    history::{ClipboardHistoryStore, UpdateClipboardHistorySettingsRequest},
};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PetPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PetSettings {
    pub position: PetPosition,
    pub idle_animation_enabled: bool,
    pub auto_move_enabled: bool,
    pub always_on_top: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePetBehaviorSettingsRequest {
    pub idle_animation_enabled: Option<bool>,
    pub auto_move_enabled: Option<bool>,
    pub always_on_top: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppPreferences {
    pub launch_at_startup: bool,
}

impl Default for AppPreferences {
    fn default() -> Self {
        Self {
            launch_at_startup: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub history_capacity: usize,
    pub record_text: bool,
    pub record_image: bool,
    pub idle_animation_enabled: bool,
    pub auto_move_enabled: bool,
    pub launch_at_startup: bool,
    pub persistence_enabled: bool,
    pub recording_paused: bool,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAppSettingsRequest {
    pub history_capacity: Option<usize>,
    pub record_text: Option<bool>,
    pub record_image: Option<bool>,
    pub idle_animation_enabled: Option<bool>,
    pub auto_move_enabled: Option<bool>,
    pub launch_at_startup: Option<bool>,
    pub persistence_enabled: Option<bool>,
    pub recording_paused: Option<bool>,
}

#[derive(Debug, thiserror::Error)]
pub enum PetSettingsError {
    #[error("invalid_position")]
    InvalidPosition,
    #[error("settings_read_failed")]
    ReadFailed,
    #[error("settings_write_failed")]
    WriteFailed,
}

#[derive(Debug, thiserror::Error)]
pub enum AppSettingsError {
    #[error("settings_read_failed")]
    ReadFailed,
    #[error("settings_write_failed")]
    WriteFailed,
    #[error("{0}")]
    Pet(#[from] PetSettingsError),
    #[error("{0}")]
    Clipboard(#[from] crate::clipboard::ClipboardSettingsError),
    #[error("{0}")]
    History(#[from] crate::history::ClipboardHistoryError),
}

pub struct PetSettingsStore {
    cache: Mutex<Option<PetSettings>>,
}

pub struct AppPreferencesStore {
    cache: Mutex<Option<AppPreferences>>,
}

impl Default for PetSettingsStore {
    fn default() -> Self {
        Self {
            cache: Mutex::new(None),
        }
    }
}

impl Default for PetSettings {
    fn default() -> Self {
        Self {
            position: PetPosition { x: 80.0, y: 120.0 },
            idle_animation_enabled: true,
            auto_move_enabled: false,
            always_on_top: true,
        }
    }
}

impl Default for AppPreferencesStore {
    fn default() -> Self {
        Self {
            cache: Mutex::new(None),
        }
    }
}

impl PetPosition {
    pub fn validate(self) -> Result<Self, PetSettingsError> {
        if self.x.is_finite() && self.y.is_finite() {
            Ok(self)
        } else {
            Err(PetSettingsError::InvalidPosition)
        }
    }
}

impl PetSettingsStore {
    pub fn load(&self, app: &AppHandle) -> Result<PetSettings, PetSettingsError> {
        let mut cache = self
            .cache
            .lock()
            .map_err(|_| PetSettingsError::ReadFailed)?;
        if let Some(settings) = cache.clone() {
            return Ok(settings);
        }

        let settings = read_settings(&settings_path(app)).unwrap_or_default();
        *cache = Some(settings.clone());
        Ok(settings)
    }

    pub fn save_position(
        &self,
        app: &AppHandle,
        position: PetPosition,
    ) -> Result<PetSettings, PetSettingsError> {
        let mut settings = self.load(app)?;
        settings.position = position.validate()?;
        self.persist(app, settings)
    }

    pub fn update_behavior(
        &self,
        app: &AppHandle,
        input: UpdatePetBehaviorSettingsRequest,
    ) -> Result<PetSettings, PetSettingsError> {
        let mut settings = self.load(app)?;
        if let Some(value) = input.idle_animation_enabled {
            settings.idle_animation_enabled = value;
        }
        if let Some(value) = input.auto_move_enabled {
            settings.auto_move_enabled = value;
        }
        if let Some(value) = input.always_on_top {
            settings.always_on_top = value;
        }
        self.persist(app, settings)
    }

    fn persist(
        &self,
        app: &AppHandle,
        settings: PetSettings,
    ) -> Result<PetSettings, PetSettingsError> {
        write_settings(&settings_path(app), &settings)?;
        let mut cache = self
            .cache
            .lock()
            .map_err(|_| PetSettingsError::WriteFailed)?;
        *cache = Some(settings.clone());
        Ok(settings)
    }
}

impl AppPreferencesStore {
    pub fn load(&self, app: &AppHandle) -> Result<AppPreferences, AppSettingsError> {
        let mut cache = self.cache.lock().map_err(|_| AppSettingsError::ReadFailed)?;
        if let Some(preferences) = cache.clone() {
            return Ok(preferences);
        }

        let preferences = read_app_preferences(&app_preferences_path(app)).unwrap_or_default();
        *cache = Some(preferences.clone());
        Ok(preferences)
    }

    pub fn update(
        &self,
        app: &AppHandle,
        launch_at_startup: Option<bool>,
    ) -> Result<AppPreferences, AppSettingsError> {
        let mut preferences = self.load(app)?;
        if let Some(value) = launch_at_startup {
            preferences.launch_at_startup = value;
        }
        self.persist(app, preferences)
    }

    fn persist(
        &self,
        app: &AppHandle,
        preferences: AppPreferences,
    ) -> Result<AppPreferences, AppSettingsError> {
        write_app_preferences(&app_preferences_path(app), &preferences)?;
        let mut cache = self.cache.lock().map_err(|_| AppSettingsError::WriteFailed)?;
        *cache = Some(preferences.clone());
        Ok(preferences)
    }
}

pub fn aggregate_app_settings(
    pet: &PetSettings,
    clipboard: &ClipboardRecordingSettings,
    history: &crate::history::ClipboardHistorySettings,
    preferences: &AppPreferences,
) -> AppSettings {
    AppSettings {
        history_capacity: history.capacity,
        record_text: clipboard.record_text,
        record_image: clipboard.record_image,
        idle_animation_enabled: pet.idle_animation_enabled,
        auto_move_enabled: pet.auto_move_enabled,
        launch_at_startup: preferences.launch_at_startup,
        persistence_enabled: history.persist_enabled,
        recording_paused: clipboard.paused,
    }
}

pub fn load_app_settings(
    app: &AppHandle,
    pet_store: &PetSettingsStore,
    clipboard_store: &ClipboardRecorderStore,
    history_store: &ClipboardHistoryStore,
    preferences_store: &AppPreferencesStore,
) -> Result<AppSettings, AppSettingsError> {
    let pet = pet_store.load(app)?;
    let clipboard = clipboard_store.load(app)?;
    let history = history_store.list()?.settings;
    let preferences = preferences_store.load(app)?;
    Ok(aggregate_app_settings(
        &pet,
        &clipboard,
        &history,
        &preferences,
    ))
}

pub fn update_app_settings(
    app: &AppHandle,
    pet_store: &PetSettingsStore,
    clipboard_store: &ClipboardRecorderStore,
    history_store: &ClipboardHistoryStore,
    preferences_store: &AppPreferencesStore,
    input: UpdateAppSettingsRequest,
) -> Result<AppSettings, AppSettingsError> {
    let pet = pet_store.update_behavior(
        app,
        UpdatePetBehaviorSettingsRequest {
            idle_animation_enabled: input.idle_animation_enabled,
            auto_move_enabled: input.auto_move_enabled,
            always_on_top: None,
        },
    )?;
    let mut clipboard = clipboard_store.update(
        app,
        UpdateClipboardRecordingSettingsRequest {
            record_text: input.record_text,
            record_image: input.record_image,
        },
    )?;
    if let Some(paused) = input.recording_paused {
        clipboard = clipboard_store.set_paused(app, paused)?;
    }
    let history = history_store
        .update_settings(UpdateClipboardHistorySettingsRequest {
            capacity: input.history_capacity,
            persist_enabled: input.persistence_enabled,
        })?
        .settings;
    let preferences = preferences_store.update(app, input.launch_at_startup)?;
    Ok(aggregate_app_settings(
        &pet,
        &clipboard,
        &history,
        &preferences,
    ))
}

pub fn settings_path(app: &AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .unwrap_or_else(|_| std::env::temp_dir().join("cute-clipboard-agent"))
        .join("pet-settings.json")
}

pub fn app_preferences_path(app: &AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .unwrap_or_else(|_| std::env::temp_dir().join("cute-clipboard-agent"))
        .join("app-preferences.json")
}

pub fn read_settings(path: &Path) -> Result<PetSettings, PetSettingsError> {
    if !path.exists() {
        return Ok(PetSettings::default());
    }

    let raw = fs::read_to_string(path).map_err(|_| PetSettingsError::ReadFailed)?;
    serde_json::from_str(&raw).map_err(|_| PetSettingsError::ReadFailed)
}

pub fn read_app_preferences(path: &Path) -> Result<AppPreferences, AppSettingsError> {
    if !path.exists() {
        return Ok(AppPreferences::default());
    }

    let raw = fs::read_to_string(path).map_err(|_| AppSettingsError::ReadFailed)?;
    serde_json::from_str(&raw).map_err(|_| AppSettingsError::ReadFailed)
}

pub fn write_settings(path: &Path, settings: &PetSettings) -> Result<(), PetSettingsError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|_| PetSettingsError::WriteFailed)?;
    }
    let raw = serde_json::to_string_pretty(settings).map_err(|_| PetSettingsError::WriteFailed)?;
    fs::write(path, raw).map_err(|_| PetSettingsError::WriteFailed)
}

pub fn write_app_preferences(
    path: &Path,
    preferences: &AppPreferences,
) -> Result<(), AppSettingsError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|_| AppSettingsError::WriteFailed)?;
    }
    let raw =
        serde_json::to_string_pretty(preferences).map_err(|_| AppSettingsError::WriteFailed)?;
    fs::write(path, raw).map_err(|_| AppSettingsError::WriteFailed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings_match_prd() {
        assert_eq!(
            PetSettings::default(),
            PetSettings {
                position: PetPosition { x: 80.0, y: 120.0 },
                idle_animation_enabled: true,
                auto_move_enabled: false,
                always_on_top: true,
            }
        );
    }

    #[test]
    fn rejects_non_finite_position() {
        assert_eq!(
            PetPosition {
                x: f64::NAN,
                y: 1.0
            }
            .validate()
            .unwrap_err()
            .to_string(),
            "invalid_position"
        );
    }

    #[test]
    fn persists_and_reads_settings() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("pet-settings.json");
        let settings = PetSettings {
            position: PetPosition { x: 240.0, y: 320.0 },
            idle_animation_enabled: false,
            auto_move_enabled: false,
            always_on_top: true,
        };

        write_settings(&path, &settings).expect("write");
        assert_eq!(read_settings(&path).expect("read"), settings);
    }

    #[test]
    fn default_app_preferences_match_prd() {
        assert_eq!(
            AppPreferences::default(),
            AppPreferences {
                launch_at_startup: false,
            }
        );
    }

    #[test]
    fn persists_and_reads_app_preferences() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("app-preferences.json");
        let preferences = AppPreferences {
            launch_at_startup: true,
        };

        write_app_preferences(&path, &preferences).expect("write");
        assert_eq!(read_app_preferences(&path).expect("read"), preferences);
    }

    #[test]
    fn aggregates_app_settings_from_domain_settings() {
        let pet = PetSettings {
            position: PetPosition { x: 1.0, y: 2.0 },
            idle_animation_enabled: false,
            auto_move_enabled: true,
            always_on_top: true,
        };
        let clipboard = ClipboardRecordingSettings {
            paused: true,
            record_text: false,
            record_image: true,
        };
        let history = crate::history::ClipboardHistorySettings {
            capacity: 10,
            persist_enabled: false,
        };
        let preferences = AppPreferences {
            launch_at_startup: true,
        };

        assert_eq!(
            aggregate_app_settings(&pet, &clipboard, &history, &preferences),
            AppSettings {
                history_capacity: 10,
                record_text: false,
                record_image: true,
                idle_animation_enabled: false,
                auto_move_enabled: true,
                launch_at_startup: true,
                persistence_enabled: false,
                recording_paused: true,
            }
        );
    }
}
