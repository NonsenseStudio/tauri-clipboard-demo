
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    // if let Some(splashscreen) = window.get_window("splashscreen") {
    //     std::thread::sleep(std::time::Duration::from_secs(10));
    //     splashscreen.close().unwrap();
    // }

    window.get_window("main").unwrap().show().unwrap();
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![close_splashscreen])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}




