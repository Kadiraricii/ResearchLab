use super::{RiskLevel, Vulnerability};

/// Internal network prefixes that indicate SSRF risk when used as rewrite destinations.
const INTERNAL_PREFIXES: &[&str] = &[
    "http://localhost",
    "http://127.",
    "http://0.0.0.0",
    "http://192.168.",
    "http://10.",
    "http://172.16.",
    "http://172.17.",
    "http://172.18.",
    "http://172.31.",
    "http://internal",
    "http://metadata.google.internal",
    "http://169.254.",  // AWS/GCP metadata service
];

pub fn analyze(vercel_json: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();

    if vercel_json.is_empty() {
        return vulns;
    }

    // FIX: Parse JSON and check each rewrite rule individually.
    // FIX: Use case-insensitive comparison to catch HTTP uppercase variants.
    // FIX: Check multiple internal network ranges (127.x, 10.x, 192.168.x, etc.)
    let Ok(val) = serde_json::from_str::<serde_json::Value>(vercel_json) else {
        return vulns;
    };

    let Some(rewrites) = val.get("rewrites").and_then(|r| r.as_array()) else {
        return vulns;
    };

    for rewrite in rewrites {
        let source = rewrite
            .get("source")
            .and_then(|s| s.as_str())
            .unwrap_or("");
        let destination = rewrite
            .get("destination")
            .and_then(|d| d.as_str())
            .unwrap_or("");

        let lower_dest = destination.to_lowercase();

        if INTERNAL_PREFIXES.iter().any(|prefix| lower_dest.starts_with(prefix)) {
            vulns.push(Vulnerability {
                id: "REWR_01".to_string(),
                title: "İç Ağ İfşası (SSRF via Rewrites)".to_string(),
                description: format!(
                    "Rewrite kuralında iç ağ adresine proxy tespit edildi. Source: '{source}' \
                     → Destination: '{destination}'"
                ),
                risk_level: RiskLevel::High,
                affected_component: "vercel.json Rewrites".to_string(),
                remediation:
                    "Dışarıdan erişilebilen rewrite kurallarında iç ağ IP veya domainlerini \
                     hedef göstermeyin. Gerekiyorsa edge middleware ile erişim denetimi ekleyin."
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
    fn detects_localhost_destination() {
        let json = r#"{"rewrites":[{"source":"/api/:path*","destination":"http://localhost:3001/:path*"}]}"#;
        assert_eq!(analyze(json).len(), 1);
    }

    #[test]
    fn detects_internal_destination() {
        let json = r#"{"rewrites":[{"source":"/admin/:path*","destination":"http://internal:8080/:path*"}]}"#;
        assert_eq!(analyze(json).len(), 1);
    }

    #[test]
    fn detects_aws_metadata_ssrf() {
        let json = r#"{"rewrites":[{"source":"/meta","destination":"http://169.254.169.254/latest/meta-data/"}]}"#;
        assert_eq!(analyze(json).len(), 1);
    }

    #[test]
    fn detects_private_ip_192_168() {
        let json = r#"{"rewrites":[{"source":"/db","destination":"http://192.168.1.100:5432"}]}"#;
        assert_eq!(analyze(json).len(), 1);
    }

    #[test]
    fn no_false_positive_for_external_destination() {
        let json = r#"{"rewrites":[{"source":"/api/:path*","destination":"https://api.example.com/:path*"}]}"#;
        assert_eq!(analyze(json).len(), 0);
    }

    #[test]
    fn handles_empty_input() {
        assert_eq!(analyze("").len(), 0);
    }

    #[test]
    fn handles_invalid_json() {
        assert_eq!(analyze("{bad json}").len(), 0);
    }
}
