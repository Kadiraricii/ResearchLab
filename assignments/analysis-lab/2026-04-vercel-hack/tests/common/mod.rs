//! Shared test utilities (Phase 10.2)
//!
//! Fixtures are loaded once per test binary using `std::sync::OnceLock`
//! (stable since Rust 1.70) — eliminates repeated disk I/O across tests.

use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

// ─── Lazy fixture cache ───────────────────────────────────────────────────────
// Each fixture is read from disk exactly once, then shared across all tests in
// the same binary. This replaces `lazy_static!` with the std-only `OnceLock`.

static VULNERABLE_VERCEL: OnceLock<String> = OnceLock::new();
static SECURE_VERCEL:     OnceLock<String> = OnceLock::new();
static VALID_PROJECTS:    OnceLock<String> = OnceLock::new();
static VALID_DEPLOYMENTS: OnceLock<String> = OnceLock::new();
static MIXED_FINDINGS:    OnceLock<String> = OnceLock::new();

fn fixture_dir() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests");
    p.push("fixtures");
    p
}

fn read_fixture(name: &str) -> String {
    let path = fixture_dir().join(name);
    fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Fixture not found: {}", path.display()))
}

// ─── Public fixture accessors ─────────────────────────────────────────────────

pub fn vulnerable_vercel_json() -> &'static str {
    VULNERABLE_VERCEL.get_or_init(|| read_fixture("vulnerable_vercel.json"))
}

pub fn secure_vercel_json() -> &'static str {
    SECURE_VERCEL.get_or_init(|| read_fixture("secure_vercel.json"))
}

pub fn valid_projects_json() -> &'static str {
    VALID_PROJECTS.get_or_init(|| read_fixture("valid_projects.json"))
}

pub fn valid_deployments_json() -> &'static str {
    VALID_DEPLOYMENTS.get_or_init(|| read_fixture("valid_deployments.json"))
}

pub fn mixed_findings_json() -> &'static str {
    MIXED_FINDINGS.get_or_init(|| read_fixture("mixed_findings.json"))
}

/// Load any fixture by name (falls back to per-call disk read for ad-hoc fixtures).
pub fn load_fixture(name: &str) -> String {
    read_fixture(name)
}

// ─── In-memory database factory ───────────────────────────────────────────────

/// Create a clean in-memory SQLite database for each test.
/// Uses the real schema migration so tests exercise the production schema.
pub fn test_db() -> vercel_hack_analysis::db::Database {
    vercel_hack_analysis::db::Database::open_in_memory()
        .expect("in-memory DB must open in tests")
}

// ─── Assertion helpers ────────────────────────────────────────────────────────

pub mod assert {
    use vercel_hack_analysis::analyzer::AnalysisReport;

    /// Assert that the report contains a finding with the given ID.
    pub fn has_vuln(report: &AnalysisReport, vuln_id: &str) {
        assert!(
            report.vulnerabilities.iter().any(|v| v.id == vuln_id),
            "Expected finding {vuln_id} in report, but it was not present.\n\
             Found: {:?}",
            report.vulnerabilities.iter().map(|v| &v.id).collect::<Vec<_>>()
        );
    }

    /// Assert that the report does NOT contain a finding with the given ID.
    pub fn no_vuln(report: &AnalysisReport, vuln_id: &str) {
        assert!(
            !report.vulnerabilities.iter().any(|v| v.id == vuln_id),
            "Expected finding {vuln_id} to be absent, but it was present."
        );
    }

    /// Assert the report risk score is within [min, max].
    pub fn score_in_range(report: &AnalysisReport, min: f64, max: f64) {
        assert!(
            report.score >= min && report.score <= max,
            "Expected score in [{min}, {max}], got {}",
            report.score
        );
    }
}
