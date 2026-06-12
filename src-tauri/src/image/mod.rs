use std::{fs, path::Path};

#[derive(Debug, thiserror::Error)]
pub enum ImageStorageError {
    #[error("image_cleanup_failed")]
    CleanupFailed,
}

pub fn delete_image_files(paths: &[Option<String>]) -> Result<(), ImageStorageError> {
    for path in paths.iter().filter_map(|path| path.as_deref()) {
        if path.starts_with("pending://") {
            continue;
        }
        let path = Path::new(path);
        if path.exists() {
            fs::remove_file(path).map_err(|_| ImageStorageError::CleanupFailed)?;
        }
    }
    Ok(())
}
