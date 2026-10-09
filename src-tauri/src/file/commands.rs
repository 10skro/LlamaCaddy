use crate::models::types::ModelFile;
use std::path::Path;

/// Maximum sub-directory depth explored by recursive scans.
const MAX_SCAN_DEPTH: usize = 3;
/// Safety cap so a pathological folder tree can never hang the UI.
const MAX_SCANNED_FILES: usize = 5000;

/// Scan a folder for files matching given extensions.
/// When `recursive` is false, only files directly in the folder are returned
/// (legacy behavior). When true, sub-directories are explored up to
/// `MAX_SCAN_DEPTH` levels deep.
/// Each result carries `rel_dir`: the sub-directory (relative to the scanned
/// root) containing the file — empty when the file sits in the root itself.
/// Returns files sorted by (rel_dir, name), case-insensitive.
/// If extensions is empty, all files are included.
pub fn scan_files(
    folder_path: &str,
    extensions: &[&str],
    recursive: bool,
) -> Result<Vec<ModelFile>, String> {
    let root = Path::new(folder_path);

    if !root.exists() {
        return Err(format!("Folder does not exist: {}", folder_path));
    }

    if !root.is_dir() {
        return Err(format!("Path is not a directory: {}", folder_path));
    }

    let mut files: Vec<ModelFile> = Vec::new();
    // Stack of (directory, relative path from root, depth).
    let mut stack: Vec<(std::path::PathBuf, String, usize)> =
        vec![(root.to_path_buf(), String::new(), 0)];

    while let Some((dir, rel_dir, depth)) = stack.pop() {
        let entries = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => continue, // unreadable sub-folder: skip, don't fail the whole scan
        };

        for entry in entries.flatten() {
            let file_path = entry.path();

            if file_path.is_dir() {
                if recursive && depth < MAX_SCAN_DEPTH {
                    let child_rel = if rel_dir.is_empty() {
                        entry.file_name().to_string_lossy().to_string()
                    } else {
                        format!("{}\\{}", rel_dir, entry.file_name().to_string_lossy())
                    };
                    stack.push((file_path, child_rel, depth + 1));
                }
                continue;
            }

            if !file_path.is_file() {
                continue;
            }

            // Filter by extensions (empty = all files)
            if !extensions.is_empty() {
                let file_ext = file_path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_lowercase();
                if !extensions.iter().any(|e| e.eq_ignore_ascii_case(&file_ext)) {
                    continue;
                }
            }

            let Ok(metadata) = entry.metadata() else {
                continue;
            };
            let name = file_path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();

            files.push(ModelFile {
                path: file_path.to_string_lossy().to_string(),
                name,
                size: metadata.len(),
                rel_dir: rel_dir.clone(),
            });

            if files.len() >= MAX_SCANNED_FILES {
                break;
            }
        }

        if files.len() >= MAX_SCANNED_FILES {
            break;
        }
    }

    // Group by folder first, then by file name, for a stable readable order.
    files.sort_by(|a, b| {
        a.rel_dir
            .to_lowercase()
            .cmp(&b.rel_dir.to_lowercase())
            .then(a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });

    Ok(files)
}

/// Scan for model files (.gguf, .safetensors, etc.), including sub-folders.
/// extensions: comma-separated list, e.g. "gguf,safetensors" or "" for all
#[tauri::command]
pub fn scan_model_files(folder_path: String, extensions: String) -> Result<Vec<ModelFile>, String> {
    let exts: Vec<&str> = if extensions.is_empty() {
        Vec::new()
    } else {
        extensions.split(',').collect()
    };
    scan_files(&folder_path, &exts, true)
}

/// Scan for mmproj files (.gguf, .safetensors, .mmproj, etc.), including sub-folders.
/// extensions: comma-separated list, e.g. "gguf,safetensors" or "" for all
#[tauri::command]
pub fn scan_mmproj_files(folder_path: String, extensions: String) -> Result<Vec<ModelFile>, String> {
    let exts: Vec<&str> = if extensions.is_empty() {
        Vec::new()
    } else {
        extensions.split(',').collect()
    };
    scan_files(&folder_path, &exts, true)
}

/// Validate that a folder path exists and is accessible as a directory.
#[tauri::command]
pub fn validate_folder(path: String) -> Result<bool, String> {
    let p = Path::new(&path);
    if !p.exists() {
        return Err(format!("Folder does not exist: {}", path));
    }
    if !p.is_dir() {
        return Err(format!("Path is not a directory: {}", path));
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::{scan_files, MAX_SCAN_DEPTH};

    fn fixture_root(tag: &str) -> std::path::PathBuf {
        let p = std::env::temp_dir().join(format!("llamacaddy-scan-{}", tag));
        let _ = std::fs::remove_dir_all(&p);
        p
    }

    #[test]
    fn test_recursive_scan_groups_by_subfolder() {
        let root = fixture_root("grouped");
        std::fs::create_dir_all(root.join("qwen3-vl")).unwrap();
        std::fs::create_dir_all(root.join("gemma-4")).unwrap();
        std::fs::write(root.join("root-model.gguf"), "x").unwrap();
        std::fs::write(root.join("qwen3-vl/mmproj-F16.gguf"), "x").unwrap();
        std::fs::write(root.join("gemma-4/mmproj-F16.gguf"), "x").unwrap();

        let files = scan_files(root.to_str().unwrap(), &["gguf"], true).unwrap();
        assert_eq!(files.len(), 3);

        let mmprojs: Vec<&super::ModelFile> = files
            .iter()
            .filter(|f| f.name == "mmproj-F16.gguf")
            .collect();
        assert_eq!(mmprojs.len(), 2);
        // Same file name, distinct rel_dir: the UI can tell them apart.
        let dirs: Vec<&str> = mmprojs.iter().map(|f| f.rel_dir.as_str()).collect();
        assert!(dirs.contains(&"gemma-4") && dirs.contains(&"qwen3-vl"));

        // Root-level file has empty rel_dir.
        let root_file = files.iter().find(|f| f.name == "root-model.gguf").unwrap();
        assert_eq!(root_file.rel_dir, "");

        // Sorted by rel_dir then name.
        assert_eq!(files[0].rel_dir, "");
        assert_eq!(files[1].rel_dir, "gemma-4");
        assert_eq!(files[2].rel_dir, "qwen3-vl");

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn test_non_recursive_skips_subfolders() {
        let root = fixture_root("flat");
        std::fs::create_dir_all(root.join("sub")).unwrap();
        std::fs::write(root.join("a.gguf"), "x").unwrap();
        std::fs::write(root.join("sub/b.gguf"), "x").unwrap();

        let files = scan_files(root.to_str().unwrap(), &["gguf"], false).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "a.gguf");

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn test_depth_is_bounded() {
        let root = fixture_root("depth");
        let deep =
            (0..=MAX_SCAN_DEPTH + 2).fold(root.clone(), |acc, i| acc.join(format!("l{}", i)));
        std::fs::create_dir_all(&deep).unwrap();
        std::fs::write(deep.join("deep.gguf"), "x").unwrap();

        let files = scan_files(root.to_str().unwrap(), &["gguf"], true).unwrap();
        assert!(files.iter().all(|f| f.name != "deep.gguf"));

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn test_extension_filter_case_insensitive() {
        let root = fixture_root("ext");
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("model.GGUF"), "x").unwrap();
        std::fs::write(root.join("notes.txt"), "x").unwrap();

        let files = scan_files(root.to_str().unwrap(), &["gguf"], true).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "model.GGUF");

        std::fs::remove_dir_all(&root).ok();
    }
}
