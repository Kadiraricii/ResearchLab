// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod summary;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            summary::parse_vercel_config,
            summary::parse_next_config,
            summary::get_platform_defaults
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
