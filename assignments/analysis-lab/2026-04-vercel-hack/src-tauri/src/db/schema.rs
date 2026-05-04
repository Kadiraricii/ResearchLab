use rusqlite::Connection;
use crate::error::AppError;

// ─── Migration ────────────────────────────────────────────────────────────────
// Single-file schema; every CREATE is idempotent (IF NOT EXISTS).
// A lightweight version table lets us add future migrations without breaking
// existing databases.

const SCHEMA_VERSION: u32 = 1;

const CREATE_META: &str = "
CREATE TABLE IF NOT EXISTS _meta (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
);
";

const CREATE_SCANS: &str = "
CREATE TABLE IF NOT EXISTS scans (
  id            TEXT PRIMARY KEY,
  created_at    TEXT NOT NULL,
  score         REAL NOT NULL,
  finding_count INTEGER NOT NULL,
  vercel_json   TEXT,
  next_config   TEXT,
  env_content   TEXT
);
";

const CREATE_FINDINGS: &str = "
CREATE TABLE IF NOT EXISTS findings (
  id                 TEXT NOT NULL,
  scan_id            TEXT NOT NULL,
  title              TEXT NOT NULL,
  description        TEXT NOT NULL,
  risk_level         TEXT NOT NULL,
  affected_component TEXT NOT NULL,
  remediation        TEXT NOT NULL,
  FOREIGN KEY (scan_id) REFERENCES scans(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_findings_scan_id ON findings(scan_id);
CREATE INDEX IF NOT EXISTS idx_findings_risk_level ON findings(risk_level);
";

const CREATE_PROJECTS: &str = "
CREATE TABLE IF NOT EXISTS projects (
  id         TEXT PRIMARY KEY,
  name       TEXT NOT NULL,
  vercel_id  TEXT,
  added_at   TEXT NOT NULL
);
";

const CREATE_REPORTS: &str = "
CREATE TABLE IF NOT EXISTS reports (
  id         TEXT PRIMARY KEY,
  scan_id    TEXT,
  created_at TEXT NOT NULL,
  format     TEXT NOT NULL,
  content    TEXT NOT NULL,
  FOREIGN KEY (scan_id) REFERENCES scans(id) ON DELETE SET NULL
);
";

const CREATE_REMEDIATIONS: &str = "
CREATE TABLE IF NOT EXISTS remediations (
  id          TEXT PRIMARY KEY,
  scan_id     TEXT,
  created_at  TEXT NOT NULL,
  applied_at  TEXT,
  description TEXT NOT NULL,
  status      TEXT NOT NULL DEFAULT 'pending',
  FOREIGN KEY (scan_id) REFERENCES scans(id) ON DELETE SET NULL
);
";

/// Applies the schema and sets WAL mode for better concurrent read performance.
/// Safe to call on every startup — all statements are idempotent.
pub fn migrate(conn: &Connection) -> Result<(), AppError> {
    // Enable WAL + foreign keys once per connection
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

    // Create schema
    conn.execute_batch(CREATE_META)?;
    conn.execute_batch(CREATE_SCANS)?;
    conn.execute_batch(CREATE_FINDINGS)?;
    conn.execute_batch(CREATE_PROJECTS)?;
    conn.execute_batch(CREATE_REPORTS)?;
    conn.execute_batch(CREATE_REMEDIATIONS)?;

    // Record version (INSERT OR IGNORE — do not overwrite an existing value)
    conn.execute(
        "INSERT OR IGNORE INTO _meta (key, value) VALUES ('schema_version', ?1)",
        [SCHEMA_VERSION.to_string()],
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn migrate_is_idempotent() {
        let conn = Connection::open_in_memory().unwrap();
        migrate(&conn).unwrap();
        // Running twice must not fail
        migrate(&conn).unwrap();
    }

    #[test]
    fn schema_version_recorded() {
        let conn = Connection::open_in_memory().unwrap();
        migrate(&conn).unwrap();
        let v: String = conn
            .query_row(
                "SELECT value FROM _meta WHERE key = 'schema_version'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(v, SCHEMA_VERSION.to_string());
    }
}
