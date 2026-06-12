use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
    sync::Mutex,
};

use rusqlite::{params, Connection};

use crate::{
    clipboard::{ClipboardItem, ClipboardItemType},
    image,
};

pub const DEFAULT_HISTORY_CAPACITY: usize = 20;
pub const ALLOWED_HISTORY_CAPACITIES: [usize; 3] = [10, 20, 50];

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipboardHistorySettings {
    pub capacity: usize,
    pub persist_enabled: bool,
}

impl Default for ClipboardHistorySettings {
    fn default() -> Self {
        Self {
            capacity: DEFAULT_HISTORY_CAPACITY,
            persist_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateClipboardHistorySettingsRequest {
    pub capacity: Option<usize>,
    pub persist_enabled: Option<bool>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipboardHistoryItemRequest {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipboardHistorySnapshot {
    pub items: Vec<ClipboardItem>,
    pub settings: ClipboardHistorySettings,
    pub total: usize,
}

#[derive(Debug, thiserror::Error)]
pub enum ClipboardHistoryError {
    #[error("invalid_history_capacity")]
    InvalidCapacity,
    #[error("history_item_not_found")]
    NotFound,
    #[error("history_store_lock_failed")]
    LockFailed,
    #[error("image_cleanup_failed")]
    ImageCleanupFailed,
    #[error("history_storage_failed")]
    StorageFailed,
}

#[derive(Default)]
pub struct ClipboardHistoryStore {
    inner: Mutex<ClipboardHistoryState>,
}

#[derive(Default)]
struct ClipboardHistoryState {
    items: VecDeque<ClipboardItem>,
    settings: ClipboardHistorySettings,
    database_path: Option<PathBuf>,
}

impl ClipboardHistoryStore {
    pub fn initialize(&self, database_path: PathBuf) -> Result<(), ClipboardHistoryError> {
        let mut state = self
            .inner
            .lock()
            .map_err(|_| ClipboardHistoryError::LockFailed)?;
        let connection = open_database(&database_path)?;
        initialize_schema(&connection)?;
        state.settings = load_settings(&connection)?;
        state.items = load_items(&connection, state.settings.capacity)?;
        state.database_path = Some(database_path);
        Ok(())
    }

    pub fn list(&self) -> Result<ClipboardHistorySnapshot, ClipboardHistoryError> {
        let state = self
            .inner
            .lock()
            .map_err(|_| ClipboardHistoryError::LockFailed)?;
        Ok(snapshot(&state))
    }

    pub fn get(&self, id: &str) -> Result<ClipboardItem, ClipboardHistoryError> {
        let state = self
            .inner
            .lock()
            .map_err(|_| ClipboardHistoryError::LockFailed)?;
        state
            .items
            .iter()
            .find(|item| item.id == id)
            .cloned()
            .ok_or(ClipboardHistoryError::NotFound)
    }

    pub fn push(&self, item: ClipboardItem) -> Result<ClipboardItem, ClipboardHistoryError> {
        let mut state = self
            .inner
            .lock()
            .map_err(|_| ClipboardHistoryError::LockFailed)?;
        if let Some(index) = state
            .items
            .iter()
            .position(|existing| existing.id == item.id)
        {
            state.items.remove(index);
        }
        state.items.push_front(item.clone());
        trim_to_capacity(&mut state)?;
        persist_state(&state)?;
        Ok(item)
    }

    pub fn delete(&self, id: &str) -> Result<ClipboardHistorySnapshot, ClipboardHistoryError> {
        let mut state = self
            .inner
            .lock()
            .map_err(|_| ClipboardHistoryError::LockFailed)?;
        let index = state
            .items
            .iter()
            .position(|item| item.id == id)
            .ok_or(ClipboardHistoryError::NotFound)?;
        let item = state
            .items
            .remove(index)
            .ok_or(ClipboardHistoryError::NotFound)?;
        cleanup_item_images(&item)?;
        persist_state(&state)?;
        Ok(snapshot(&state))
    }

    pub fn clear(&self) -> Result<usize, ClipboardHistoryError> {
        let mut state = self
            .inner
            .lock()
            .map_err(|_| ClipboardHistoryError::LockFailed)?;
        let deleted_count = state.items.len();
        let items: Vec<ClipboardItem> = state.items.drain(..).collect();
        for item in &items {
            cleanup_item_images(item)?;
        }
        persist_state(&state)?;
        Ok(deleted_count)
    }

    pub fn update_settings(
        &self,
        input: UpdateClipboardHistorySettingsRequest,
    ) -> Result<ClipboardHistorySnapshot, ClipboardHistoryError> {
        let mut state = self
            .inner
            .lock()
            .map_err(|_| ClipboardHistoryError::LockFailed)?;
        if let Some(capacity) = input.capacity {
            validate_capacity(capacity)?;
            state.settings.capacity = capacity;
        }
        if let Some(persist_enabled) = input.persist_enabled {
            state.settings.persist_enabled = persist_enabled;
        }
        trim_to_capacity(&mut state)?;
        persist_state(&state)?;
        Ok(snapshot(&state))
    }
}

fn snapshot(state: &ClipboardHistoryState) -> ClipboardHistorySnapshot {
    ClipboardHistorySnapshot {
        items: state.items.iter().cloned().collect(),
        settings: state.settings.clone(),
        total: state.items.len(),
    }
}

fn trim_to_capacity(state: &mut ClipboardHistoryState) -> Result<(), ClipboardHistoryError> {
    while state.items.len() > state.settings.capacity {
        if let Some(item) = state.items.pop_back() {
            cleanup_item_images(&item)?;
        }
    }
    Ok(())
}

fn cleanup_item_images(item: &ClipboardItem) -> Result<(), ClipboardHistoryError> {
    if item.item_type != ClipboardItemType::Image {
        return Ok(());
    }
    image::delete_image_files(&[item.image_path.clone(), item.thumbnail_path.clone()])
        .map_err(|_| ClipboardHistoryError::ImageCleanupFailed)
}

fn validate_capacity(capacity: usize) -> Result<(), ClipboardHistoryError> {
    if ALLOWED_HISTORY_CAPACITIES.contains(&capacity) {
        Ok(())
    } else {
        Err(ClipboardHistoryError::InvalidCapacity)
    }
}

fn persist_state(state: &ClipboardHistoryState) -> Result<(), ClipboardHistoryError> {
    let Some(database_path) = &state.database_path else {
        return Ok(());
    };
    let connection = open_database(database_path)?;
    initialize_schema(&connection)?;
    save_settings(&connection, &state.settings)?;
    if state.settings.persist_enabled {
        replace_items(&connection, &state.items)?;
    }
    Ok(())
}

fn open_database(path: &Path) -> Result<Connection, ClipboardHistoryError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|_| ClipboardHistoryError::StorageFailed)?;
    }
    Connection::open(path).map_err(|_| ClipboardHistoryError::StorageFailed)
}

fn initialize_schema(connection: &Connection) -> Result<(), ClipboardHistoryError> {
    connection
        .execute_batch(
            "
            CREATE TABLE IF NOT EXISTS clipboard_history_items (
                id TEXT PRIMARY KEY NOT NULL,
                item_type TEXT NOT NULL,
                preview TEXT NOT NULL,
                text TEXT,
                image_path TEXT,
                thumbnail_path TEXT,
                hash TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS history_settings (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                capacity INTEGER NOT NULL,
                persist_enabled INTEGER NOT NULL,
                updated_at TEXT NOT NULL
            );
            ",
        )
        .map_err(|_| ClipboardHistoryError::StorageFailed)
}

fn load_settings(
    connection: &Connection,
) -> Result<ClipboardHistorySettings, ClipboardHistoryError> {
    let mut statement = connection
        .prepare("SELECT capacity, persist_enabled FROM history_settings WHERE id = 1")
        .map_err(|_| ClipboardHistoryError::StorageFailed)?;
    let result = statement.query_row([], |row| {
        Ok(ClipboardHistorySettings {
            capacity: row.get::<_, usize>(0)?,
            persist_enabled: row.get::<_, i64>(1)? != 0,
        })
    });

    match result {
        Ok(settings) => {
            validate_capacity(settings.capacity)?;
            Ok(settings)
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            let settings = ClipboardHistorySettings::default();
            save_settings(connection, &settings)?;
            Ok(settings)
        }
        Err(_) => Err(ClipboardHistoryError::StorageFailed),
    }
}

fn save_settings(
    connection: &Connection,
    settings: &ClipboardHistorySettings,
) -> Result<(), ClipboardHistoryError> {
    validate_capacity(settings.capacity)?;
    connection
        .execute(
            "INSERT INTO history_settings (id, capacity, persist_enabled, updated_at)
             VALUES (1, ?1, ?2, datetime('now'))
             ON CONFLICT(id) DO UPDATE SET
                capacity = excluded.capacity,
                persist_enabled = excluded.persist_enabled,
                updated_at = excluded.updated_at",
            params![settings.capacity as i64, settings.persist_enabled as i64],
        )
        .map_err(|_| ClipboardHistoryError::StorageFailed)?;
    Ok(())
}

fn load_items(
    connection: &Connection,
    capacity: usize,
) -> Result<VecDeque<ClipboardItem>, ClipboardHistoryError> {
    let mut statement = connection
        .prepare(
            "SELECT id, item_type, preview, text, image_path, thumbnail_path, hash, created_at, updated_at
             FROM clipboard_history_items
             ORDER BY created_at DESC
             LIMIT ?1",
        )
        .map_err(|_| ClipboardHistoryError::StorageFailed)?;
    let rows = statement
        .query_map(params![capacity as i64], |row| {
            let item_type: String = row.get(1)?;
            Ok(ClipboardItem {
                id: row.get(0)?,
                item_type: if item_type == "image" {
                    ClipboardItemType::Image
                } else {
                    ClipboardItemType::Text
                },
                preview: row.get(2)?,
                text: row.get(3)?,
                image_path: row.get(4)?,
                thumbnail_path: row.get(5)?,
                hash: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })
        .map_err(|_| ClipboardHistoryError::StorageFailed)?;

    let mut items = VecDeque::new();
    for row in rows {
        items.push_back(row.map_err(|_| ClipboardHistoryError::StorageFailed)?);
    }
    Ok(items)
}

fn replace_items(
    connection: &Connection,
    items: &VecDeque<ClipboardItem>,
) -> Result<(), ClipboardHistoryError> {
    connection
        .execute("DELETE FROM clipboard_history_items", [])
        .map_err(|_| ClipboardHistoryError::StorageFailed)?;
    for item in items {
        connection
            .execute(
                "INSERT INTO clipboard_history_items
                 (id, item_type, preview, text, image_path, thumbnail_path, hash, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    item.id,
                    match item.item_type {
                        ClipboardItemType::Text => "text",
                        ClipboardItemType::Image => "image",
                    },
                    item.preview,
                    item.text,
                    item.image_path,
                    item.thumbnail_path,
                    item.hash,
                    item.created_at,
                    item.updated_at
                ],
            )
            .map_err(|_| ClipboardHistoryError::StorageFailed)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text_item(id: usize) -> ClipboardItem {
        ClipboardItem {
            id: format!("clip-{id}"),
            item_type: ClipboardItemType::Text,
            preview: format!("item {id}"),
            text: Some(format!("item {id}")),
            image_path: None,
            thumbnail_path: None,
            hash: format!("hash-{id}"),
            created_at: format!("2026-06-10T00:00:{id:02}Z"),
            updated_at: format!("2026-06-10T00:00:{id:02}Z"),
        }
    }

    fn image_item(id: usize, image_path: String, thumbnail_path: String) -> ClipboardItem {
        ClipboardItem {
            id: format!("image-{id}"),
            item_type: ClipboardItemType::Image,
            preview: format!("image {id}"),
            text: None,
            image_path: Some(image_path),
            thumbnail_path: Some(thumbnail_path),
            hash: format!("image-hash-{id}"),
            created_at: format!("2026-06-10T00:00:{id:02}Z"),
            updated_at: format!("2026-06-10T00:00:{id:02}Z"),
        }
    }

    #[test]
    fn pushes_new_items_to_top() {
        let store = ClipboardHistoryStore::default();

        store.push(text_item(1)).expect("push 1");
        store.push(text_item(2)).expect("push 2");

        let snapshot = store.list().expect("list");
        assert_eq!(snapshot.items[0].id, "clip-2");
        assert_eq!(snapshot.items[1].id, "clip-1");
    }

    #[test]
    fn trims_default_capacity_to_twenty_items() {
        let store = ClipboardHistoryStore::default();

        for id in 0..21 {
            store.push(text_item(id)).expect("push");
        }

        let snapshot = store.list().expect("list");
        assert_eq!(snapshot.total, DEFAULT_HISTORY_CAPACITY);
        assert!(snapshot.items.iter().all(|item| item.id != "clip-0"));
    }

    #[test]
    fn trims_after_capacity_update() {
        let store = ClipboardHistoryStore::default();
        for id in 0..12 {
            store.push(text_item(id)).expect("push");
        }

        let snapshot = store
            .update_settings(UpdateClipboardHistorySettingsRequest {
                capacity: Some(10),
                persist_enabled: None,
            })
            .expect("settings");

        assert_eq!(snapshot.total, 10);
        assert_eq!(snapshot.settings.capacity, 10);
        assert!(snapshot.items.iter().all(|item| item.id != "clip-0"));
        assert!(snapshot.items.iter().all(|item| item.id != "clip-1"));
    }

    #[test]
    fn rejects_unsupported_capacity() {
        let store = ClipboardHistoryStore::default();

        assert!(matches!(
            store.update_settings(UpdateClipboardHistorySettingsRequest {
                capacity: Some(11),
                persist_enabled: None,
            }),
            Err(ClipboardHistoryError::InvalidCapacity)
        ));
    }

    #[test]
    fn deletes_single_item() {
        let store = ClipboardHistoryStore::default();
        store.push(text_item(1)).expect("push");

        let snapshot = store.delete("clip-1").expect("delete");

        assert_eq!(snapshot.total, 0);
        assert!(matches!(
            store.get("clip-1"),
            Err(ClipboardHistoryError::NotFound)
        ));
    }

    #[test]
    fn clears_all_items() {
        let store = ClipboardHistoryStore::default();
        store.push(text_item(1)).expect("push");
        store.push(text_item(2)).expect("push");

        assert_eq!(store.clear().expect("clear"), 2);
        assert_eq!(store.list().expect("list").total, 0);
    }

    #[test]
    fn deletes_image_files_when_item_is_removed() {
        let dir = tempfile::tempdir().expect("tempdir");
        let image_path = dir.path().join("image.png");
        let thumbnail_path = dir.path().join("thumb.png");
        std::fs::write(&image_path, [1, 2, 3]).expect("image");
        std::fs::write(&thumbnail_path, [4, 5, 6]).expect("thumb");
        let store = ClipboardHistoryStore::default();
        store
            .push(image_item(
                1,
                image_path.to_string_lossy().to_string(),
                thumbnail_path.to_string_lossy().to_string(),
            ))
            .expect("push");

        store.delete("image-1").expect("delete");

        assert!(!image_path.exists());
        assert!(!thumbnail_path.exists());
    }

    #[test]
    fn persists_history_when_initialized_with_database() {
        let dir = tempfile::tempdir().expect("tempdir");
        let database_path = dir.path().join("history.sqlite3");
        let store = ClipboardHistoryStore::default();
        store.initialize(database_path.clone()).expect("init");
        store.push(text_item(1)).expect("push");

        let restored = ClipboardHistoryStore::default();
        restored.initialize(database_path).expect("restore");

        let snapshot = restored.list().expect("list");
        assert_eq!(snapshot.total, 1);
        assert_eq!(snapshot.items[0].id, "clip-1");
    }

    #[test]
    fn does_not_persist_new_items_when_persistence_is_disabled() {
        let dir = tempfile::tempdir().expect("tempdir");
        let database_path = dir.path().join("history.sqlite3");
        let store = ClipboardHistoryStore::default();
        store.initialize(database_path.clone()).expect("init");
        store
            .update_settings(UpdateClipboardHistorySettingsRequest {
                capacity: None,
                persist_enabled: Some(false),
            })
            .expect("settings");
        store.push(text_item(1)).expect("push");
        assert_eq!(store.list().expect("list").total, 1);

        let restored = ClipboardHistoryStore::default();
        restored.initialize(database_path).expect("restore");

        assert_eq!(restored.list().expect("list").total, 0);
        assert!(!restored.list().expect("list").settings.persist_enabled);
    }
}
