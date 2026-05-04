use super::{RiskLevel, Vulnerability};

pub fn analyze(vercel_json: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();

    if vercel_json.is_empty() {
        return vulns;
    }

    let lower = vercel_json.to_lowercase();

    // Check 1: HSTS max-age below recommended minimum (1 year = 31536000 seconds)
    if lower.contains("strict-transport-security") {
        if let Some(max_age) = extract_hsts_max_age(&lower) {
            if max_age < 31_536_000 {
                vulns.push(Vulnerability {
                    id: "SSL_01".to_string(),
                    title: "HSTS max-age Çok Düşük".to_string(),
                    description: format!(
                        "Strict-Transport-Security max-age değeri {max_age}s; \
                         önerilen minimum 31536000s (1 yıl)."
                    ),
                    risk_level: RiskLevel::Medium,
                    affected_component: "HSTS Header".to_string(),
                    remediation:
                        "max-age değerini en az 31536000 (1 yıl) olarak ayarlayın. \
                         HSTS preload için 63072000 (2 yıl) önerirsiz."
                            .to_string(),
                });
            }
        }

        // Check 2: HSTS without includeSubDomains
        if !lower.contains("includesubdomains") {
            vulns.push(Vulnerability {
                id: "SSL_02".to_string(),
                title: "HSTS includeSubDomains Eksik".to_string(),
                description:
                    "HSTS başlığında includeSubDomains direktifi yok. \
                     Alt domainler HTTP üzerinden erişilebilir kalmaya devam eder."
                        .to_string(),
                risk_level: RiskLevel::Low,
                affected_component: "HSTS Header".to_string(),
                remediation:
                    "Strict-Transport-Security değerine '; includeSubDomains' ekleyin."
                        .to_string(),
            });
        }
    }

    vulns
}

fn extract_hsts_max_age(lower_config: &str) -> Option<u64> {
    let start = lower_config.find("max-age=")? + "max-age=".len();
    let digits: String = lower_config[start..]
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    digits.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_low_max_age() {
        let json = r#"{"headers":[{"headers":[{"key":"Strict-Transport-Security","value":"max-age=86400"}]}]}"#;
        assert!(analyze(json).iter().any(|v| v.id == "SSL_01"));
    }

    #[test]
    fn flags_missing_include_subdomains() {
        let json = r#"{"headers":[{"headers":[{"key":"Strict-Transport-Security","value":"max-age=31536000"}]}]}"#;
        let vulns = analyze(json);
        assert!(vulns.iter().any(|v| v.id == "SSL_02"));
        assert!(!vulns.iter().any(|v| v.id == "SSL_01"));
    }

    #[test]
    fn no_flag_for_correct_hsts() {
        let json = r#"{"headers":[{"headers":[{"key":"Strict-Transport-Security","value":"max-age=63072000; includeSubDomains; preload"}]}]}"#;
        assert_eq!(analyze(json).len(), 0);
    }

    #[test]
    fn handles_empty_input() {
        assert_eq!(analyze("").len(), 0);
    }
}
