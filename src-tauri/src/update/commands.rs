use std::process::Command;

use tauri::Manager;

use crate::db::connection::DbManager;
use crate::db::repo;
use crate::terminal::manager::TerminalManager;

/// npm package this application is distributed as.
const NPM_PACKAGE: &str = "llamacaddy";
const NPM_REGISTRY_URL: &str = "https://registry.npmjs.org/llamacaddy/latest";
const GITHUB_RELEASE_URL: &str = "https://api.github.com/repos/10skro/LlamaCaddy/releases/tags/";

/// Parse "x.y.z" into a comparable tuple. Non-numeric parts become 0.
fn parse_version(v: &str) -> (u64, u64, u64) {
    let mut parts = [0u64; 3];
    for (i, p) in v
        .trim()
        .trim_start_matches('v')
        .split('.')
        .take(3)
        .enumerate()
    {
        parts[i] = p.parse().unwrap_or(0);
    }
    (parts[0], parts[1], parts[2])
}

#[cfg(test)]
mod tests {
    use super::parse_version;

    #[test]
    fn test_parse_version_basic() {
        assert_eq!(parse_version("0.9.0"), (0, 9, 0));
        assert_eq!(parse_version("v1.2.3"), (1, 2, 3));
    }

    #[test]
    fn test_parse_version_malformed() {
        assert_eq!(parse_version("1.x.3"), (1, 0, 3));
        assert_eq!(parse_version("2"), (2, 0, 0));
    }
}

/// Check if a new application version is published on npm.
/// Returns a JSON object with `available`, `version`, `date`, and `body` fields
/// (body = GitHub release notes for that version, when available).
#[tauri::command]
pub async fn check_app_update(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let current = app.package_info().version.to_string();

    let client = reqwest::Client::new();
    let resp = client
        .get(NPM_REGISTRY_URL)
        .header("User-Agent", "llamacaddy-updater")
        .send()
        .await
        .map_err(|e| format!("Failed to check for updates: {}", e))?;
    let latest: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Failed to read npm registry response: {}", e))?;
    let latest_version = latest["version"]
        .as_str()
        .ok_or("Invalid npm registry response")?
        .to_string();

    if parse_version(&latest_version) <= parse_version(&current) {
        log::info!(
            "No update available (current {}, npm {})",
            current,
            latest_version
        );
        return Ok(serde_json::json!({
            "available": false,
            "version": null,
            "date": null,
            "body": null,
        }));
    }

    // Fetch GitHub release notes for the changelog (best effort).
    let mut date = None;
    let mut body = None;
    match client
        .get(format!("{}v{}", GITHUB_RELEASE_URL, latest_version))
        .header("User-Agent", "llamacaddy-updater")
        .send()
        .await
    {
        Ok(r) if r.status().is_success() => {
            if let Ok(release) = r.json::<serde_json::Value>().await {
                date = release["published_at"].as_str().map(|s| s.to_string());
                body = release["body"].as_str().map(|s| s.to_string());
            }
        }
        _ => {}
    }

    log::info!("Update available: {} -> {}", current, latest_version);
    Ok(serde_json::json!({
        "available": true,
        "version": latest_version,
        "date": date,
        "body": body,
    }))
}

/// Install the application update via npm: opens a visible terminal running
/// `npm install -g llamacaddy@latest`, then exits the app so Windows releases
/// the lock on the running executable. Persists changelog data to the database
/// before exiting so the changelog is shown on next startup.
#[tauri::command]
pub async fn install_app_update(
    app: tauri::AppHandle,
    changelog_version: Option<String>,
    changelog_body: Option<String>,
) -> Result<(), String> {
    log::info!("[UPDATE] install_app_update: starting npm-based update");

    // Persist changelog to database BEFORE exiting (eliminates race condition)
    if let (Some(ref version), Some(ref body)) = (&changelog_version, &changelog_body) {
        let db = app.state::<DbManager>();
        {
            let conn = db
                .lock_conn()
                .map_err(|e| format!("Failed to lock database: {}", e))?;
            repo::set_setting(&conn, "pending_changelog_version", version)
                .map_err(|e| format!("Failed to save changelog version: {}", e))?;
            repo::set_setting(&conn, "pending_changelog_body", body)
                .map_err(|e| format!("Failed to save changelog body: {}", e))?;
        }
        log::info!("[UPDATE] persisting changelog for version {}", version);

        // Force WAL checkpoint to guarantee data is flushed to disk before exit
        if let Err(e) = db.checkpoint() {
            log::warn!(
                "[UPDATE] checkpoint failed: {} (data may not be persisted)",
                e
            );
        }
    }

    // Kill all terminal sessions before updating (safety net)
    let terminal = app.state::<TerminalManager>();
    terminal.kill_all();

    // Open a visible terminal that: waits for this app to exit (releasing the
    // lock on the executable), runs the npm update, then relaunches the app
    // and closes itself. On failure the terminal stays open to show the error.
    let script = format!(
        "timeout /t 3 /nobreak >nul & npm install -g {NPM_PACKAGE}@latest && (echo. && echo Mise a jour terminee, relance de {NPM_PACKAGE}... && start \"\" {NPM_PACKAGE}) || (echo. && echo Echec de la mise a jour npm. && pause)"
    );
    let mut cmd = Command::new("cmd");
    cmd.args(["/C", "start", "cmd", "/C", &script]);
    cmd.spawn()
        .map_err(|e| format!("Update launch failed: {}", e))?;

    log::info!("[UPDATE] npm update requested from UI, exiting");
    app.exit(0);
    Ok(())
}
