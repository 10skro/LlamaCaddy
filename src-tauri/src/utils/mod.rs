use std::path::Path;

use crate::models::types::AppError;

/// Mask absolute paths in error messages to prevent information disclosure.
/// Returns a shortened version showing only the last directory and filename.
pub fn mask_path(path: &str) -> String {
    let p = Path::new(path);
    if let Some(parent) = p.parent() {
        if let (Some(_parent_name), Some(file_name)) = (parent.file_name(), p.file_name()) {
            return format!(".../{}", file_name.to_string_lossy());
        }
    }
    // If we can't extract components, try masking everything except the last segment
    if let Some(file_name) = p.file_name() {
        return format!(".../{}", file_name.to_string_lossy());
    }
    path.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_path_with_file() {
        let result = mask_path("/some/long/path/to/file.txt");
        assert_eq!(result, ".../file.txt");
    }

    #[test]
    fn test_mask_path_simple() {
        let result = mask_path("simple");
        assert_eq!(result, ".../simple");
    }

    #[test]
    fn test_mask_path_directory() {
        let result = mask_path("/path/to/dir/");
        assert_eq!(result, ".../dir");
    }
}

/// One-time migration from the legacy `%LOCALAPPDATA%\llama-manager` folder to
/// the new `%LOCALAPPDATA%\llamacaddy` folder. Moves the whole tree (database,
/// versions, downloads, config, logs) when the new folder does not exist yet.
pub fn migrate_legacy_data_dir(new_base: &Path) {
    if new_base.exists() {
        return;
    }
    let Some(local) = std::env::var_os("LOCALAPPDATA") else {
        return;
    };
    let legacy = std::path::Path::new(&local).join("llama-manager");
    if !legacy.is_dir() {
        return;
    }
    if let Some(parent) = new_base.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    // rename is atomic on the same volume; fall back to a copy if it fails
    // (e.g. files locked by a running instance).
    if std::fs::rename(&legacy, new_base).is_ok() {
        return;
    }
    fn copy_tree(from: &Path, to: &Path) -> std::io::Result<()> {
        std::fs::create_dir_all(to)?;
        for entry in std::fs::read_dir(from)? {
            let entry = entry?;
            let ft = entry.file_type()?;
            let src = entry.path();
            let dst = to.join(entry.file_name());
            if ft.is_dir() {
                copy_tree(&src, &dst)?;
            } else {
                std::fs::copy(&src, &dst)?;
            }
        }
        Ok(())
    }
    let _ = copy_tree(&legacy, new_base);
}

/// Create required application directories under the app data folder.
pub fn setup_directories(base: &Path) -> Result<(), AppError> {
    let dirs = ["versions", "database", "downloads", "logs", "config"];
    for dir in &dirs {
        let path = base.join(dir);
        std::fs::create_dir_all(&path)?;
    }
    Ok(())
}
