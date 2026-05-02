use super::{RiskLevel, Vulnerability};

pub fn analyze(vercel_json: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();
    if vercel_json.contains("destination\": \"http://internal") || vercel_json.contains("destination\": \"http://localhost") {
        vulns.push(Vulnerability {
            id: "REWR_01".to_string(),
            title: "İç Ağ İfşası (SSRF via Rewrites)".to_string(),
            description: "Rewrite kurallarında iç ağ (internal/localhost) hedeflerine proxy yapıldığı tespit edildi.".to_string(),
            risk_level: RiskLevel::High,
            affected_component: "vercel.json Rewrites".to_string(),
            remediation: "Dışarıdan erişilebilen rewrite kurallarında iç ağ IP veya domainlerini hedef göstermeyin.".to_string(),
        });
    }
    vulns
}
