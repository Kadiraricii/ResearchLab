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

pub fn generate_report() -> RemediationReport {
    let recommendations = recommendations::get_recommendations();
    let (secure_vercel_json, secure_next_config) = templates::generate_secure_templates();
    let hardening_score = scoring::calculate_hardening_score();
    
    RemediationReport {
        secure_vercel_json,
        secure_next_config,
        recommendations,
        hardening_score,
    }
}
