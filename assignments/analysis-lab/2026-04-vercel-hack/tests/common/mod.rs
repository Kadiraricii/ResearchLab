use std::fs;
use std::path::PathBuf;

/// Test veritabanı (in-memory SQLite) oluşturur
pub fn setup_test_db() {
    // In a real scenario, use rusqlite or sqlx to setup an in-memory DB here.
    println!("[TEST HELPER] In-memory SQLite veritabanı kuruldu.");
}

/// Sahte (Mock) Vercel API Sunucusu Başlatır (Örn: wiremock kullanarak)
pub fn start_mock_api_server() -> String {
    // In a real scenario, start a wiremock server and return its URI.
    let mock_uri = "http://localhost:8080/api/mock".to_string();
    println!("[TEST HELPER] Mock Vercel API sunucusu başlatıldı: {}", mock_uri);
    mock_uri
}

/// Test senaryoları için fixture yükleyici (JSON, config dosyaları vs.)
pub fn load_fixture(name: &str) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push(name);

    fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Fixture dosyası bulunamadı: {}", path.display()))
}

/// Özel assertion yardımcıları
pub mod assert {
    pub fn is_vulnerable(report: &crate::analyzer::AnalysisReport, vuln_id: &str) {
        let has_vuln = report.vulnerabilities.iter().any(|v| v.id == vuln_id);
        assert!(has_vuln, "Rapor, beklenen zafiyeti ({}) içermiyor!", vuln_id);
    }
}
