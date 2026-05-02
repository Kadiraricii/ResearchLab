use super::{RiskLevel, Vulnerability};

pub fn analyze(vercel_json: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();
    
    if vercel_json.is_empty() {
        return vulns; // Skip if no config
    }
    
    // Basit string analizi
    if !vercel_json.contains("Strict-Transport-Security") {
        vulns.push(Vulnerability {
            id: "HDR_01".to_string(),
            title: "Eksik HSTS Başlığı".to_string(),
            description: "Strict-Transport-Security (HSTS) başlığı tanımlanmamış. Bu durum Ortadaki Adam (MitM) saldırılarını kolaylaştırır.".to_string(),
            risk_level: RiskLevel::Medium,
            affected_component: "vercel.json Headers".to_string(),
            remediation: "vercel.json içine Strict-Transport-Security başlığını ekleyin.".to_string(),
        });
    }

    if !vercel_json.contains("Content-Security-Policy") {
        vulns.push(Vulnerability {
            id: "HDR_02".to_string(),
            title: "Eksik CSP Başlığı".to_string(),
            description: "Content-Security-Policy (CSP) başlığı eksik. Bu durum XSS saldırılarına zemin hazırlar.".to_string(),
            risk_level: RiskLevel::High,
            affected_component: "vercel.json Headers".to_string(),
            remediation: "XSS koruması için uygun bir CSP başlığı tanımlayın.".to_string(),
        });
    }
    
    vulns
}
