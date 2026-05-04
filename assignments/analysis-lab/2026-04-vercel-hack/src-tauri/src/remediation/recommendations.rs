use crate::analyzer::AnalysisReport;

pub fn get_recommendations(report: &AnalysisReport) -> Vec<String> {
    let mut recs = Vec::new();
    
    if report.vulnerabilities.is_empty() {
        recs.push("Harika! Sisteminizde bilinen kritik zafiyet bulunamadı. Genel güvenlik standartlarını korumaya devam edin.".to_string());
        return recs;
    }

    for vuln in &report.vulnerabilities {
        recs.push(format!("[{}] {}", vuln.id, vuln.remediation));
    }
    
    recs
}
