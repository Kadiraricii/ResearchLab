use rayon::prelude::*;
use serde::Serialize;
use tauri::State;

use crate::{
    analyzer::{self, AnalysisReport, Vulnerability},
    error::{AppError, Result},
    remediation,
    state::AppState,
    vercel::{deployments, env_vars, projects},
};

// ─── Response Types ───────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct ScanResult {
    pub project_id: String,
    pub analysis: AnalysisReport,
    pub env_var_count: usize,
    /// Number of env vars with `sensitive: true` (values are redacted by the API)
    pub sensitive_env_count: usize,
    pub deployment_count: usize,
}

#[derive(Serialize)]
pub struct ConfiguredStatus {
    pub configured: bool,
}

// ─── Token / Configuration ────────────────────────────────────────────────────

/// Set the Vercel API token. Must be called before any API commands.
#[tauri::command]
pub async fn set_vercel_token(
    token: String,
    state: State<'_, AppState>,
) -> Result<()> {
    if token.trim().is_empty() {
        return Err(AppError::Analysis("Token cannot be empty".to_string()));
    }
    state.set_token(token).await
}

/// Check whether an API token has been configured.
#[tauri::command]
pub async fn is_configured(state: State<'_, AppState>) -> Result<ConfiguredStatus> {
    Ok(ConfiguredStatus {
        configured: state.is_configured().await,
    })
}

// ─── Vercel API Data Commands ─────────────────────────────────────────────────

/// Fetch all Vercel projects accessible with the configured token.
#[tauri::command]
pub async fn get_vercel_projects(
    state: State<'_, AppState>,
) -> Result<Vec<crate::vercel::types::VercelProject>> {
    let client = state.get_client().await?;
    projects::list_projects(&client).await
}

