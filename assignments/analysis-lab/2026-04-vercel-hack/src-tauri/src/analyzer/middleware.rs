use super::{RiskLevel, Vulnerability};

/// Path prefixes considered sensitive — access should be restricted.
const SENSITIVE_PATHS: &[&str] = &[
    "/admin", "/api/admin", "/_admin", "/dashboard", "/internal", "/management",
];

pub fn analyze(vercel_json: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();

    if vercel_json.is_empty() {
        return vulns;
    }

    let Ok(val) = serde_json::from_str::<serde_json::Value>(vercel_json) else {
        return vulns;
    };

    let Some(routes) = val.get("routes").and_then(|r| r.as_array()) else {
        return vulns;
    };

    for route in routes {
        let src = route.get("src").and_then(|s| s.as_str()).unwrap_or("");
        let dest = route.get("dest").and_then(|d| d.as_str()).unwrap_or("");
        let has_continue = route
            .get("continue")
            .and_then(|c| c.as_bool())
            .unwrap_or(false);
        let has_condition = route.get("has").is_some();

        // Check 1: `continue: true` on a sensitive path skips downstream auth middleware
        if has_continue && SENSITIVE_PATHS.iter().any(|p| src.contains(p)) {
            vulns.push(Vulnerability {
                id: "MDLW_01".to_string(),
                title: "Middleware Zinciri Atlama Riski".to_string(),
                description: format!(
                    "'{src}' route'unda `continue: true` ayarı var ve hassas bir yola \
                     eşleşiyor. Bu, sonraki kimlik doğrulama middleware'lerinin \
                     atlanmasına yol açabilir."
                ),
                risk_level: RiskLevel::High,
                affected_component: "vercel.json Routes".to_string(),
                remediation:
                    "Hassas rotalar için `continue: true` kullanımını kaldırın. \
                     Kimlik doğrulama middleware'ini rota zincirinin başına yerleştirin."
                        .to_string(),
            });
        }

        // Check 2: Route to a sensitive destination with no `has` condition guard
        if !dest.is_empty()
            && SENSITIVE_PATHS.iter().any(|p| dest.starts_with(p))
            && !has_condition
        {
            vulns.push(Vulnerability {
                id: "MDLW_02".to_string(),
                title: "Korumasız Hassas Rota Hedefi".to_string(),
                description: format!(
                    "'{src}' → '{dest}' rotası hassas bir hedefe yönlendiriyor, \
                     ancak herhangi bir `has` koşulu (auth header vb.) içermiyor. \
                     Kimlik doğrulanmamış istekler bu hedefe ulaşabilir."
                ),
                risk_level: RiskLevel::Medium,
                affected_component: "vercel.json Routes".to_string(),
                remediation:
                    "Hassas rotalara `has` koşuluyla kimlik doğrulama header kontrolü \
                     ekleyin veya Vercel Authentication özelliğini etkinleştirin."
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
    fn flags_continue_on_admin_route() {
        let json =
            r#"{"routes":[{"src":"/admin(.*)","dest":"/admin/$1","continue":true}]}"#;
        assert!(analyze(json).iter().any(|v| v.id == "MDLW_01"));
    }

    #[test]
    fn no_flag_for_continue_on_public_route() {
        let json =
            r#"{"routes":[{"src":"/public(.*)","dest":"/public/$1","continue":true}]}"#;
        assert!(!analyze(json).iter().any(|v| v.id == "MDLW_01"));
    }

    #[test]
    fn flags_unguarded_sensitive_destination() {
        let json = r#"{"routes":[{"src":"/(.*)","dest":"/admin/$1"}]}"#;
        assert!(analyze(json).iter().any(|v| v.id == "MDLW_02"));
    }

    #[test]
    fn no_flag_when_has_condition_present() {
        let json = r#"{"routes":[{"src":"/(.*)","dest":"/admin/$1","has":[{"type":"header","key":"x-auth"}]}]}"#;
        assert!(!analyze(json).iter().any(|v| v.id == "MDLW_02"));
    }

    #[test]
    fn handles_empty_input() {
        assert_eq!(analyze("").len(), 0);
    }

    #[test]
    fn handles_no_routes_key() {
        assert_eq!(analyze(r#"{"redirects":[]}"#).len(), 0);
    }
}
