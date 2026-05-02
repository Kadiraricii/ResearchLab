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
fn generate_remediation_report(vercel_json: String, next_config: String, env_content: String) -> remediation::RemediationReport {
    let analysis = analyzer::run_all_analyzers(&vercel_json, &next_config, &env_content);
    remediation::generate_report(&analysis)
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
            generate_remediation_report
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