/// Fetch recent deployments for a specific project.
#[tauri::command]
pub async fn get_vercel_deployments(
    project_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<crate::vercel::types::VercelDeployment>> {
    let client = state.get_client().await?;
    deployments::list_deployments(&client, &project_id).await
}

/// Fetch environment variable metadata for a project.
/// Values of sensitive vars are redacted by the Vercel API.
#[tauri::command]
pub async fn get_vercel_env_vars(
    project_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<crate::vercel::types::VercelEnvVar>> {
    let client = state.get_client().await?;
    env_vars::list_env_vars(&client, &project_id).await
}

// ─── Analysis Commands ────────────────────────────────────────────────────────

/// Analyze exposed environment variables from a KEY=VALUE formatted string.
/// Returns vulnerabilities found by the env_exposure analyzer.
#[tauri::command]
pub async fn analyze_env_exposure(env_content: String) -> Result<AnalysisReport> {
    // CPU-bound work runs in spawn_blocking to avoid blocking the tokio executor
    tokio::task::spawn_blocking(move || {
        let vulns = analyzer::env_exposure::analyze(&env_content);
        let score = analyzer::scoring::calculate_cvss_like_score(&vulns);
        AnalysisReport {
            vulnerabilities: vulns,
            score,
        }
    })
    .await
    .map_err(|e| AppError::Analysis(e.to_string()))
}

/// Check security headers in a vercel.json config string.
#[tauri::command]
pub async fn check_security_headers(vercel_json: String) -> Result<AnalysisReport> {
    tokio::task::spawn_blocking(move || {
        let vulns = analyzer::headers::analyze(&vercel_json);
        let score = analyzer::scoring::calculate_cvss_like_score(&vulns);
        AnalysisReport {
            vulnerabilities: vulns,
            score,
        }
    })
    .await
    .map_err(|e| AppError::Analysis(e.to_string()))
}

/// Run all static analyzers on provided config content.
/// All analyzers run in parallel via rayon.
#[tauri::command]
pub async fn scan_misconfigurations(
    vercel_json: String,
    next_config: String,
    env_content: String,
) -> Result<AnalysisReport> {
    run_parallel_analysis(vercel_json, next_config, env_content).await
}

/// Fetch live project data from the Vercel API and run a full parallel security scan.
///
/// Workflow:
/// 1. Fetch env vars + deployments concurrently (I/O bound — tokio::join!)
/// 2. Synthesize env content string from fetched vars
/// 3. Run all analyzers in parallel (CPU bound — rayon inside spawn_blocking)
#[tauri::command]
pub async fn run_full_scan(
    project_id: String,
    vercel_json: String,
    next_config: String,
    state: State<'_, AppState>,
) -> Result<ScanResult> {
    let client = state.get_client().await?;

    // Fetch API data concurrently — both are independent HTTP requests
    let (env_result, deploy_result) = tokio::join!(
        env_vars::list_env_vars(&client, &project_id),
        deployments::list_deployments(&client, &project_id),
    );

    let fetched_vars = env_result?;
    let fetched_deployments = deploy_result?;

    let env_var_count = fetched_vars.len();
    let sensitive_env_count = fetched_vars
        .iter()
        .filter(|v| v.sensitive == Some(true))
        .count();
    let deployment_count = fetched_deployments.len();

    // Synthesize env file content: include all keys, use empty value for redacted ones
    // so analyzers can still detect dangerous key names (e.g., NEXT_PUBLIC_SECRET_KEY=)
    let env_content = env_vars::to_env_file_content(&fetched_vars);

    let analysis = run_parallel_analysis(vercel_json, next_config, env_content).await?;

    Ok(ScanResult {
        project_id,
        analysis,
        env_var_count,
        sensitive_env_count,
        deployment_count,
    })
}

/// Get remediation recommendations based on config content.
#[tauri::command]
pub async fn get_remediation(
    vercel_json: String,
    next_config: String,
    env_content: String,
) -> Result<remediation::RemediationReport> {
    tokio::task::spawn_blocking(move || {
        let analysis = analyzer::run_all_analyzers(&vercel_json, &next_config, &env_content);
        Ok::<remediation::RemediationReport, AppError>(remediation::generate_report(&analysis))
    })
    .await
    .map_err(|e| AppError::Analysis(e.to_string()))?
}

/// Export an AnalysisReport as a pretty-printed JSON string.
#[tauri::command]
pub async fn export_report(report: AnalysisReport) -> Result<String> {
    serde_json::to_string_pretty(&report).map_err(AppError::Json)
}

// ─── Internal: Parallel Analysis Engine ──────────────────────────────────────

/// Run all 12 security analyzers in parallel using rayon.
///
/// Rayon uses its own thread pool (separate from tokio's), so we wrap the
/// rayon work in `spawn_blocking` to avoid starving the async executor.
async fn run_parallel_analysis(
    vercel_json: String,
    next_config: String,
    env_content: String,
) -> Result<AnalysisReport> {
    tokio::task::spawn_blocking(move || {
        // Each closure captures its own clone of the input strings.
        // The closures are Fn() + Send + Sync, which rayon requires for par_iter.
        type AnalyzerFn = Box<dyn Fn() -> Vec<Vulnerability> + Send + Sync>;

        let analyzers: Vec<AnalyzerFn> = vec![
            Box::new({
                let e = env_content.clone();
                move || analyzer::env_exposure::analyze(&e)
            }),
            Box::new({
                let v = vercel_json.clone();
                move || analyzer::headers::analyze(&v)
            }),
            Box::new({
                let v = vercel_json.clone();
                let n = next_config.clone();
                move || analyzer::cors::analyze(&v, &n)
            }),
            Box::new({
                let n = next_config.clone();
                move || analyzer::source_maps::analyze(&n)
            }),
            Box::new({
                let v = vercel_json.clone();
                move || analyzer::redirects::analyze(&v)
            }),
            Box::new({
                let v = vercel_json.clone();
                move || analyzer::rewrites::analyze(&v)
            }),
            Box::new({
                let v = vercel_json.clone();
                move || analyzer::dns::analyze(&v)
            }),
            Box::new({
                let v = vercel_json.clone();
                move || analyzer::ssl::analyze(&v)
            }),
            Box::new({
                let v = vercel_json.clone();
                move || analyzer::serverless::analyze(&v)
            }),
            Box::new({
                let v = vercel_json.clone();
                move || analyzer::middleware::analyze(&v)
            }),
            Box::new({
                let v = vercel_json.clone();
                move || analyzer::preview_auth::analyze(&v)
            }),
            Box::new({
                let v = vercel_json.clone();
                move || analyzer::build_logs::analyze(&v)
            }),
        ];

        // par_iter: each analyzer runs on a rayon thread pool worker concurrently.
        // flat_map flattens Vec<Vulnerability> from each analyzer into a single Vec.
        let all_vulns: Vec<Vulnerability> = analyzers.par_iter().flat_map(|f| f()).collect();

        let score = analyzer::scoring::calculate_cvss_like_score(&all_vulns);
        AnalysisReport {
            vulnerabilities: all_vulns,
            score,
        }
    })
    .await
    .map_err(|e| AppError::Analysis(e.to_string()))
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn parallel_analysis_returns_report_for_empty_input() {
        let report = run_parallel_analysis(
            String::new(),
            String::new(),
            String::new(),
        )
        .await
        .expect("analysis should not fail on empty input");

        // With empty config, headers + CSP analyzers should still flag issues
        assert!(report.score >= 0.0);
        assert!(report.score <= 10.0);
    }

    #[tokio::test]
    async fn parallel_analysis_detects_env_exposure() {
        // env_content is the THIRD argument (vercel_json, next_config, env_content)
        let env_content = "NEXT_PUBLIC_SECRET_TOKEN=sk-1234567890".to_string();
        let report = run_parallel_analysis(String::new(), String::new(), env_content)
            .await
            .expect("analysis should succeed");

        let found = report.vulnerabilities.iter().any(|v| v.id == "ENV_01");
        assert!(found, "should detect NEXT_PUBLIC_ sensitive env exposure");
    }
}
