use serde::{Deserialize, Serialize};

pub mod env_exposure;
pub mod source_maps;
pub mod headers;
pub mod cors;
pub mod redirects;
pub mod rewrites;
pub mod dns;
pub mod ssl;
pub mod serverless;
pub mod middleware;
pub mod preview_auth;
pub mod build_logs;
pub mod scoring;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RiskLevel {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vulnerability {
    pub id: String,
    pub title: String,
    pub description: String,
    pub risk_level: RiskLevel,
    pub affected_component: String,
    pub remediation: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnalysisReport {
    pub vulnerabilities: Vec<Vulnerability>,
    pub score: f64,
}

pub fn run_all_analyzers(vercel_json: &str, next_config: &str, env_content: &str) -> AnalysisReport {
    let mut vulns = Vec::new();
    
    vulns.extend(env_exposure::analyze(env_content));
    vulns.extend(headers::analyze(vercel_json));
    vulns.extend(cors::analyze(vercel_json, next_config));
    vulns.extend(source_maps::analyze(next_config));
    vulns.extend(redirects::analyze(vercel_json));
    vulns.extend(rewrites::analyze(vercel_json));
    // Diger stub moduller cagriliyor
    vulns.extend(dns::analyze());
    vulns.extend(ssl::analyze());
    vulns.extend(serverless::analyze());
    vulns.extend(middleware::analyze());
    vulns.extend(preview_auth::analyze());
    vulns.extend(build_logs::analyze());
    
    let score = scoring::calculate_cvss_like_score(&vulns);
    
    AnalysisReport {
        vulnerabilities: vulns,
        score,
    }
}
