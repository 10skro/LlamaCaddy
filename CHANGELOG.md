# Changelog

## Unreleased

### New
- **One model folder is enough**: the model and mmproj pickers now scan the model folder recursively (up to 3 levels) and group files by sub-folder, so same-named mmproj files from different model folders are easy to tell apart. Recommended layout: one sub-folder per model, with its mmproj inside — selecting a model then preselects the matching mmproj automatically.
- **Settings simplified**: the separate "Mmproj Folder" field is gone — a single "Model Folder" covers everything. A previously configured mmproj folder keeps working silently as an override.

### Fixed
- **Updates now relaunch the app automatically** after `npm install -g llamacaddy@latest` (previously the terminal asked to close and restart manually).
- **Changelog shown after an update**: the changelog of the installed version now opens once on next startup (the mechanism was saved but never displayed).

## v0.9.2 — 2026-10-09

### Internal
- **Dead dependency cleanup**: removed never-imported npm packages (Radix avatar/scroll-area/tabs/tooltip, react-table, react-color types, JS wrappers of Tauri plugins) and unused backend Tauri plugins (shell, fs, http) with their capability permissions. The dialog plugin is kept (native folder picker).
- Disabled `withGlobalTauri` (unused: the UI imports the Tauri API via npm).
- READMEs aligned with npm distribution: `npx llamacaddy`, updating via npm, link to the CHANGELOG.
- Versioning fix: the root `package.json` was stuck at 0.8.0 — aligned with the rest.

## v0.9.0 — 2026-10-09

### New
- **New name: LlamaCaddy** — the application is now called LlamaCaddy (window, UI, installer, GitHub repo `10skro/LlamaCaddy`). Your data is migrated automatically on first launch (`%LOCALAPPDATA%\llama-manager` → `%LOCALAPPDATA%\llamacaddy`): installed versions, configurations and favorites are kept.

### Internal
- npm dependencies updated (minor); removed the unused legacy `tauri` package (the code uses `@tauri-apps/api`).
- Rust dependencies updated in majors: rusqlite 0.40 (with SQLite i64/u64 type fixes), reqwest 0.13, thiserror 2, zip 9 (`mangled_name` API).
- ESLint stays on v9: moving to v10 (strict react-hooks rules) is deferred to a dedicated branch.
- Remaining npm vulnerabilities (vitest, tailwindcss, react-router, braces): only fixable by major updates, planned separately.

### Coming soon
- ~~Distribution via npm (`llamacaddy`) instead of the `.exe` installer + custom updater + `latest.json` + GitHub Pages (like WhisperPro).~~ → done below.

### Distribution
- **Switch to npm**: the app installs and updates with `npm install -g llamacaddy` (like WhisperPro). Removed the Tauri updater, the `docs/latest.json` file and the GitHub Pages workflow. The "Update available" button now opens a terminal running `npm install -g llamacaddy@latest`. Automatic signed publishing (npm provenance) on pushing a `v*` tag, release notes taken from the CHANGELOG.

## v0.8.0 and earlier

Notes generated automatically from commits — see the corresponding GitHub releases.
