use super::{RiskLevel, Vulnerability};

pub fn analyze(vercel_json: &str, next_config: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();
    
    if vercel_json.contains("\"Access-Control-Allow-Origin\", \"value\": \"*\"") || next_config.contains("Access-Control-Allow-Origin', value: '*'") {
        vulns.push(Vulnerability {
            id: "CORS_01".to_string(),
            title: "Wildcard CORS Yapılandırması".to_string(),
            description: "CORS politikası tüm domainlere (*) açık şekilde yapılandırılmış. Bu durum hassas verilerin çalınmasına yol açabilir.".to_string(),
            risk_level: RiskLevel::High,
            affected_component: "CORS Headers".to_string(),
            remediation: "Access-Control-Allow-Origin başlığına sadece güvenilir domainlerin tam listesini ekleyin.".to_string(),
        });
    }
    
    vulns
}
