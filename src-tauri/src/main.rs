#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> Result<String, String> {
    // format!("Hello, {}! You've been greeted from Rust!", name)
    if name.is_empty() {
        Err(format!("An error has occurred: {}", "name is empty"))
    } else {
        Ok(format!("Hello, {}! You've been greeted from Rust!", name))
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
