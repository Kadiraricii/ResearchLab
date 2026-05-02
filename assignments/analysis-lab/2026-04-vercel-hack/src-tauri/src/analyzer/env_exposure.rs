use super::{RiskLevel, Vulnerability};

pub fn analyze(env_content: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();
    let sensitive_keywords = ["SECRET", "TOKEN", "PASSWORD", "KEY", "AUTH", "DB_URL"];
    
    for line in env_content.lines() {
        if line.starts_with("NEXT_PUBLIC_") || line.starts_with("VITE_") {
            let upper_line = line.to_uppercase();
            for keyword in sensitive_keywords.iter() {
                if upper_line.contains(keyword) {
                    vulns.push(Vulnerability {
                        id: "ENV_01".to_string(),
                        title: "Hassas Veri İfşası (NEXT_PUBLIC_)".to_string(),
                        description: format!("Frontend ortamında ifşa olabilecek riskli bir anahtar bulundu: {}", line.split('=').next().unwrap_or("")),
                        risk_level: RiskLevel::Critical,
                        affected_component: "Environment Variables".to_string(),
                        remediation: "Sadece backend'de kullanılması gereken anahtarlardan NEXT_PUBLIC_ veya VITE_ ön ekini kaldırın.".to_string(),
                    });
                }
            }
        }
    }
    
    vulns
}
