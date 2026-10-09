use tauri::{AppHandle, Manager, State};

use crate::config::settings::SettingsManager;
use crate::db::connection::DbManager;
use crate::db::repo;
use crate::github::api::GithubClient;
use tauri_plugin_dialog::DialogExt;

/// Get current application settings from the database.
#[tauri::command]
pub fn get_settings(state: State<'_, DbManager>) -> Result<serde_json::Value, String> {
    let settings = SettingsManager::get_settings(&state).map_err(|e| e.to_string())?;
    serde_json::to_value(settings).map_err(|e| e.to_string())
}

/// Save application settings to the database.
#[tauri::command]
pub fn save_settings(
    state: State<'_, DbManager>,
    settings: serde_json::Value,
) -> Result<(), String> {
    let s: crate::models::types::AppSettings = serde_json::from_value(settings).map_err(|e| e.to_string())?;
    SettingsManager::save_settings(&state, &s).map_err(|e| e.to_string())
}

/// Open a native folder picker dialog.
#[tauri::command]
pub fn open_folder_dialog(app: AppHandle) -> Result<Option<String>, String> {
    // .file() is the correct builder for folder dialogs in Tauri's dialog API.
    // The .blocking_pick_folder() method turns the file dialog builder into a
    // folder picker, overriding the default file-selection behavior.
    let folder = app
        .dialog()
        .file()
        .set_title("Select Storage Folder")
        .blocking_pick_folder();
    Ok(folder.map(|p| p.to_string()))
}

/// Resolve the effective storage directory (configured path, or the app data
/// directory as fallback). Same resolution as the install/download logic.
#[tauri::command]
pub fn get_storage_path(
    app: AppHandle,
    state_db: State<'_, DbManager>,
) -> Result<String, String> {
    let fallback = app
        .path()
        .app_local_data_dir()
        .map_err(|e| e.to_string())?
        .to_string_lossy()
        .to_string();
    Ok(SettingsManager::get_storage_path(&state_db, &fallback))
}

/// Open the storage folder in the system file manager.
#[tauri::command]
pub fn open_storage_folder(
    app: AppHandle,
    state_db: State<'_, DbManager>,
) -> Result<(), String> {
    let path = get_storage_path(app, state_db)?;
    #[cfg(windows)]
    let opener = "explorer";
    #[cfg(target_os = "macos")]
    let opener = "open";
    #[cfg(all(unix, not(target_os = "macos")))]
    let opener = "xdg-open";
    let mut cmd = std::process::Command::new(opener);
    cmd.arg(&path);
    // explorer.exe exits with a non-zero status even on success: spawn and ignore.
    cmd
        .spawn()
        .map_err(|e| format!("Failed to open folder {}: {}", path, e))?;
    Ok(())
}

/// Save (or clear) the GitHub API token.
#[tauri::command]
pub fn save_github_token(
    state_db: State<'_, DbManager>,
    state_github: State<'_, GithubClient>,
    token: String,
) -> Result<(), String> {
    if token.is_empty() {
        let conn = state_db.lock_conn().map_err(|e| e.to_string())?;
        repo::delete_setting(&conn, "github_token").map_err(|e| e.to_string())?;
        state_github.set_token(None);
    } else {
        let conn = state_db.lock_conn().map_err(|e| e.to_string())?;
        repo::set_setting(&conn, "github_token", &token).map_err(|e| e.to_string())?;
        state_github.set_token(Some(token));
    }
    Ok(())
}

/// Check whether a GitHub token is configured.
#[tauri::command]
pub fn has_github_token(
    state_db: State<'_, DbManager>,
) -> Result<bool, String> {
    let conn = state_db.lock_conn().map_err(|e| e.to_string())?;
    Ok(repo::get_setting(&conn, "github_token").map_err(|e| e.to_string())?.is_some())
}

/// Delete the GitHub token from the database and clear it from the client.
#[tauri::command]
pub fn delete_github_token(
    state_db: State<'_, DbManager>,
    state_github: State<'_, GithubClient>,
) -> Result<(), String> {
    let conn = state_db.lock_conn().map_err(|e| e.to_string())?;
    repo::delete_setting(&conn, "github_token").map_err(|e| e.to_string())?;
    state_github.set_token(None);
    Ok(())
}

/// Get the application version from package info.
#[tauri::command]
pub fn get_app_version(app: AppHandle) -> String {
    app.package_info().version.to_string()
}


