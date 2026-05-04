use super::{RiskLevel, Vulnerability};

/// Risk weight per severity level (CVSS base score reference ranges).
const WEIGHT_CRITICAL: f64 = 9.5;
const WEIGHT_HIGH: f64 = 7.5;
const WEIGHT_MEDIUM: f64 = 5.0;
const WEIGHT_LOW: f64 = 2.5;
const MAX_SCORE: f64 = 10.0;

/// Maximum bonus added by additional high-severity issues.
const MAX_SEVERITY_BONUS: f64 = 1.5;

/// Per-issue bonus for each additional high/critical issue beyond the first.
const BONUS_PER_ISSUE: f64 = 0.25;

/// Calculate a CVSS-like risk score in the range [0, 10].
///
/// Algorithm:
/// 1. The base score equals the worst single vulnerability's weight.
/// 2. Each additional Critical/High issue adds a diminishing bonus.
/// 3. The total is capped at 10.0.
///
/// This differs from the naive summation approach which loses resolution once
/// the total exceeds 10.0 (e.g., 2 Criticals and 20 Criticals both becoming
/// indistinguishable at 10.0).
pub fn calculate_cvss_like_score(vulns: &[Vulnerability]) -> f64 {
    if vulns.is_empty() {
        return 0.0;
    }

    // Step 1: Base score = highest single vulnerability weight
    let base = vulns
        .iter()
        .map(vulnerability_weight)
        .fold(0.0_f64, f64::max);

    // Step 2: Bonus for each additional high-severity issue
    let high_severity_count = vulns
        .iter()
        .filter(|v| matches!(v.risk_level, RiskLevel::Critical | RiskLevel::High))
        .count();

    let bonus = if high_severity_count > 1 {
        ((high_severity_count - 1) as f64 * BONUS_PER_ISSUE).min(MAX_SEVERITY_BONUS)
    } else {
        0.0
    };

    (base + bonus).min(MAX_SCORE)
}

fn vulnerability_weight(v: &Vulnerability) -> f64 {
    match v.risk_level {
        RiskLevel::Critical => WEIGHT_CRITICAL,
        RiskLevel::High => WEIGHT_HIGH,
        RiskLevel::Medium => WEIGHT_MEDIUM,
        RiskLevel::Low => WEIGHT_LOW,
        RiskLevel::Info => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzer::Vulnerability;

    fn vuln(id: &str, level: RiskLevel) -> Vulnerability {
        Vulnerability {
            id: id.to_string(),
            title: "Test".to_string(),
            description: "desc".to_string(),
            risk_level: level,
            affected_component: "test".to_string(),
            remediation: "fix it".to_string(),
        }
    }

    #[test]
    fn empty_vulns_returns_zero() {
        assert_eq!(calculate_cvss_like_score(&[]), 0.0);
    }

    #[test]
    fn single_critical_returns_base_weight() {
        let vulns = vec![vuln("C1", RiskLevel::Critical)];
        assert_eq!(calculate_cvss_like_score(&vulns), 9.5);
    }

    #[test]
    fn single_medium_returns_medium_weight() {
        let vulns = vec![vuln("M1", RiskLevel::Medium)];
        assert_eq!(calculate_cvss_like_score(&vulns), 5.0);
    }

    #[test]
    fn multiple_criticals_add_bonus_and_cap_at_ten() {
        let vulns = vec![
            vuln("C1", RiskLevel::Critical),
            vuln("C2", RiskLevel::Critical),
            vuln("C3", RiskLevel::Critical),
        ];
        let score = calculate_cvss_like_score(&vulns);
        assert!(score > 9.5, "more criticals should score higher than one");
        assert!(score <= 10.0, "score must not exceed 10.0");
    }

    #[test]
    fn low_only_returns_low_weight() {
        let vulns = vec![vuln("L1", RiskLevel::Low), vuln("L2", RiskLevel::Low)];
        // Low vulns don't trigger the high-severity bonus
        let score = calculate_cvss_like_score(&vulns);
        assert_eq!(score, 2.5);
    }

    #[test]
    fn score_is_always_in_range() {
        let vulns: Vec<_> = (0..50).map(|i| vuln(&i.to_string(), RiskLevel::Critical)).collect();
        let score = calculate_cvss_like_score(&vulns);
        assert!((0.0..=10.0).contains(&score));
    }
}
