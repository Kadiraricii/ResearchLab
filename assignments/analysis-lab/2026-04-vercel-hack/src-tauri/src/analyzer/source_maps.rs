use super::{RiskLevel, Vulnerability};

pub fn analyze(next_config: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();
    
    if next_config.contains("productionBrowserSourceMaps: true") {
        vulns.push(Vulnerability {
            id: "SRC_01".to_string(),
            title: "Source Map İfşası".to_string(),
            description: "productionBrowserSourceMaps özelliği aktif edilmiş. Bu durum üretim ortamında kaynak kodların tamamen okunabilmesine neden olur.".to_string(),
            risk_level: RiskLevel::High,
            affected_component: "next.config.js".to_string(),
            remediation: "productionBrowserSourceMaps: true ayarını kaldırın veya false olarak değiştirin.".to_string(),
        });
    }
    
    vulns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_source_maps_enabled() {
        let config = "productionBrowserSourceMaps: true";
        let vulns = analyze(config);
        assert_eq!(vulns.len(), 1);
        assert_eq!(vulns[0].id, "SRC_01");
        assert!(matches!(vulns[0].risk_level, RiskLevel::High));
    }

    #[test]
    fn no_vuln_when_source_maps_disabled() {
        let config = "productionBrowserSourceMaps: false";
        let vulns = analyze(config);
        assert_eq!(vulns.len(), 0);
    }

    #[test]
    fn no_vuln_when_config_empty() {
        let vulns = analyze("");
        assert_eq!(vulns.len(), 0);
    }

    #[test]
    fn no_false_positive_from_unrelated_config() {
        let config = "reactStrictMode: true\npoweredByHeader: false";
        let vulns = analyze(config);
        assert_eq!(vulns.len(), 0);
    }

    #[test]
    fn no_false_positive_when_source_maps_not_explicitly_true() {
        // If someone writes productionBrowserSourceMaps without a value,
        // or with an unrecognized value, we should not flag it.
        let config = "productionBrowserSourceMaps: 1";
        let vulns = analyze(config);
        assert_eq!(vulns.len(), 0);
    }
}
