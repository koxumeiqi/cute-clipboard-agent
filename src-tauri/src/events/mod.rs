use serde::Serialize;
use tauri::{AppHandle, Emitter};
use time::OffsetDateTime;

use crate::{
    clipboard::{ClipboardEventItem, ClipboardReadFailureReason},
    settings::PetPosition,
};

const CLIPBOARD_CHANGED: &str = "clipboard.changed";
const CLIPBOARD_CREATED: &str = "clipboard.created";
const CLIPBOARD_DUPLICATED: &str = "clipboard.duplicated";
const CLIPBOARD_READ_FAILED: &str = "clipboard.read_failed";
const CLIPBOARD_DELETED: &str = "clipboard.deleted";
const HISTORY_CLEARED: &str = "history.cleared";
const PET_DOUBLE_CLICKED: &str = "pet.double_clicked";
const PET_DRAG_ENDED: &str = "pet.drag_ended";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct PetEventPayload {
    at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct PetDragEndedPayload {
    at: String,
    position: PetPosition,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClipboardCreatedPayload {
    at: String,
    item: ClipboardEventItem,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClipboardDuplicatedPayload {
    at: String,
    hash: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClipboardReadFailedPayload {
    at: String,
    reason: ClipboardReadFailureReason,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClipboardDeletedPayload {
    at: String,
    id: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct HistoryClearedPayload {
    at: String,
    deleted_count: usize,
}

pub fn emit_pet_double_clicked(app: &AppHandle) {
    let _ = app.emit(PET_DOUBLE_CLICKED, PetEventPayload { at: now_rfc3339() });
}

pub fn emit_pet_drag_ended(app: &AppHandle, position: PetPosition) {
    let _ = app.emit(
        PET_DRAG_ENDED,
        PetDragEndedPayload {
            at: now_rfc3339(),
            position,
        },
    );
}

pub fn emit_clipboard_changed(app: &AppHandle) {
    let _ = app.emit(CLIPBOARD_CHANGED, PetEventPayload { at: now_rfc3339() });
}

pub fn emit_clipboard_created(app: &AppHandle, item: &crate::clipboard::ClipboardItem) {
    let _ = app.emit(
        CLIPBOARD_CREATED,
        ClipboardCreatedPayload {
            at: now_rfc3339(),
            item: ClipboardEventItem::from(item),
        },
    );
}

pub fn emit_clipboard_duplicated(app: &AppHandle, hash: String) {
    let _ = app.emit(
        CLIPBOARD_DUPLICATED,
        ClipboardDuplicatedPayload {
            at: now_rfc3339(),
            hash,
        },
    );
}

pub fn emit_clipboard_read_failed(app: &AppHandle, reason: ClipboardReadFailureReason) {
    let _ = app.emit(
        CLIPBOARD_READ_FAILED,
        ClipboardReadFailedPayload {
            at: now_rfc3339(),
            reason,
        },
    );
}

pub fn emit_clipboard_deleted(app: &AppHandle, id: String) {
    let _ = app.emit(
        CLIPBOARD_DELETED,
        ClipboardDeletedPayload {
            at: now_rfc3339(),
            id,
        },
    );
}

pub fn emit_history_cleared(app: &AppHandle, deleted_count: usize) {
    let _ = app.emit(
        HISTORY_CLEARED,
        HistoryClearedPayload {
            at: now_rfc3339(),
            deleted_count,
        },
    );
}

fn now_rfc3339() -> String {
    OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string())
}
