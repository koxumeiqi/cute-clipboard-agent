use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::VecDeque,
    fs,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    sync::Mutex,
    thread,
    time::Duration,
};
use tauri::{AppHandle, Manager};
use time::OffsetDateTime;

const MAX_SUPPRESSED_HASHES: usize = 16;
const PREVIEW_CHAR_LIMIT: usize = 80;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClipboardItemType {
    Text,
    Image,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipboardItem {
    pub id: String,
    #[serde(rename = "type")]
    pub item_type: ClipboardItemType,
    pub preview: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_path: Option<String>,
    pub hash: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipboardEventItem {
    pub id: String,
    #[serde(rename = "type")]
    pub item_type: ClipboardItemType,
    pub preview: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_path: Option<String>,
    pub hash: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<&ClipboardItem> for ClipboardEventItem {
    fn from(item: &ClipboardItem) -> Self {
        Self {
            id: item.id.clone(),
            item_type: item.item_type.clone(),
            preview: item.preview.clone(),
            image_path: item.image_path.clone(),
            thumbnail_path: item.thumbnail_path.clone(),
            hash: item.hash.clone(),
            created_at: item.created_at.clone(),
            updated_at: item.updated_at.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipboardRecordingSettings {
    pub paused: bool,
    pub record_text: bool,
    pub record_image: bool,
}

impl Default for ClipboardRecordingSettings {
    fn default() -> Self {
        Self {
            paused: false,
            record_text: true,
            record_image: true,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateClipboardRecordingSettingsRequest {
    pub record_text: Option<bool>,
    pub record_image: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuppressNextClipboardHashRequest {
    pub hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ClipboardRawContent {
    Text(String),
    Image {
        bytes: Vec<u8>,
        extension: String,
        width: Option<u32>,
        height: Option<u32>,
    },
    Unsupported,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub enum ClipboardReadFailureReason {
    Empty,
    UnsupportedType,
    ReadFailed,
    RecordingPaused,
    TypeDisabled,
    Duplicate,
    SelfWriteSuppressed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClipboardProcessOutcome {
    Created(ClipboardItem),
    Duplicate { hash: String },
    Ignored(ClipboardReadFailureReason),
}

#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum ClipboardSettingsError {
    #[error("clipboard_settings_read_failed")]
    ReadFailed,
    #[error("clipboard_settings_write_failed")]
    WriteFailed,
    #[error("invalid_clipboard_settings")]
    Invalid,
}

#[derive(Debug, thiserror::Error)]
pub enum ClipboardRestoreError {
    #[error("unsupported_clipboard_restore")]
    Unsupported,
    #[error("clipboard_write_failed")]
    WriteFailed,
}

pub struct ClipboardRecorderStore {
    cache: Mutex<Option<ClipboardRecordingSettings>>,
}

impl Default for ClipboardRecorderStore {
    fn default() -> Self {
        Self {
            cache: Mutex::new(None),
        }
    }
}

impl ClipboardRecorderStore {
    pub fn load(
        &self,
        app: &AppHandle,
    ) -> Result<ClipboardRecordingSettings, ClipboardSettingsError> {
        let mut cache = self
            .cache
            .lock()
            .map_err(|_| ClipboardSettingsError::ReadFailed)?;
        if let Some(settings) = cache.clone() {
            return Ok(settings);
        }

        let settings = read_recording_settings(&recording_settings_path(app)).unwrap_or_default();
        *cache = Some(settings.clone());
        Ok(settings)
    }

    pub fn set_paused(
        &self,
        app: &AppHandle,
        paused: bool,
    ) -> Result<ClipboardRecordingSettings, ClipboardSettingsError> {
        let mut settings = self.load(app)?;
        settings.paused = paused;
        self.persist(app, settings)
    }

    pub fn update(
        &self,
        app: &AppHandle,
        input: UpdateClipboardRecordingSettingsRequest,
    ) -> Result<ClipboardRecordingSettings, ClipboardSettingsError> {
        let mut settings = self.load(app)?;
        if let Some(value) = input.record_text {
            settings.record_text = value;
        }
        if let Some(value) = input.record_image {
            settings.record_image = value;
        }
        self.persist(app, settings)
    }

    fn persist(
        &self,
        app: &AppHandle,
        settings: ClipboardRecordingSettings,
    ) -> Result<ClipboardRecordingSettings, ClipboardSettingsError> {
        validate_recording_settings(&settings)?;
        write_recording_settings(&recording_settings_path(app), &settings)?;
        let mut cache = self
            .cache
            .lock()
            .map_err(|_| ClipboardSettingsError::WriteFailed)?;
        *cache = Some(settings.clone());
        Ok(settings)
    }
}

pub struct ClipboardRecorder {
    last_hash: Option<String>,
    suppressed_hashes: VecDeque<String>,
}

impl Default for ClipboardRecorder {
    fn default() -> Self {
        Self {
            last_hash: None,
            suppressed_hashes: VecDeque::new(),
        }
    }
}

pub fn start_clipboard_polling_listener(app: AppHandle) {
    let _ = thread::Builder::new()
        .name("cute-clipboard-listener".to_string())
        .spawn(move || {
            let mut clipboard = match arboard::Clipboard::new() {
                Ok(clipboard) => clipboard,
                Err(_) => {
                    crate::events::emit_clipboard_read_failed(
                        &app,
                        ClipboardReadFailureReason::ReadFailed,
                    );
                    return;
                }
            };
            let mut observed_hash: Option<String> = None;

            loop {
                thread::sleep(Duration::from_millis(600));

                let raw = match read_arboard_content(&mut clipboard) {
                    Some(raw) => raw,
                    None => continue,
                };
                let fingerprint = raw_content_hash(&raw);
                if observed_hash.as_deref() == Some(fingerprint.as_str()) {
                    continue;
                }
                observed_hash = Some(fingerprint);

                crate::events::emit_clipboard_changed(&app);

                let settings_store = app.state::<ClipboardRecorderStore>();
                let settings = match settings_store.load(&app) {
                    Ok(settings) => settings,
                    Err(_) => {
                        crate::events::emit_clipboard_read_failed(
                            &app,
                            ClipboardReadFailureReason::ReadFailed,
                        );
                        continue;
                    }
                };

                let recorder_state = app.state::<Mutex<ClipboardRecorder>>();
                let mut recorder = match recorder_state.lock() {
                    Ok(recorder) => recorder,
                    Err(_) => {
                        crate::events::emit_clipboard_read_failed(
                            &app,
                            ClipboardReadFailureReason::ReadFailed,
                        );
                        continue;
                    }
                };

                match recorder.process(raw, &settings) {
                    ClipboardProcessOutcome::Created(item) => {
                        let history_store = app.state::<crate::history::ClipboardHistoryStore>();
                        match history_store.push(item) {
                            Ok(item) => crate::events::emit_clipboard_created(&app, &item),
                            Err(_) => crate::events::emit_clipboard_read_failed(
                                &app,
                                ClipboardReadFailureReason::ReadFailed,
                            ),
                        }
                    }
                    ClipboardProcessOutcome::Duplicate { hash } => {
                        crate::events::emit_clipboard_duplicated(&app, hash);
                    }
                    ClipboardProcessOutcome::Ignored(reason) => {
                        crate::events::emit_clipboard_read_failed(&app, reason);
                    }
                }
            }
        });
}

impl ClipboardRecorder {
    pub fn suppress_next_hash(&mut self, hash: String) {
        if hash.trim().is_empty() {
            return;
        }
        if self.suppressed_hashes.len() >= MAX_SUPPRESSED_HASHES {
            self.suppressed_hashes.pop_front();
        }
        self.suppressed_hashes.push_back(hash);
    }

    pub fn process(
        &mut self,
        raw: ClipboardRawContent,
        settings: &ClipboardRecordingSettings,
    ) -> ClipboardProcessOutcome {
        if settings.paused {
            return ClipboardProcessOutcome::Ignored(ClipboardReadFailureReason::RecordingPaused);
        }

        let normalized = match raw {
            ClipboardRawContent::Text(text) => normalize_text(text, settings),
            ClipboardRawContent::Image {
                bytes,
                extension,
                width,
                height,
            } => normalize_image(bytes, extension, width, height, settings),
            ClipboardRawContent::Unsupported => Err(ClipboardReadFailureReason::UnsupportedType),
            ClipboardRawContent::Empty => Err(ClipboardReadFailureReason::Empty),
        };

        let item = match normalized {
            Ok(item) => item,
            Err(reason) => return ClipboardProcessOutcome::Ignored(reason),
        };

        if self.consume_suppressed_hash(&item.hash) {
            return ClipboardProcessOutcome::Ignored(
                ClipboardReadFailureReason::SelfWriteSuppressed,
            );
        }

        if self.last_hash.as_deref() == Some(item.hash.as_str()) {
            return ClipboardProcessOutcome::Duplicate {
                hash: item.hash.clone(),
            };
        }

        self.last_hash = Some(item.hash.clone());
        ClipboardProcessOutcome::Created(item)
    }

    fn consume_suppressed_hash(&mut self, hash: &str) -> bool {
        if let Some(index) = self
            .suppressed_hashes
            .iter()
            .position(|value| value == hash)
        {
            self.suppressed_hashes.remove(index);
            return true;
        }
        false
    }
}

fn read_arboard_content(clipboard: &mut arboard::Clipboard) -> Option<ClipboardRawContent> {
    if let Ok(text) = clipboard.get_text() {
        if !text.is_empty() {
            return Some(ClipboardRawContent::Text(text));
        }
    }

    if let Ok(image) = clipboard.get_image() {
        return Some(ClipboardRawContent::Image {
            bytes: image.bytes.into_owned(),
            extension: "png".to_string(),
            width: Some(image.width as u32),
            height: Some(image.height as u32),
        });
    }

    None
}

pub fn recording_settings_path(app: &AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .unwrap_or_else(|_| std::env::temp_dir().join("cute-clipboard-agent"))
        .join("clipboard-recording-settings.json")
}

pub fn read_recording_settings(
    path: &Path,
) -> Result<ClipboardRecordingSettings, ClipboardSettingsError> {
    if !path.exists() {
        return Ok(ClipboardRecordingSettings::default());
    }

    let raw = fs::read_to_string(path).map_err(|_| ClipboardSettingsError::ReadFailed)?;
    let settings: ClipboardRecordingSettings =
        serde_json::from_str(&raw).map_err(|_| ClipboardSettingsError::ReadFailed)?;
    validate_recording_settings(&settings)?;
    Ok(settings)
}

pub fn write_recording_settings(
    path: &Path,
    settings: &ClipboardRecordingSettings,
) -> Result<(), ClipboardSettingsError> {
    validate_recording_settings(settings)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|_| ClipboardSettingsError::WriteFailed)?;
    }
    let raw =
        serde_json::to_string_pretty(settings).map_err(|_| ClipboardSettingsError::WriteFailed)?;
    fs::write(path, raw).map_err(|_| ClipboardSettingsError::WriteFailed)
}

pub fn normalize_text(
    text: String,
    settings: &ClipboardRecordingSettings,
) -> Result<ClipboardItem, ClipboardReadFailureReason> {
    if !settings.record_text {
        return Err(ClipboardReadFailureReason::TypeDisabled);
    }

    if text.is_empty() {
        return Err(ClipboardReadFailureReason::Empty);
    }

    let hash = content_hash("text", text.as_bytes());
    let at = now_rfc3339();
    Ok(ClipboardItem {
        id: item_id(&hash, &at),
        item_type: ClipboardItemType::Text,
        preview: preview_text(&text),
        text: Some(text),
        image_path: None,
        thumbnail_path: None,
        hash,
        created_at: at.clone(),
        updated_at: at,
    })
}

pub fn normalize_image(
    bytes: Vec<u8>,
    extension: String,
    width: Option<u32>,
    height: Option<u32>,
    settings: &ClipboardRecordingSettings,
) -> Result<ClipboardItem, ClipboardReadFailureReason> {
    if !settings.record_image {
        return Err(ClipboardReadFailureReason::TypeDisabled);
    }
    if bytes.is_empty() {
        return Err(ClipboardReadFailureReason::Empty);
    }

    let hash = content_hash("image", &bytes);
    let at = now_rfc3339();
    let extension = sanitize_extension(&extension);
    let dimensions = match (width, height) {
        (Some(width), Some(height)) => format!(" {}x{}", width, height),
        _ => String::new(),
    };
    Ok(ClipboardItem {
        id: item_id(&hash, &at),
        item_type: ClipboardItemType::Image,
        preview: format!("Image {} bytes{}", bytes.len(), dimensions),
        text: None,
        image_path: Some(format!("pending://clipboard/{}.{}", hash, extension)),
        thumbnail_path: Some(format!("pending://clipboard/{}-thumb.{}", hash, extension)),
        hash,
        created_at: at.clone(),
        updated_at: at,
    })
}

pub fn image_data_from_path(
    path: &str,
) -> Result<arboard::ImageData<'static>, ClipboardRestoreError> {
    let image = ::image::open(path).map_err(|_| ClipboardRestoreError::WriteFailed)?;
    let rgba = image.to_rgba8();
    let width = rgba.width() as usize;
    let height = rgba.height() as usize;
    Ok(arboard::ImageData {
        width,
        height,
        bytes: Cow::Owned(rgba.into_raw()),
    })
}

pub fn write_item_to_clipboard(item: &ClipboardItem) -> Result<(), ClipboardRestoreError> {
    let mut clipboard =
        arboard::Clipboard::new().map_err(|_| ClipboardRestoreError::WriteFailed)?;
    match item.item_type {
        ClipboardItemType::Text => {
            let text = item
                .text
                .clone()
                .ok_or(ClipboardRestoreError::Unsupported)?;
            clipboard
                .set_text(text)
                .map_err(|_| ClipboardRestoreError::WriteFailed)
        }
        ClipboardItemType::Image => {
            let image_path = item
                .image_path
                .as_deref()
                .ok_or(ClipboardRestoreError::Unsupported)?;
            let image = image_data_from_path(image_path)?;
            clipboard
                .set_image(image)
                .map_err(|_| ClipboardRestoreError::WriteFailed)
        }
    }
}

fn validate_recording_settings(
    _settings: &ClipboardRecordingSettings,
) -> Result<(), ClipboardSettingsError> {
    Ok(())
}

fn preview_text(text: &str) -> String {
    let mut preview: String = text.chars().take(PREVIEW_CHAR_LIMIT).collect();
    if text.chars().count() > PREVIEW_CHAR_LIMIT {
        preview.push('…');
    }
    preview
}

fn content_hash(kind: &str, bytes: &[u8]) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    kind.hash(&mut hasher);
    bytes.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

fn raw_content_hash(raw: &ClipboardRawContent) -> String {
    match raw {
        ClipboardRawContent::Text(text) => content_hash("text", text.as_bytes()),
        ClipboardRawContent::Image { bytes, .. } => content_hash("image", bytes),
        ClipboardRawContent::Unsupported => content_hash("unsupported", &[]),
        ClipboardRawContent::Empty => content_hash("empty", &[]),
    }
}

fn item_id(hash: &str, at: &str) -> String {
    format!("clip-{}-{}", hash, content_hash("time", at.as_bytes()))
}

fn sanitize_extension(extension: &str) -> String {
    let normalized = extension
        .trim()
        .trim_start_matches('.')
        .to_ascii_lowercase();
    if normalized.chars().all(|ch| ch.is_ascii_alphanumeric()) && !normalized.is_empty() {
        normalized
    } else {
        "png".to_string()
    }
}

fn now_rfc3339() -> String {
    OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings() -> ClipboardRecordingSettings {
        ClipboardRecordingSettings::default()
    }

    #[test]
    fn default_recording_settings_match_prd() {
        assert_eq!(
            ClipboardRecordingSettings::default(),
            ClipboardRecordingSettings {
                paused: false,
                record_text: true,
                record_image: true,
            }
        );
    }

    #[test]
    fn normalizes_text_clipboard_content() {
        let item = normalize_text("hello clipboard".to_string(), &settings()).expect("item");

        assert_eq!(item.item_type, ClipboardItemType::Text);
        assert_eq!(item.preview, "hello clipboard");
        assert_eq!(item.text.as_deref(), Some("hello clipboard"));
        assert!(!item.hash.is_empty());
    }

    #[test]
    fn keeps_emoji_as_text_preview() {
        let item = normalize_text("😀🚀".to_string(), &settings()).expect("item");

        assert_eq!(item.item_type, ClipboardItemType::Text);
        assert_eq!(item.preview, "😀🚀");
        assert_eq!(item.text.as_deref(), Some("😀🚀"));
    }

    #[test]
    fn rejects_empty_text() {
        assert_eq!(
            normalize_text(String::new(), &settings()).unwrap_err(),
            ClipboardReadFailureReason::Empty
        );
    }

    #[test]
    fn ignores_text_when_text_recording_is_disabled() {
        let settings = ClipboardRecordingSettings {
            record_text: false,
            ..ClipboardRecordingSettings::default()
        };

        assert_eq!(
            normalize_text("hello".to_string(), &settings).unwrap_err(),
            ClipboardReadFailureReason::TypeDisabled
        );
    }

    #[test]
    fn normalizes_image_clipboard_content() {
        let item = normalize_image(
            vec![1, 2, 3, 4],
            "PNG".to_string(),
            Some(2),
            Some(2),
            &settings(),
        )
        .expect("item");

        assert_eq!(item.item_type, ClipboardItemType::Image);
        assert_eq!(item.preview, "Image 4 bytes 2x2");
        assert!(item.text.is_none());
        assert!(item
            .image_path
            .as_deref()
            .unwrap_or_default()
            .ends_with(".png"));
    }

    #[test]
    fn ignores_image_when_image_recording_is_disabled() {
        let settings = ClipboardRecordingSettings {
            record_image: false,
            ..ClipboardRecordingSettings::default()
        };

        assert_eq!(
            normalize_image(vec![1], "png".to_string(), None, None, &settings).unwrap_err(),
            ClipboardReadFailureReason::TypeDisabled
        );
    }

    #[test]
    fn process_ignores_changes_when_paused() {
        let mut recorder = ClipboardRecorder::default();
        let settings = ClipboardRecordingSettings {
            paused: true,
            ..ClipboardRecordingSettings::default()
        };

        assert_eq!(
            recorder.process(ClipboardRawContent::Text("hello".to_string()), &settings),
            ClipboardProcessOutcome::Ignored(ClipboardReadFailureReason::RecordingPaused)
        );
    }

    #[test]
    fn process_deduplicates_top_hash() {
        let mut recorder = ClipboardRecorder::default();
        let first = recorder.process(ClipboardRawContent::Text("same".to_string()), &settings());
        let second = recorder.process(ClipboardRawContent::Text("same".to_string()), &settings());

        let hash = match first {
            ClipboardProcessOutcome::Created(item) => item.hash,
            _ => panic!("first change should create item"),
        };
        assert_eq!(second, ClipboardProcessOutcome::Duplicate { hash });
    }

    #[test]
    fn process_suppresses_self_write_hash_once() {
        let mut recorder = ClipboardRecorder::default();
        let item = normalize_text("restored".to_string(), &settings()).expect("item");
        recorder.suppress_next_hash(item.hash.clone());

        assert_eq!(
            recorder.process(
                ClipboardRawContent::Text("restored".to_string()),
                &settings()
            ),
            ClipboardProcessOutcome::Ignored(ClipboardReadFailureReason::SelfWriteSuppressed)
        );
        assert!(matches!(
            recorder.process(
                ClipboardRawContent::Text("restored".to_string()),
                &settings()
            ),
            ClipboardProcessOutcome::Created(_)
        ));
    }

    #[test]
    fn event_item_does_not_include_full_text() {
        let item = normalize_text("secret full text".to_string(), &settings()).expect("item");
        let event_item = ClipboardEventItem::from(&item);

        assert_eq!(event_item.preview, "secret full text");
        assert_eq!(event_item.hash, item.hash);
    }

    #[test]
    fn image_data_from_path_reads_image_dimensions_and_bytes() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("restore.png");
        let image = ::image::RgbaImage::from_pixel(2, 3, ::image::Rgba([10, 20, 30, 255]));
        image.save(&path).expect("save image");

        let data = image_data_from_path(path.to_str().expect("path")).expect("image data");

        assert_eq!(data.width, 2);
        assert_eq!(data.height, 3);
        assert_eq!(data.bytes.len(), 24);
    }

    #[test]
    fn image_data_from_path_rejects_missing_file() {
        assert!(matches!(
            image_data_from_path("missing-image.png"),
            Err(ClipboardRestoreError::WriteFailed)
        ));
    }

    #[test]
    fn persists_and_reads_recording_settings() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("clipboard-settings.json");
        let settings = ClipboardRecordingSettings {
            paused: true,
            record_text: false,
            record_image: true,
        };

        write_recording_settings(&path, &settings).expect("write");
        assert_eq!(read_recording_settings(&path).expect("read"), settings);
    }
}
