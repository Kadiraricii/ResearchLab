use super::{RiskLevel, Vulnerability};

pub fn analyze(vercel_json: &str, next_config: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();

    // FIX: Use a two-step approach instead of exact string matching:
    // 1. Check if Access-Control-Allow-Origin appears anywhere in the config
    // 2. Check if a wildcard value (*) is nearby
    // This catches JSON with whitespace variations and different quote styles.
    if has_wildcard_cors(vercel_json) || has_wildcard_cors(next_config) {
        vulns.push(Vulnerability {
            id: "CORS_01".to_string(),
            title: "Wildcard CORS Yapılandırması".to_string(),
            description:
                "CORS politikası tüm domainlere (*) açık şekilde yapılandırılmış. Credentials \
                 ile birlikte kullanıldığında hassas verilerin çalınmasına yol açabilir."
                    .to_string(),
            risk_level: RiskLevel::High,
            affected_component: "CORS Headers".to_string(),
            remediation:
                "Access-Control-Allow-Origin başlığına sadece güvenilir domainlerin tam \
                 listesini ekleyin. Wildcard (*) ve credentials'ı birlikte kullanmayın."
                    .to_string(),
        });
    }

    // Additional check: CORS with credentials + wildcard is Critical (browsers block it,
    // but misconfigured servers may not)
    if has_credentials_cors(vercel_json) || has_credentials_cors(next_config) {
        vulns.push(Vulnerability {
            id: "CORS_02".to_string(),
            title: "CORS Allow-Credentials Yapılandırması".to_string(),
            description:
                "Access-Control-Allow-Credentials: true tespit edildi. Bu ayar, kötüye \
                 kullanılırsa çapraz kaynak kimlik doğrulama saldırılarına kapı açar."
                    .to_string(),
            risk_level: RiskLevel::Medium,
            affected_component: "CORS Headers".to_string(),
            remediation:
                "Allow-Credentials: true kullanıyorsanız, Allow-Origin başlığına kesinlikle \
                 wildcard (*) yerine belirli domain listesi girin."
                    .to_string(),
        });
    }

    vulns
}

/// Returns true if the config string contains a wildcard CORS origin setting.
/// Handles different quote styles and whitespace variations.
fn has_wildcard_cors(config: &str) -> bool {
    let lower = config.to_lowercase();
    if !lower.contains("access-control-allow-origin") {
        return false;
    }
    // After finding the header name, check for wildcard value in various formats:
    // "value": "*", 'value': '*', : "*", : '*', value:"*"
    lower.contains(": \"*\"")
        || lower.contains(": '*'")
        || lower.contains(":\"*\"")
        || lower.contains(":'*'")
        || lower.contains("= \"*\"")
        || lower.contains("= '*'")
        || lower.contains("=\"*\"")
        || lower.contains("='*'")
}

fn has_credentials_cors(config: &str) -> bool {
    let lower = config.to_lowercase();
    lower.contains("access-control-allow-credentials")
        && (lower.contains(": true") || lower.contains(":true") || lower.contains("= true"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_wildcard_in_vercel_json() {
        let json = r#"{"headers":[{"source":"/(.*)", "headers":[{"key":"Access-Control-Allow-Origin","value":"*"}]}]}"#;
        let vulns = analyze(json, "");
        assert!(vulns.iter().any(|v| v.id == "CORS_01"));
    }

    #[test]
    fn detects_wildcard_in_next_config() {
        let next = "headers: [{'key': 'Access-Control-Allow-Origin', 'value': '*'}]";
        let vulns = analyze("", next);
        assert!(vulns.iter().any(|v| v.id == "CORS_01"));
    }

    #[test]
    fn detects_credentials_header() {
        let next = "Access-Control-Allow-Credentials: true";
        let vulns = analyze("", next);
        assert!(vulns.iter().any(|v| v.id == "CORS_02"));
    }

    #[test]
    fn no_false_positive_for_specific_origin() {
        let json = r#"{"headers":[{"headers":[{"key":"Access-Control-Allow-Origin","value":"https://app.example.com"}]}]}"#;
        let vulns = analyze(json, "");
        assert!(!vulns.iter().any(|v| v.id == "CORS_01"));
    }

    #[test]
    fn handles_empty_input() {
        assert_eq!(analyze("", "").len(), 0);
    }
}
