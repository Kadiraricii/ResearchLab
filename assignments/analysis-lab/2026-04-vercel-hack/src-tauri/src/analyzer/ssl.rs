use super::{RiskLevel, Vulnerability};

pub fn analyze(vercel_json: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();

    if vercel_json.is_empty() {
        return vulns;
    }

    let lower = vercel_json.to_lowercase();

    // Check for explicit HTTP (non-HTTPS) destinations or missing TLS enforcement
    if lower.contains("\"http://") || lower.contains("'http://") {
        vulns.push(Vulnerability {
            id: "SSL_01".to_string(),
            title: "HTTP (Şifresiz) Bağlantı Tespit Edildi".to_string(),
            description:
                "Yapılandırma dosyasında http:// ile başlayan bir URL tespit edildi. \
                 Bu trafik şifrelenmemiş (plaintext) iletilir ve dinleme saldırılarına açıktır."
                    .to_string(),
            risk_level: RiskLevel::High,
            affected_component: "SSL/TLS Configuration".to_string(),
            remediation:
                "Tüm bağlantıları https:// protokolüne taşıyın ve HSTS başlığını ekleyin."
                    .to_string(),
        });
    }

    vulns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_http_url_in_config() {
        let json = r#"{"redirects":[{"source":"/old","destination":"http://example.com"}]}"#;
        let vulns = analyze(json);
        assert!(vulns.iter().any(|v| v.id == "SSL_01"), "should flag http:// URL");
    }

    #[test]
    fn no_vuln_for_https_url() {
        let json = r#"{"redirects":[{"source":"/old","destination":"https://example.com"}]}"#;
        let vulns = analyze(json);
        assert_eq!(vulns.len(), 0);
    }

    #[test]
    fn empty_input_returns_no_vulns() {
        assert_eq!(analyze("").len(), 0);
    }

    #[test]
    fn no_vuln_for_headers_only_config() {
        let json = r#"{"headers":[{"source":"/(.*)", "headers":[{"key":"Strict-Transport-Security","value":"max-age=63072000"}]}]}"#;
        let vulns = analyze(json);
        assert_eq!(vulns.len(), 0);
    }
}
