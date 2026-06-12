use std::path::PathBuf;

use tauri::{AppHandle, Manager};

pub fn app_database_path(app: &AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .unwrap_or_else(|_| std::env::temp_dir().join("cute-clipboard-agent"))
        .join("cute-clipboard-agent.sqlite3")
}
