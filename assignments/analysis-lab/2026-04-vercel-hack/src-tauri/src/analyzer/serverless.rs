use super::{RiskLevel, Vulnerability};

pub fn analyze(vercel_json: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();

    if vercel_json.is_empty() {
        return vulns;
    }

    // Serverless function security: check for missing input validation patterns
    // Heuristic: if there's an api/ route rewrite without any validation hints
    if vercel_json.contains("/api/") || vercel_json.contains("\"functions\"") {
        // Check if any common security middleware markers are absent
        if !vercel_json.contains("rateLimit") && !vercel_json.contains("rate-limit") {
            vulns.push(Vulnerability {
                id: "SLS_01".to_string(),
                title: "Serverless Function Rate Limiting Eksikliği".to_string(),
                description:
                    "API rotaları yapılandırmasında rate limiting tanımı tespit edilemedi. \
                     Bu durum DDoS ve brute-force saldırılarına karşı savunmasızlık oluşturur."
                        .to_string(),
                risk_level: RiskLevel::Medium,
                affected_component: "Serverless Functions / API Routes".to_string(),
                remediation:
                    "API rotalarınıza rate limiting middleware ekleyin (ör: Vercel Edge \
                     Middleware, upstash/ratelimit veya express-rate-limit)."
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
    fn detects_api_route_without_rate_limit() {
        let json = r#"{"rewrites":[{"source":"/api/:path*","destination":"/api/:path*"}]}"#;
        let vulns = analyze(json);
        assert!(vulns.iter().any(|v| v.id == "SLS_01"), "should flag missing rate limit");
    }

    #[test]
    fn no_vuln_when_rate_limit_mentioned() {
        let json = r#"{"functions":{"/api/**":{"memory":1024,"rateLimit":{"limit":100}}}}"#;
        let vulns = analyze(json);
        assert_eq!(vulns.len(), 0);
    }

    #[test]
    fn empty_input_returns_no_vulns() {
        assert_eq!(analyze("").len(), 0);
    }

    #[test]
    fn no_api_routes_returns_no_vulns() {
        let json = r#"{"headers":[]}"#;
        assert_eq!(analyze(json).len(), 0);
    }
}
