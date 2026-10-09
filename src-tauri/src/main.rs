#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    llamacaddy_lib::run_tauri_app();
}
