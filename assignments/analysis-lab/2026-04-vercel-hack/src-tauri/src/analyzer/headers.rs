use super::{RiskLevel, Vulnerability};

pub fn analyze(vercel_json: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();
    
    if vercel_json.is_empty() {
        return vulns;
    }
    
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_missing_hsts() {
        let json = r#"{"headers":[{"source":"/(.*)", "headers":[{"key":"Content-Security-Policy","value":"default-src 'self'"}]}]}"#;
        let vulns = analyze(json);
        assert!(vulns.iter().any(|v| v.id == "HDR_01"), "should detect missing HSTS");
    }

    #[test]
    fn detects_missing_csp() {
        let json = r#"{"headers":[{"source":"/(.*)", "headers":[{"key":"Strict-Transport-Security","value":"max-age=63072000"}]}]}"#;
        let vulns = analyze(json);
        assert!(vulns.iter().any(|v| v.id == "HDR_02"), "should detect missing CSP");
    }

    #[test]
    fn no_vulns_when_all_headers_present() {
        let json = r#"{"headers":[{"source":"/(.*)", "headers":[{"key":"Strict-Transport-Security","value":"max-age=63072000"},{"key":"Content-Security-Policy","value":"default-src 'self'"}]}]}"#;
        let vulns = analyze(json);
        assert_eq!(vulns.len(), 0, "fully configured headers should have no vulns");
    }

    #[test]
    fn empty_input_returns_no_vulns() {
        let vulns = analyze("");
        assert_eq!(vulns.len(), 0);
    }

    #[test]
    fn detects_both_missing_headers() {
        let json = r#"{"headers":[]}"#;
        let vulns = analyze(json);
        assert_eq!(vulns.len(), 2, "should detect both missing HDR_01 and HDR_02");
    }
}

