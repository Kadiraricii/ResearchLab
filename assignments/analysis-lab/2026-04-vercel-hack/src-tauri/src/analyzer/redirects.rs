use super::{RiskLevel, Vulnerability};

pub fn analyze(vercel_json: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();
    if vercel_json.contains("destination\": \"http") && vercel_json.contains("source\": \"/(.*)") {
        vulns.push(Vulnerability {
            id: "REDIR_01".to_string(),
            title: "Olası Open Redirect Zafiyeti".to_string(),
            description: "Redirect kurallarında dış bağlantılara (http://...) açık regex yakalamaları tespit edildi.".to_string(),
            risk_level: RiskLevel::Medium,
            affected_component: "vercel.json Redirects".to_string(),
            remediation: "Yönlendirmelerde kullanıcı girdisini (.*) doğrudan destination olarak kullanmaktan kaçının.".to_string(),
        });
    }
    vulns
}
