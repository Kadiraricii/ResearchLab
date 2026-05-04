use super::{RiskLevel, Vulnerability};

pub fn analyze(vercel_json: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();

    if vercel_json.is_empty() {
        return vulns;
    }

    let Ok(val) = serde_json::from_str::<serde_json::Value>(vercel_json) else {
        return vulns;
    };

    // Check 1: github.silent: true suppresses PR deployment comments, reducing review visibility
    if val
        .get("github")
        .and_then(|g| g.get("silent"))
        .and_then(|s| s.as_bool())
        .unwrap_or(false)
    {
        vulns.push(Vulnerability {
            id: "PREV_01".to_string(),
            title: "GitHub Deployment Yorumları Devre Dışı".to_string(),
            description:
                "github.silent: true ayarı PR'larda deployment durum yorumlarını gizler. \
                 Güvenlik incelemeleri ve deployment durumu takibi atlanabilir."
                    .to_string(),
            risk_level: RiskLevel::Low,
            affected_component: "vercel.json GitHub Integration".to_string(),
            remediation:
                "github.silent ayarını kaldırın ya da false olarak ayarlayın. \
                 Deployment yorumları, güvenlik incelemesi için kritik görünürlük sağlar."
                    .to_string(),
        });
    }

    // Check 2: github.enabled: false disables integration and all automated deployment checks
    if val
        .get("github")
        .and_then(|g| g.get("enabled"))
        .and_then(|e| e.as_bool())
        == Some(false)
    {
        vulns.push(Vulnerability {
            id: "PREV_02".to_string(),
            title: "GitHub Entegrasyonu Devre Dışı".to_string(),
            description:
                "github.enabled: false ayarı GitHub entegrasyonunu ve otomatik \
                 deployment kontrollerini devre dışı bırakır. Güvenli olmayan kodun \
                 inceleme olmadan deploy edilmesi riski oluşur."
                    .to_string(),
            risk_level: RiskLevel::Medium,
            affected_component: "vercel.json GitHub Integration".to_string(),
            remediation:
                "github.enabled: false ayarını kaldırın. GitHub entegrasyonu, \
                 deployment'ları PR ve review süreciyle ilişkilendirir."
                    .to_string(),
        });
    }

    // Check 3: public: true makes deployment accessible to everyone without auth
    // (legacy field — still accepted by Vercel for older projects)
    if val
        .get("public")
        .and_then(|p| p.as_bool())
        .unwrap_or(false)
    {
        vulns.push(Vulnerability {
            id: "PREV_03".to_string(),
            title: "Deployment Herkese Açık Olarak İşaretlenmiş".to_string(),
            description:
                "public: true ayarı deployment'ı kimlik doğrulama gerektirmeksizin \
                 herkese açık hale getirir. Preview ortamlarındaki hassas veriler \
                 dışarıya sızabilir."
                    .to_string(),
            risk_level: RiskLevel::High,
            affected_component: "vercel.json".to_string(),
            remediation:
                "public: true ayarını kaldırın. Vercel Dashboard > Settings > \
                 Deployment Protection bölümünden Password Protection veya \
                 Vercel Authentication etkinleştirin."
                    .to_string(),
        });
    }

    vulns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_github_silent() {
        let json = r#"{"github":{"silent":true}}"#;
        assert!(analyze(json).iter().any(|v| v.id == "PREV_01"));
    }

    #[test]
    fn flags_github_disabled() {
        let json = r#"{"github":{"enabled":false}}"#;
        assert!(analyze(json).iter().any(|v| v.id == "PREV_02"));
    }

    #[test]
    fn flags_public_deployment() {
        let json = r#"{"public":true}"#;
        assert!(analyze(json).iter().any(|v| v.id == "PREV_03"));
    }

    #[test]
    fn no_flag_for_secure_github_config() {
        let json = r#"{"github":{"enabled":true,"silent":false}}"#;
        assert_eq!(analyze(json).len(), 0);
    }

    #[test]
    fn no_flag_when_public_false() {
        let json = r#"{"public":false}"#;
        assert_eq!(analyze(json).len(), 0);
    }

    #[test]
    fn handles_empty_input() {
        assert_eq!(analyze("").len(), 0);
    }
}
