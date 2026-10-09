# Changelog

## Unreleased

## v0.9.2 — 2026-10-09

### Interne
- **Nettoyage des dépendances mortes** : suppression de paquets npm jamais importés (Radix avatar/scroll-area/tabs/tooltip, react-table, types react-color, wrappers JS des plugins Tauri) et des plugins Tauri inutilisés côté backend (shell, fs, http) avec leurs permissions de capacités. Le plugin dialog est conservé (sélecteur de dossier natif).
- Désactivation de `withGlobalTauri` (inutilisé : l'interface importe l'API Tauri par npm).
- READMEs alignés sur la distribution npm : `npx llamacaddy`, mise à jour via npm, lien vers le CHANGELOG.
- Correction du versionnement : le `package.json` racine était resté à 0.8.0 — aligné avec le reste.

## v0.9.0 — 2026-10-09

### Nouveautés
- **Nouveau nom : LlamaCaddy** — l'application s'appelle désormais LlamaCaddy (fenêtre, interface, installeur, dépôt GitHub `10skro/LlamaCaddy`). Vos données sont migrées automatiquement au premier lancement (`%LOCALAPPDATA%\llama-manager` → `%LOCALAPPDATA%\llamacaddy`) : versions installées, configurations et favoris sont conservés.

### Interne
- Dépendances npm mises à jour (mineures) ; suppression du paquet legacy `tauri` inutilisé (le code passe par `@tauri-apps/api`).
- Dépendances Rust mises à jour en majeures : rusqlite 0.40 (avec correction des types i64/u64 SQLite), reqwest 0.13, thiserror 2, zip 9 (API `mangled_name`).
- ESLint reste en v9 : le passage à v10 (règles strictes react-hooks) est reporté à une branche dédiée.
- Vulnérabilités npm restantes (vitest, tailwindcss, react-router, braces) : corrigées seulement par des mises à jour majeures, planifiées séparément.

### À venir
- ~~Distribution par npm (`llamacaddy`) à la place de l'installeur `.exe` + updater maison + `latest.json` + GitHub Pages (comme WhisperPro).~~ → fait ci-dessous.

### Distribution
- **Passage à npm** : l'application s'installe et se met à jour avec `npm install -g llamacaddy` (comme WhisperPro). Suppression de l'updater Tauri, du fichier `docs/latest.json` et du workflow GitHub Pages. Le bouton « Update available » de l'application ouvre maintenant un terminal `npm install -g llamacaddy@latest`. Publication automatique signée (npm provenance) au push d'un tag `v*`, notes de release reprises du CHANGELOG.

## v0.8.0 et antérieurs

Notes générées automatiquement à partir des commits — voir les releases GitHub correspondantes.
