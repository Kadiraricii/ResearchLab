use super::{RiskLevel, Vulnerability};

pub fn calculate_cvss_like_score(vulns: &[Vulnerability]) -> f64 {
    let mut score = 0.0;
    
    for vuln in vulns {
        match vuln.risk_level {
            RiskLevel::Critical => score += 9.5,
            RiskLevel::High => score += 7.5,
            RiskLevel::Medium => score += 5.0,
            RiskLevel::Low => score += 2.5,
            RiskLevel::Info => score += 0.0,
        }
    }
    
    // Basit bir cap
    if score > 10.0 {
        10.0
    } else {
        score
    }
}
