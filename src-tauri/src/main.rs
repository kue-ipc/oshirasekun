// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use encoding_rs;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn sjis_to_utf8(src: Vec<u8>) -> String {
    let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&src);
    res.into_owned()
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![sjis_to_utf8])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
