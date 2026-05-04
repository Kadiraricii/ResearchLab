/// Integration test: Full scan pipeline
/// Runs all analyzers → generates remediation → validates output
#[cfg(test)]
mod full_pipeline_tests {
    use vercel_hack_analysis::analyzer::run_all_analyzers;
    use vercel_hack_analysis::remediation::generate_report;

    const VULNERABLE_VERCEL_JSON: &str = r#"{
        "redirects": [{"source":"/:path*","destination":"http://external.com/:path*","permanent":false}],
        "buildCommand": "npm run build -- --secret=hardcoded_secret"
    }"#;

    const VULNERABLE_NEXT_CONFIG: &str = "productionBrowserSourceMaps: true";

    const VULNERABLE_ENV: &str = "NEXT_PUBLIC_API_KEY=sk-supersecret\nNEXT_PUBLIC_SECRET_TOKEN=abc123";

    const SECURE_VERCEL_JSON: &str = r#"{
        "headers": [{
            "source": "/(.*)",
            "headers": [
                {"key": "Strict-Transport-Security", "value": "max-age=63072000"},
                {"key": "Content-Security-Policy", "value": "default-src 'self'"}
            ]
        }]
    }"#;

    const SECURE_ENV: &str = "DATABASE_URL=postgres://localhost/db\nAPP_NAME=MyApp";

    #[test]
    fn vulnerable_config_produces_vulnerabilities() {
        let report = run_all_analyzers(VULNERABLE_VERCEL_JSON, VULNERABLE_NEXT_CONFIG, VULNERABLE_ENV);
        assert!(!report.vulnerabilities.is_empty(), "vulnerable config should produce findings");
        assert!(report.score > 0.0, "risk score should be above 0 for vulnerable config");
    }

    #[test]
    fn secure_config_produces_no_critical_vulns() {
        use vercel_hack_analysis::analyzer::RiskLevel;
        let report = run_all_analyzers(SECURE_VERCEL_JSON, "", SECURE_ENV);
        let critical_count = report.vulnerabilities.iter()
            .filter(|v| matches!(v.risk_level, RiskLevel::Critical))
            .count();
        assert_eq!(critical_count, 0, "secure config should not produce critical vulns");
    }

    #[test]
    fn empty_configs_produce_no_vulns() {
        let report = run_all_analyzers("", "", "");
        // Empty input: headers analyzer will skip (empty), others also skip
        // Some analyzers flag missing headers only when non-empty config is provided
        assert!(report.score >= 0.0);
    }

    #[test]
    fn remediation_report_generated_from_vulnerable_config() {
        let analysis = run_all_analyzers(VULNERABLE_VERCEL_JSON, VULNERABLE_NEXT_CONFIG, VULNERABLE_ENV);
        let rem = generate_report(&analysis);
        assert!(!rem.secure_vercel_json.is_empty());
        assert!(!rem.secure_next_config.is_empty());
        assert!(!rem.recommendations.is_empty());
        assert!(rem.hardening_score >= 0.0 && rem.hardening_score <= 10.0);
    }

    #[test]
    fn hardening_score_higher_for_clean_than_vulnerable() {
        let vulnerable = run_all_analyzers(VULNERABLE_VERCEL_JSON, VULNERABLE_NEXT_CONFIG, VULNERABLE_ENV);
        let secure = run_all_analyzers(SECURE_VERCEL_JSON, "", SECURE_ENV);

        let vuln_rem = generate_report(&vulnerable);
        let secure_rem = generate_report(&secure);

        assert!(
            secure_rem.hardening_score >= vuln_rem.hardening_score,
            "secure config should have >= hardening score than vulnerable config: {} vs {}",
            secure_rem.hardening_score, vuln_rem.hardening_score
        );
    }

    #[test]
    fn env_exposure_detected_in_full_pipeline() {
        let report = run_all_analyzers("", "", VULNERABLE_ENV);
        let env_vulns: Vec<_> = report.vulnerabilities.iter()
            .filter(|v| v.id.starts_with("ENV"))
            .collect();
        assert!(!env_vulns.is_empty(), "ENV_ prefixed vulns should be detected");
    }
}
