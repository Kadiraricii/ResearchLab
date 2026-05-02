pub mod summary;
pub mod analyzer;
pub mod remediation;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn run_analysis(vercel_json: String, next_config: String, env_content: String) -> analyzer::AnalysisReport {
    analyzer::run_all_analyzers(&vercel_json, &next_config, &env_content)
}

#[tauri::command]
fn get_remediation_templates() -> (String, String) {
    remediation::templates::generate_secure_templates()
}

#[tauri::command]
fn generate_recommendations() -> Vec<String> {
    remediation::recommendations::get_recommendations()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            summary::parse_vercel_config,
            summary::parse_next_config,
            summary::get_platform_defaults,
            run_analysis,
            get_remediation_templates,
            generate_recommendations
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
