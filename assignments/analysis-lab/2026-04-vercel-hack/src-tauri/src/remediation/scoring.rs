use crate::analyzer::{AnalysisReport, RiskLevel};

/// Calculate a hardening score in the range [0, 10].
///
/// Higher score = better hardened system.
///
/// FIX: The previous implementation used `10.0 - risk_score`, which lost
/// resolution when risk_score was capped at 10.0 (e.g., systems with 1 Critical
/// and 20 Criticals both showed hardening = 0.0, making them indistinguishable).
///
/// New approach: deduct per-issue penalties directly from each vulnerability's
/// count by severity. This preserves resolution even for highly compromised systems.
pub fn calculate_hardening_score(report: &AnalysisReport) -> f64 {
    let critical = count_by_level(report, &RiskLevel::Critical);
    let high = count_by_level(report, &RiskLevel::High);
    let medium = count_by_level(report, &RiskLevel::Medium);
    let low = count_by_level(report, &RiskLevel::Low);

    let penalty = (critical as f64 * 3.0)
        + (high as f64 * 1.5)
        + (medium as f64 * 0.5)
        + (low as f64 * 0.1);

    (10.0_f64 - penalty).max(0.0)
}

fn count_by_level(report: &AnalysisReport, target: &RiskLevel) -> usize {
    report
        .vulnerabilities
        .iter()
        .filter(|v| std::mem::discriminant(&v.risk_level) == std::mem::discriminant(target))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzer::{AnalysisReport, Vulnerability};

    fn report_with(levels: &[RiskLevel]) -> AnalysisReport {
        let vulnerabilities = levels
            .iter()
            .enumerate()
            .map(|(i, level)| Vulnerability {
                id: i.to_string(),
                title: "test".to_string(),
                description: "desc".to_string(),
                risk_level: level.clone(),
                affected_component: "test".to_string(),
                remediation: "fix".to_string(),
            })
            .collect();
        AnalysisReport {
            vulnerabilities,
            score: 0.0,
        }
    }

    #[test]
    fn clean_system_scores_ten() {
        let report = report_with(&[]);
        assert_eq!(calculate_hardening_score(&report), 10.0);
    }

    #[test]
    fn one_critical_reduces_score() {
        let report = report_with(&[RiskLevel::Critical]);
        assert_eq!(calculate_hardening_score(&report), 7.0); // 10 - 3
    }

    #[test]
    fn multiple_criticals_compound() {
        let report = report_with(&[RiskLevel::Critical, RiskLevel::Critical, RiskLevel::Critical]);
        assert_eq!(calculate_hardening_score(&report), 1.0); // 10 - 9
    }

    #[test]
    fn many_criticals_floor_at_zero() {
        let levels: Vec<_> = std::iter::repeat(RiskLevel::Critical).take(10).collect();
        let report = report_with(&levels);
        assert_eq!(calculate_hardening_score(&report), 0.0);
    }

    #[test]
    fn mixed_levels_compound_correctly() {
        // 1 Critical (3.0) + 1 High (1.5) + 1 Medium (0.5) = 5.0 penalty → score 5.0
        let report = report_with(&[RiskLevel::Critical, RiskLevel::High, RiskLevel::Medium]);
        assert_eq!(calculate_hardening_score(&report), 5.0);
    }
}
