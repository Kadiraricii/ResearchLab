use proptest::prelude::*;

// Property-based tests using proptest
// These tests run the analyzers against random inputs to catch panics/crashes.

use vercel_hack_analysis::analyzer::{run_all_analyzers, scoring::calculate_cvss_like_score};

proptest! {
    /// Any random string should never cause env_exposure::analyze to panic
    /// #[ignore]: slow property-based test — run with `cargo test -- --ignored`
    #[test]
    #[ignore]
    fn env_exposure_never_panics(input in ".*") {
        use vercel_hack_analysis::analyzer::env_exposure;
        let _ = env_exposure::analyze(&input);
    }

    /// Any random string should never cause headers::analyze to panic
    #[test]
    #[ignore]
    fn headers_never_panics(input in ".*") {
        use vercel_hack_analysis::analyzer::headers;
        let _ = headers::analyze(&input);
    }

    /// Any random string should never cause cors::analyze to panic
    #[test]
    #[ignore]
    fn cors_never_panics(s1 in ".*", s2 in ".*") {
        use vercel_hack_analysis::analyzer::cors;
        let _ = cors::analyze(&s1, &s2);
    }

    /// Any random string should never cause source_maps::analyze to panic
    #[test]
    #[ignore]
    fn source_maps_never_panics(input in ".*") {
        use vercel_hack_analysis::analyzer::source_maps;
        let _ = source_maps::analyze(&input);
    }

    /// Full pipeline should never panic with any input combination
    #[test]
    #[ignore]
    fn full_pipeline_never_panics(vercel in ".*", next in ".*", env in ".*") {
        let report = run_all_analyzers(&vercel, &next, &env);
        // Score must always be in range [0, 10]
        prop_assert!(report.score >= 0.0);
        prop_assert!(report.score <= 10.0);
    }

    /// Risk score is always between 0 and 100 regardless of vuln count
    #[test]
    #[ignore]
    fn score_always_in_valid_range_for_arbitrary_vuln_count(count in 0usize..100) {
        use vercel_hack_analysis::analyzer::{Vulnerability, RiskLevel};
        let vulns: Vec<Vulnerability> = (0..count).map(|i| Vulnerability {
            id: format!("TEST_{}", i),
            title: "test".to_string(),
            description: "desc".to_string(),
            risk_level: RiskLevel::Critical,
            affected_component: "test".to_string(),
            remediation: "fix".to_string(),
        }).collect();
        let score = calculate_cvss_like_score(&vulns);
        prop_assert!(score >= 0.0, "score below 0: {}", score);
        prop_assert!(score <= 10.0, "score above 10: {}", score);
    }

    /// Empty vulnerability list always returns 0
    #[test]
    #[ignore]
    fn empty_vulns_always_returns_zero(_unused in 0..10i32) {
        let score = calculate_cvss_like_score(&[]);
        prop_assert_eq!(score, 0.0);
    }

    /// Any env content with "NEXT_PUBLIC_" prefix that contains a sensitive keyword
    /// should produce at least one vulnerability
    #[test]
    #[ignore]
    fn next_public_sensitive_always_detected(suffix in "[A-Z_]+", value in "[a-z0-9]+") {
        use vercel_hack_analysis::analyzer::env_exposure;
        let sensitive_keywords = ["SECRET", "TOKEN", "PASSWORD", "KEY", "AUTH"];
        for kw in sensitive_keywords {
            let env = format!("NEXT_PUBLIC_{}{}={}", kw, suffix, value);
            let vulns = env_exposure::analyze(&env);
            prop_assert!(!vulns.is_empty(),
                "NEXT_PUBLIC_{}{} should be flagged, got 0 vulns", kw, suffix);
        }
    }
}
