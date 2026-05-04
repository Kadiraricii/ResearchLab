use super::{RiskLevel, Vulnerability};

pub fn analyze(vercel_json: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();
    
    if vercel_json.is_empty() {
        return vulns;
    }

    if vercel_json.contains("rewrites") {
        // Check for path traversal through rewrites pointing to internal APIs
        if vercel_json.contains("/api/") || vercel_json.contains("/internal/") || vercel_json.contains("/_") {
            if vercel_json.contains(":path*") || vercel_json.contains(":slug*") {
                vulns.push(Vulnerability {
                    id: "RWR_01".to_string(),
                    title: "Rewrite ile Dahili API İfşası".to_string(),
                    description: "vercel.json içindeki rewrite kuralı, kullanıcı girişini filtrelemeden dahili API veya özel rotalara yönlendiriyor olabilir.".to_string(),
                    risk_level: RiskLevel::High,
                    affected_component: "vercel.json Rewrites".to_string(),
                    remediation: "Rewrite kurallarında kaynak yolu için katı regex pattern kullanın. Dinamik parametre geçişini iç servislere yasaklayın.".to_string(),
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
    fn detects_path_traversal_via_rewrites() {
        let json = r#"{"rewrites":[{"source":"/:path*","destination":"/api/:path*"}]}"#;
        let vulns = analyze(json);
        assert!(vulns.iter().any(|v| v.id == "RWR_01"), "should detect path traversal via rewrites");
    }

    #[test]
    fn no_vuln_for_static_rewrite() {
        let json = r#"{"rewrites":[{"source":"/page","destination":"/static-page"}]}"#;
        let vulns = analyze(json);
        assert_eq!(vulns.len(), 0);
    }

    #[test]
    fn empty_input_returns_no_vulns() {
        assert_eq!(analyze("").len(), 0);
    }

    #[test]
    fn no_rewrite_key_returns_no_vulns() {
        let json = r#"{"headers":[]}"#;
        assert_eq!(analyze(json).len(), 0);
    }
}
