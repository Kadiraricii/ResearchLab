pub mod analyzer;
pub mod commands;
pub mod error;
pub mod remediation;
pub mod state;
pub mod summary;
pub mod vercel;

use state::AppState;

// ─── Legacy Sync Commands (kept for backward compatibility) ───────────────────

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Synchronous full analysis — used by the existing frontend before Phase 6 async migration.
#[tauri::command]
fn run_analysis(
    vercel_json: String,
    next_config: String,
    env_content: String,
) -> analyzer::AnalysisReport {
    analyzer::run_all_analyzers(&vercel_json, &next_config, &env_content)
}

#[tauri::command]
fn generate_remediation_report(
    vercel_json: String,
    next_config: String,
    env_content: String,
) -> remediation::RemediationReport {
    let analysis = analyzer::run_all_analyzers(&vercel_json, &next_config, &env_content);
    remediation::generate_report(&analysis)
}

// ─── Application Entry Point ──────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging. In tests, use env_logger::try_init() instead.
    let _ = env_logger::try_init();

    // Load .env file if present; silently ignore if missing.
    dotenvy::dotenv().ok();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // .manage() MUST be called before .invoke_handler()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            // Legacy commands
            greet,
            summary::parse_vercel_config,
            summary::parse_next_config,
            summary::get_platform_defaults,
            run_analysis,
            generate_remediation_report,
            // Phase 5: Token + config
            commands::set_vercel_token,
            commands::is_configured,
            // Phase 5: Vercel API data
            commands::get_vercel_projects,
            commands::get_vercel_deployments,
            commands::get_vercel_env_vars,
            // Phase 5: Analysis
            commands::analyze_env_exposure,
            commands::check_security_headers,
            commands::scan_misconfigurations,
            commands::run_full_scan,
            commands::get_remediation,
            commands::export_report,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
