use super::{RiskLevel, Vulnerability};

pub fn analyze(vercel_json: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();
    
    if vercel_json.is_empty() {
        return vulns;
    }

    // Check for open redirects: destination contains a variable that could be abused
    if vercel_json.contains("redirects") {
        if vercel_json.contains(":path*") || vercel_json.contains(":slug*") {
            // Check for external redirect destinations using wildcards
            if vercel_json.contains("https://") || vercel_json.contains("http://") {
                vulns.push(Vulnerability {
                    id: "RDR_01".to_string(),
                    title: "Açık Yönlendirme (Open Redirect)".to_string(),
                    description: "vercel.json içindeki redirect kuralı kullanıcı kontrolündeki yolu dış URL'ye yönlendiriyor olabilir. Bu durum phishing saldırılarına kapı açar.".to_string(),
                    risk_level: RiskLevel::Medium,
                    affected_component: "vercel.json Redirects".to_string(),
                    remediation: "Redirect hedeflerini sabit URL'ler ile sınırlayın. Dinamik path parametrelerini harici URL'lere asla aktarmayın.".to_string(),
                });
            }
        }
    }
    
    vulns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_open_redirect_with_external_url() {
        let json = r#"{"redirects":[{"source":"/:path*","destination":"https://external.com/:path*","permanent":false}]}"#;
        let vulns = analyze(json);
        assert!(vulns.iter().any(|v| v.id == "RDR_01"), "should detect open redirect");
    }

    #[test]
    fn no_vuln_for_internal_redirect() {
        let json = r#"{"redirects":[{"source":"/old","destination":"/new","permanent":true}]}"#;
        let vulns = analyze(json);
        assert_eq!(vulns.len(), 0, "internal redirect should not trigger RDR_01");
    }

    #[test]
    fn empty_input_returns_no_vulns() {
        assert_eq!(analyze("").len(), 0);
    }

    #[test]
    fn no_redirects_key_returns_no_vulns() {
        let json = r#"{"headers":[]}"#;
        assert_eq!(analyze(json).len(), 0);
    }
}
