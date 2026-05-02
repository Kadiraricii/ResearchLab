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
