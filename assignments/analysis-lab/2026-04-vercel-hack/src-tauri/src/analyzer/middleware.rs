use super::{RiskLevel, Vulnerability};

pub fn analyze(vercel_json: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();

    if vercel_json.is_empty() {
        return vulns;
    }

    let lower = vercel_json.to_lowercase();

    // Middleware bypass: if middleware config skips auth for certain paths
    if lower.contains("middleware") {
        if lower.contains("matcher") && (lower.contains("public") || lower.contains("skip")) {
            vulns.push(Vulnerability {
                id: "MID_01".to_string(),
                title: "Middleware Bypass Riski".to_string(),
                description:
                    "Middleware yapılandırması bazı rotaları doğrulama dışında bırakıyor \
                     olabilir. Yanlış matcher kuralları yetkisiz erişime kapı açar."
                        .to_string(),
                risk_level: RiskLevel::High,
                affected_component: "Middleware Configuration".to_string(),
                remediation:
                    "Middleware matcher kurallarını gözden geçirin. Varsayılan olarak tüm \
                     rotaları koruyun ve sadece gerçekten genel olan yolları (örn: /public, \
                     /assets) hariç tutun."
                        .to_string(),
            });
        }
    }

    vulns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_middleware_skip_pattern() {
        let json = r#"{"middleware":{"matcher":["/((?!public|skip).*)"],"config":{}}}"#;
        let vulns = analyze(json);
        assert!(vulns.iter().any(|v| v.id == "MID_01"), "should detect middleware bypass risk");
    }

    #[test]
    fn no_vuln_when_no_middleware_config() {
        let json = r#"{"headers":[]}"#;
        let vulns = analyze(json);
        assert_eq!(vulns.len(), 0);
    }

    #[test]
    fn empty_input_returns_no_vulns() {
        assert_eq!(analyze("").len(), 0);
    }

    #[test]
    fn middleware_without_skip_returns_no_vuln() {
        // middleware present but no "public"/"skip" skip-list
        let json = r#"{"middleware":{"matcher":["/(.*)"]}}"#;
        let vulns = analyze(json);
        assert_eq!(vulns.len(), 0);
    }
}
