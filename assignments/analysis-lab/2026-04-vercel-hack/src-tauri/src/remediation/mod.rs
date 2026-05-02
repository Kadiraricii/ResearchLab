use crate::analyzer::AnalysisReport;
use serde::{Deserialize, Serialize};

pub mod recommendations;
pub mod templates;
pub mod scoring;

#[derive(Debug, Serialize, Deserialize)]
pub struct RemediationReport {
    pub secure_vercel_json: String,
    pub secure_next_config: String,
    pub recommendations: Vec<String>,
    pub hardening_score: f64,
}

pub fn generate_report(analysis_report: &AnalysisReport) -> RemediationReport {
    let recommendations = recommendations::get_recommendations(analysis_report);
    let (secure_vercel_json, secure_next_config) = templates::generate_secure_templates();
    let hardening_score = scoring::calculate_hardening_score(analysis_report);
    
    RemediationReport {
        secure_vercel_json,
        secure_next_config,
        recommendations,
        hardening_score,
    }
}
