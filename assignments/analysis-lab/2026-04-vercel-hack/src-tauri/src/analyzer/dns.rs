use super::{RiskLevel, Vulnerability};

pub fn analyze(vercel_json: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();

    if vercel_json.is_empty() {
        return vulns;
    }

    // Dangling CNAME / subdomain takeover risk
    let lower = vercel_json.to_lowercase();
    if lower.contains("cname") || lower.contains("domains") {
        vulns.push(Vulnerability {
            id: "DNS_01".to_string(),
            title: "Olası DNS Takeover (Dangling CNAME)".to_string(),
            description:
                "Yapılandırmada domain veya CNAME referansı tespit edildi. \
                 Silinmiş bir Vercel projesine işaret eden CNAME kayıtları subdomain \
                 hijacking saldırılarına kapı açabilir."
                    .to_string(),
            risk_level: RiskLevel::High,
            affected_component: "DNS / Domain Configuration".to_string(),
            remediation:
                "Kullanılmayan Vercel projelerini silerken DNS sağlayıcınızdaki CNAME \
                 kayıtlarını da silin. DNSSEC'i etkinleştirin."
                    .to_string(),
        });
    }

    vulns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_cname_reference() {
        let json = r#"{"domains":["app.example.com"],"cname":"cname.vercel-dns.com"}"#;
        let vulns = analyze(json);
        assert!(vulns.iter().any(|v| v.id == "DNS_01"));
    }

    #[test]
    fn detects_domains_key() {
        let json = r#"{"domains":["staging.example.com"]}"#;
        let vulns = analyze(json);
        assert!(vulns.iter().any(|v| v.id == "DNS_01"));
    }

    #[test]
    fn no_vuln_for_config_without_dns() {
        let json = r#"{"headers":[]}"#;
        let vulns = analyze(json);
        assert_eq!(vulns.len(), 0);
    }

    #[test]
    fn empty_input_returns_no_vulns() {
        assert_eq!(analyze("").len(), 0);
    }
}
