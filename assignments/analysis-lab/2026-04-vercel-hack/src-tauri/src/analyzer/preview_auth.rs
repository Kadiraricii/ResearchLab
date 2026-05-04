use super::{RiskLevel, Vulnerability};

pub fn analyze(vercel_json: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();

    if vercel_json.is_empty() {
        return vulns;
    }

    let lower = vercel_json.to_lowercase();

    // Check for preview deployments without authentication
    // Heuristic: if "preview" context is mentioned without auth settings
    if lower.contains("preview") && !lower.contains("authentication") && !lower.contains("password") {
        vulns.push(Vulnerability {
            id: "PRV_01".to_string(),
            title: "Korumasız Preview Deployment".to_string(),
            description:
                "Preview deployment yapılandırmasında kimlik doğrulama (authentication) \
                 veya şifre koruması tanımlanmamış. Bu durum yetkisiz kişilerin \
                 geliştirme ortamına erişmesine olanak tanır."
                    .to_string(),
            risk_level: RiskLevel::Medium,
            affected_component: "Preview Deployments".to_string(),
            remediation:
                "Vercel Dashboard'dan projeniz için 'Vercel Authentication' veya \
                 'Password Protection' özelliğini etkinleştirin."
                    .to_string(),
        });
    }

    vulns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_unprotected_preview() {
        let json = r#"{"github":{"enabled":true,"silent":true,"autoAlias":true},"preview":{"enabled":true}}"#;
        let vulns = analyze(json);
        assert!(vulns.iter().any(|v| v.id == "PRV_01"), "should detect unprotected preview");
    }

    #[test]
    fn no_vuln_when_preview_has_auth() {
        let json = r#"{"preview":{"authentication":{"enabled":true}}}"#;
        let vulns = analyze(json);
        assert_eq!(vulns.len(), 0);
    }

    #[test]
    fn no_vuln_when_preview_has_password() {
        let json = r#"{"preview":{"password":"mypassword"}}"#;
        let vulns = analyze(json);
        assert_eq!(vulns.len(), 0);
    }

    #[test]
    fn empty_input_returns_no_vulns() {
        assert_eq!(analyze("").len(), 0);
    }

    #[test]
    fn no_preview_key_returns_no_vuln() {
        let json = r#"{"headers":[]}"#;
        assert_eq!(analyze(json).len(), 0);
    }
}
