pub mod analyzer;
pub mod commands;
pub mod db;
pub mod error;
pub mod remediation;
pub mod state;
pub mod summary;
pub mod vercel;

use tauri::Manager;
use state::AppState;
use db::{Database, crud, models};
use error::AppError;

// ─── Legacy Sync Commands ─────────────────────────────────────────────────────

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

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

// ─── Phase 7 Database Commands ────────────────────────────────────────────────

/// Persist a completed scan (with its findings) to the local database.
#[tauri::command]
fn save_scan(
    state: tauri::State<'_, AppState>,
    scan: models::Scan,
    findings: Vec<models::Finding>,
) -> Result<(), AppError> {
    let conn = state.db.conn.lock().map_err(|_| AppError::Analysis("DB mutex poisoned".into()))?;
    crud::insert_scan(&conn, &scan)?;
    crud::insert_findings(&conn, &findings)?;
    Ok(())
}

/// Return all scans ordered by creation date (most recent first).
#[tauri::command]
fn list_scans(state: tauri::State<'_, AppState>) -> Result<Vec<models::Scan>, AppError> {
    let conn = state.db.conn.lock().map_err(|_| AppError::Analysis("DB mutex poisoned".into()))?;
    crud::list_scans(&conn)
}

/// Return all findings that belong to a given scan.
#[tauri::command]
fn get_findings(
    state: tauri::State<'_, AppState>,
    scan_id: String,
) -> Result<Vec<models::Finding>, AppError> {
    let conn = state.db.conn.lock().map_err(|_| AppError::Analysis("DB mutex poisoned".into()))?;
    crud::get_findings_for_scan(&conn, &scan_id)
}

/// Delete a scan and its findings (ON DELETE CASCADE handles findings).
#[tauri::command]
fn delete_scan_cmd(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    let conn = state.db.conn.lock().map_err(|_| AppError::Analysis("DB mutex poisoned".into()))?;
    crud::delete_scan(&conn, &id)
}

/// Persist a generated report to the database.
#[tauri::command]
fn save_report(
    state: tauri::State<'_, AppState>,
    report: models::Report,
) -> Result<(), AppError> {
    let conn = state.db.conn.lock().map_err(|_| AppError::Analysis("DB mutex poisoned".into()))?;
    crud::insert_report(&conn, &report)
}

/// Return all reports ordered by creation date.
#[tauri::command]
fn list_reports(state: tauri::State<'_, AppState>) -> Result<Vec<models::Report>, AppError> {
    let conn = state.db.conn.lock().map_err(|_| AppError::Analysis("DB mutex poisoned".into()))?;
    crud::list_reports(&conn)
}

/// Persist a tracked project.
#[tauri::command]
fn add_project(
    state: tauri::State<'_, AppState>,
    project: models::Project,
) -> Result<(), AppError> {
    let conn = state.db.conn.lock().map_err(|_| AppError::Analysis("DB mutex poisoned".into()))?;
    crud::insert_project(&conn, &project)
}

/// Return all tracked projects.
#[tauri::command]
fn list_projects(state: tauri::State<'_, AppState>) -> Result<Vec<models::Project>, AppError> {
    let conn = state.db.conn.lock().map_err(|_| AppError::Analysis("DB mutex poisoned".into()))?;
    crud::list_projects(&conn)
}

// ─── Application Entry Point ──────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = env_logger::try_init();
    dotenvy::dotenv().ok();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Resolve the per-app data directory and open (or create) the DB there.
            // Falls back to in-memory if the path cannot be determined.
            let db = match app.path().app_data_dir() {
                Ok(mut dir) => {
                    std::fs::create_dir_all(&dir).ok();
                    dir.push("analyzer.db");
                    Database::open(&dir).unwrap_or_else(|e| {
                        log::error!("Failed to open DB at {dir:?}: {e}; falling back to in-memory");
                        Database::open_in_memory().expect("in-memory DB must open")
                    })
                }
                Err(e) => {
                    log::warn!("Could not resolve app data dir: {e}; using in-memory DB");
                    Database::open_in_memory().expect("in-memory DB must open")
                }
            };
            app.manage(AppState::new(db));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Legacy
            greet,
            summary::parse_vercel_config,
            summary::parse_next_config,
            summary::get_platform_defaults,
            run_analysis,
            generate_remediation_report,
            // Phase 5: Token + Vercel API
            commands::set_vercel_token,
            commands::is_configured,
            commands::get_vercel_projects,
            commands::get_vercel_deployments,
            commands::get_vercel_env_vars,
            commands::analyze_env_exposure,
            commands::check_security_headers,
            commands::scan_misconfigurations,
            commands::run_full_scan,
            commands::get_remediation,
            commands::export_report,
            // Phase 7: Database
            save_scan,
            list_scans,
            get_findings,
            delete_scan_cmd,
            save_report,
            list_reports,
            add_project,
            list_projects,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
