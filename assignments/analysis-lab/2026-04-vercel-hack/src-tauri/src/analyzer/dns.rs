use super::{RiskLevel, Vulnerability};

pub fn analyze(vercel_json: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();

    if vercel_json.is_empty() {
        return vulns;
    }

    let Ok(val) = serde_json::from_str::<serde_json::Value>(vercel_json) else {
        return vulns;
    };

    // Check: HTTPS redirect not enforced (indicates HTTP still accepted)
    // If there's no redirect from http to https, DNS-based downgrade attacks are easier
    let has_https_redirect = val
        .get("redirects")
        .and_then(|r| r.as_array())
        .map(|arr| {
            arr.iter().any(|rule| {
                rule.get("destination")
                    .and_then(|d| d.as_str())
                    .map(|d| d.starts_with("https://"))
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false);

    // If headers include HSTS, HTTPS is enforced anyway — skip this check
    let has_hsts = vercel_json.to_lowercase().contains("strict-transport-security");

    if !has_https_redirect && !has_hsts {
        vulns.push(Vulnerability {
            id: "DNS_01".to_string(),
            title: "HTTPS Zorlaması Eksik".to_string(),
            description:
                "Vercel yapılandırmasında HTTP → HTTPS yönlendirmesi veya HSTS başlığı \
                 bulunamadı. Bu durum DNS tabanlı indirme saldırılarını kolaylaştırır."
                    .to_string(),
            risk_level: RiskLevel::Medium,
            affected_component: "DNS / HTTPS".to_string(),
            remediation:
                "vercel.json içinde Strict-Transport-Security başlığı ekleyin veya \
                 HTTP'yi HTTPS'ye yönlendiren bir redirect kuralı tanımlayın."
                    .to_string(),
        });
    }

    // Check: Wildcard domain configuration (could indicate subdomain takeover risk)
    let has_wildcard_domain = val
        .get("routes")
        .and_then(|r| r.as_array())
        .map(|arr| {
            arr.iter().any(|route| {
                route
                    .get("src")
                    .and_then(|s| s.as_str())
                    .map(|s| s.starts_with("*"))
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false);

    if has_wildcard_domain {
        vulns.push(Vulnerability {
            id: "DNS_02".to_string(),
            title: "Wildcard Route Yapılandırması".to_string(),
            description:
                "Vercel routes bölümünde wildcard (*) pattern kullanılmakta. \
                 Subdomain takeover saldırısına zemin hazırlayabilir."
                    .to_string(),
            risk_level: RiskLevel::Low,
            affected_component: "vercel.json Routes".to_string(),
            remediation:
                "Wildcard route'ları yalnızca gerçekten gerekli olduğunda kullanın \
                 ve hangi subdomainlerin aktif olduğunu düzenli denetleyin."
                    .to_string(),
        });
    }

    vulns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_missing_hsts_and_https_redirect() {
        let json = r#"{"redirects":[]}"#;
        let vulns = analyze(json);
        assert!(vulns.iter().any(|v| v.id == "DNS_01"));
    }

    #[test]
    fn no_flag_when_hsts_present() {
        let json = r#"{"headers":[{"headers":[{"key":"Strict-Transport-Security","value":"max-age=31536000"}]}]}"#;
        let vulns = analyze(json);
        assert!(!vulns.iter().any(|v| v.id == "DNS_01"));
    }

    #[test]
    fn handles_empty_input() {
        assert_eq!(analyze("").len(), 0);
    }
}
