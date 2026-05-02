use crate::analyzer::AnalysisReport;

pub fn calculate_hardening_score(report: &AnalysisReport) -> f64 {
    let penalty = report.score; // risk score
    let hardening = 10.0 - penalty;
    if hardening < 0.0 {
        0.0
    } else {
        hardening
    }
}
