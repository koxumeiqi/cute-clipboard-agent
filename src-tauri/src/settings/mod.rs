use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};
use tauri::{AppHandle, Manager};

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

#[derive(Debug, thiserror::Error)]
pub enum PetSettingsError {
    #[error("invalid_position")]
    InvalidPosition,
    #[error("settings_read_failed")]
    ReadFailed,
    #[error("settings_write_failed")]
    WriteFailed,
}

pub struct PetSettingsStore {
    cache: Mutex<Option<PetSettings>>,
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

pub fn settings_path(app: &AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .unwrap_or_else(|_| std::env::temp_dir().join("cute-clipboard-agent"))
        .join("pet-settings.json")
}

pub fn read_settings(path: &Path) -> Result<PetSettings, PetSettingsError> {
    if !path.exists() {
        return Ok(PetSettings::default());
    }

    let raw = fs::read_to_string(path).map_err(|_| PetSettingsError::ReadFailed)?;
    serde_json::from_str(&raw).map_err(|_| PetSettingsError::ReadFailed)
}

pub fn write_settings(path: &Path, settings: &PetSettings) -> Result<(), PetSettingsError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|_| PetSettingsError::WriteFailed)?;
    }
    let raw = serde_json::to_string_pretty(settings).map_err(|_| PetSettingsError::WriteFailed)?;
    fs::write(path, raw).map_err(|_| PetSettingsError::WriteFailed)
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
}
