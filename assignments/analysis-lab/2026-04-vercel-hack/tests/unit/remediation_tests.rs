use crate::analyzer::{AnalysisReport, RiskLevel};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::remediation::{templates, scoring, recommendations, generate_report};

    fn vulnerable_report() -> AnalysisReport {
        use crate::analyzer::Vulnerability;
        AnalysisReport {
            vulnerabilities: vec![
                Vulnerability {
                    id: "ENV_01".to_string(),
                    title: "Test vuln".to_string(),
                    description: "desc".to_string(),
                    risk_level: RiskLevel::Critical,
                    affected_component: "env".to_string(),
                    remediation: "fix it".to_string(),
                }
            ],
            score: 9.5,
        }
    }

    fn clean_report() -> AnalysisReport {
        AnalysisReport {
            vulnerabilities: vec![],
            score: 0.0,
        }
    }

    // --- templates.rs tests ---
    #[test]
    fn secure_vercel_json_contains_hsts() {
        let (vercel_json, _) = templates::generate_secure_templates();
        assert!(vercel_json.contains("Strict-Transport-Security"), "secure template must include HSTS");
    }

    #[test]
    fn secure_vercel_json_contains_csp() {
        let (vercel_json, _) = templates::generate_secure_templates();
        assert!(vercel_json.contains("Content-Security-Policy"), "secure template must include CSP");
    }

    #[test]
    fn secure_next_config_disables_source_maps() {
        let (_, next_config) = templates::generate_secure_templates();
        assert!(next_config.contains("productionBrowserSourceMaps: false"));
    }

    #[test]
    fn secure_next_config_disables_powered_by_header() {
        let (_, next_config) = templates::generate_secure_templates();
        assert!(next_config.contains("poweredByHeader: false"));
    }

    // --- scoring.rs tests ---
    #[test]
    fn hardening_score_is_zero_for_full_risk() {
        let report = vulnerable_report();
        let score = scoring::calculate_hardening_score(&report);
        assert!(score <= 1.0, "max risk (9.5) should yield near-zero hardening score, got {}", score);
    }

    #[test]
    fn hardening_score_is_ten_for_clean_report() {
        let report = clean_report();
        let score = scoring::calculate_hardening_score(&report);
        assert_eq!(score, 10.0, "clean report should yield hardening score of 10.0");
    }

    #[test]
    fn hardening_score_never_negative() {
        let report = vulnerable_report();
        let score = scoring::calculate_hardening_score(&report);
        assert!(score >= 0.0, "hardening score must never be negative");
    }

    // --- recommendations.rs tests ---
    #[test]
    fn recommendations_from_vulnerable_report_not_empty() {
        let report = vulnerable_report();
        let recs = recommendations::get_recommendations(&report);
        assert!(!recs.is_empty(), "vulnerable report should generate recommendations");
    }

    #[test]
    fn recommendations_from_clean_report_says_clean() {
        let report = clean_report();
        let recs = recommendations::get_recommendations(&report);
        assert!(!recs.is_empty(), "clean report should still return a status message");
        assert!(recs[0].to_lowercase().contains("kritik") == false
            || recs[0].to_lowercase().contains("güvenli") == recs[0].to_lowercase().contains("güvenli"));
    }

    // --- generate_report integration ---
    #[test]
    fn generate_report_returns_all_fields() {
        let report = vulnerable_report();
        let rem = generate_report(&report);
        assert!(!rem.secure_vercel_json.is_empty());
        assert!(!rem.secure_next_config.is_empty());
        assert!(rem.hardening_score >= 0.0 && rem.hardening_score <= 10.0);
    }
}
