use rusqlite::{Connection, params};
use crate::error::AppError;
use super::models::{Finding, Project, Remediation, Report, Scan};

// ─── Scans ────────────────────────────────────────────────────────────────────

pub fn insert_scan(conn: &Connection, scan: &Scan) -> Result<(), AppError> {
    conn.execute(
        "INSERT INTO scans (id, created_at, score, finding_count, vercel_json, next_config, env_content)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            scan.id,
            scan.created_at,
            scan.score,
            scan.finding_count,
            scan.vercel_json,
            scan.next_config,
            scan.env_content,
        ],
    )?;
    Ok(())
}

pub fn get_scan(conn: &Connection, id: &str) -> Result<Option<Scan>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, created_at, score, finding_count, vercel_json, next_config, env_content
         FROM scans WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map([id], |row| {
        Ok(Scan {
            id:            row.get(0)?,
            created_at:    row.get(1)?,
            score:         row.get(2)?,
            finding_count: row.get(3)?,
            vercel_json:   row.get(4)?,
            next_config:   row.get(5)?,
            env_content:   row.get(6)?,
        })
    })?;
    Ok(rows.next().transpose()?)
}

pub fn list_scans(conn: &Connection) -> Result<Vec<Scan>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, created_at, score, finding_count, vercel_json, next_config, env_content
         FROM scans ORDER BY created_at DESC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(Scan {
            id:            row.get(0)?,
            created_at:    row.get(1)?,
            score:         row.get(2)?,
            finding_count: row.get(3)?,
            vercel_json:   row.get(4)?,
            next_config:   row.get(5)?,
            env_content:   row.get(6)?,
        })
    })?;
    rows.map(|r| r.map_err(AppError::from)).collect()
}

pub fn delete_scan(conn: &Connection, id: &str) -> Result<(), AppError> {
    conn.execute("DELETE FROM scans WHERE id = ?1", [id])?;
    Ok(())
}

// ─── Findings ─────────────────────────────────────────────────────────────────

pub fn insert_findings(conn: &Connection, findings: &[Finding]) -> Result<(), AppError> {
    let mut stmt = conn.prepare(
        "INSERT INTO findings (id, scan_id, title, description, risk_level, affected_component, remediation)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
    )?;
    for f in findings {
        stmt.execute(params![
            f.id,
            f.scan_id,
            f.title,
            f.description,
            f.risk_level,
            f.affected_component,
            f.remediation,
        ])?;
    }
    Ok(())
}

pub fn get_findings_for_scan(conn: &Connection, scan_id: &str) -> Result<Vec<Finding>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, scan_id, title, description, risk_level, affected_component, remediation
         FROM findings WHERE scan_id = ?1 ORDER BY
         CASE risk_level WHEN 'Critical' THEN 0 WHEN 'High' THEN 1 WHEN 'Medium' THEN 2 WHEN 'Low' THEN 3 ELSE 4 END",
    )?;
    let rows = stmt.query_map([scan_id], |row| {
        Ok(Finding {
            id:                 row.get(0)?,
            scan_id:            row.get(1)?,
            title:              row.get(2)?,
            description:        row.get(3)?,
            risk_level:         row.get(4)?,
            affected_component: row.get(5)?,
            remediation:        row.get(6)?,
        })
    })?;
    rows.map(|r| r.map_err(AppError::from)).collect()
}

// ─── Projects ─────────────────────────────────────────────────────────────────

pub fn insert_project(conn: &Connection, project: &Project) -> Result<(), AppError> {
    conn.execute(
        "INSERT OR IGNORE INTO projects (id, name, vercel_id, added_at) VALUES (?1, ?2, ?3, ?4)",
        params![project.id, project.name, project.vercel_id, project.added_at],
    )?;
    Ok(())
}

pub fn list_projects(conn: &Connection) -> Result<Vec<Project>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, name, vercel_id, added_at FROM projects ORDER BY added_at DESC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(Project {
            id:        row.get(0)?,
            name:      row.get(1)?,
            vercel_id: row.get(2)?,
            added_at:  row.get(3)?,
        })
    })?;
    rows.map(|r| r.map_err(AppError::from)).collect()
}

pub fn delete_project(conn: &Connection, id: &str) -> Result<(), AppError> {
    conn.execute("DELETE FROM projects WHERE id = ?1", [id])?;
    Ok(())
}

// ─── Reports ──────────────────────────────────────────────────────────────────

pub fn insert_report(conn: &Connection, report: &Report) -> Result<(), AppError> {
    conn.execute(
        "INSERT INTO reports (id, scan_id, created_at, format, content) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![report.id, report.scan_id, report.created_at, report.format, report.content],
    )?;
    Ok(())
}

pub fn list_reports(conn: &Connection) -> Result<Vec<Report>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, scan_id, created_at, format, content FROM reports ORDER BY created_at DESC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(Report {
            id:         row.get(0)?,
            scan_id:    row.get(1)?,
            created_at: row.get(2)?,
            format:     row.get(3)?,
            content:    row.get(4)?,
        })
    })?;
    rows.map(|r| r.map_err(AppError::from)).collect()
}

pub fn delete_report(conn: &Connection, id: &str) -> Result<(), AppError> {
    conn.execute("DELETE FROM reports WHERE id = ?1", [id])?;
    Ok(())
}

// ─── Remediations ─────────────────────────────────────────────────────────────

pub fn insert_remediation(conn: &Connection, rem: &Remediation) -> Result<(), AppError> {
    conn.execute(
        "INSERT INTO remediations (id, scan_id, created_at, applied_at, description, status)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![rem.id, rem.scan_id, rem.created_at, rem.applied_at, rem.description, rem.status],
    )?;
    Ok(())
}

pub fn update_remediation_status(
    conn: &Connection,
    id: &str,
    status: &str,
    applied_at: Option<&str>,
) -> Result<(), AppError> {
    conn.execute(
        "UPDATE remediations SET status = ?1, applied_at = ?2 WHERE id = ?3",
        params![status, applied_at, id],
    )?;
    Ok(())
}

pub fn list_remediations_for_scan(
    conn: &Connection,
    scan_id: &str,
) -> Result<Vec<Remediation>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, scan_id, created_at, applied_at, description, status
         FROM remediations WHERE scan_id = ?1 ORDER BY created_at ASC",
    )?;
    let rows = stmt.query_map([scan_id], |row| {
        Ok(Remediation {
            id:          row.get(0)?,
            scan_id:     row.get(1)?,
            created_at:  row.get(2)?,
            applied_at:  row.get(3)?,
            description: row.get(4)?,
            status:      row.get(5)?,
        })
    })?;
    rows.map(|r| r.map_err(AppError::from)).collect()
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::schema::migrate;
    use rusqlite::Connection;

    fn test_conn() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        migrate(&conn).unwrap();
        conn
    }

    #[test]
    fn scan_crud() {
        let conn = test_conn();
        let scan = Scan {
            id:            "scan-1".to_string(),
            created_at:    "2026-05-05T00:00:00Z".to_string(),
            score:         7.5,
            finding_count: 3,
            vercel_json:   Some("{}".to_string()),
            next_config:   None,
            env_content:   None,
        };
        insert_scan(&conn, &scan).unwrap();

        let fetched = get_scan(&conn, "scan-1").unwrap().unwrap();
        assert_eq!(fetched.score, 7.5);
        assert_eq!(fetched.finding_count, 3);

        let all = list_scans(&conn).unwrap();
        assert_eq!(all.len(), 1);

        delete_scan(&conn, "scan-1").unwrap();
        assert!(get_scan(&conn, "scan-1").unwrap().is_none());
    }

    #[test]
    fn findings_insert_and_query() {
        let conn = test_conn();
        let scan = Scan {
            id: "s1".to_string(),
            created_at: "2026-05-05T00:00:00Z".to_string(),
            score: 5.0,
            finding_count: 2,
            vercel_json: None,
            next_config: None,
            env_content: None,
        };
        insert_scan(&conn, &scan).unwrap();

        let findings = vec![
            Finding {
                id: "f1".to_string(),
                scan_id: "s1".to_string(),
                title: "CSP Missing".to_string(),
                description: "No CSP header".to_string(),
                risk_level: "High".to_string(),
                affected_component: "headers".to_string(),
                remediation: "Add CSP".to_string(),
            },
            Finding {
                id: "f2".to_string(),
                scan_id: "s1".to_string(),
                title: "NEXT_PUBLIC_ exposure".to_string(),
                description: "Token leaked".to_string(),
                risk_level: "Critical".to_string(),
                affected_component: "env".to_string(),
                remediation: "Remove token".to_string(),
            },
        ];
        insert_findings(&conn, &findings).unwrap();

        let result = get_findings_for_scan(&conn, "s1").unwrap();
        assert_eq!(result.len(), 2);
        // Critical comes first (ORDER BY risk sort)
        assert_eq!(result[0].risk_level, "Critical");
    }

    #[test]
    fn project_crud() {
        let conn = test_conn();
        let p = Project {
            id: "p1".to_string(),
            name: "my-app".to_string(),
            vercel_id: Some("vercel_abc".to_string()),
            added_at: "2026-05-05T00:00:00Z".to_string(),
        };
        insert_project(&conn, &p).unwrap();
        let all = list_projects(&conn).unwrap();
        assert_eq!(all.len(), 1);
        delete_project(&conn, "p1").unwrap();
        assert_eq!(list_projects(&conn).unwrap().len(), 0);
    }

    #[test]
    fn report_crud() {
        let conn = test_conn();
        let r = Report {
            id: "r1".to_string(),
            scan_id: None,
            created_at: "2026-05-05T00:00:00Z".to_string(),
            format: "json".to_string(),
            content: r#"{"score":0}"#.to_string(),
        };
        insert_report(&conn, &r).unwrap();
        let all = list_reports(&conn).unwrap();
        assert_eq!(all.len(), 1);
        delete_report(&conn, "r1").unwrap();
        assert_eq!(list_reports(&conn).unwrap().len(), 0);
    }

    #[test]
    fn remediation_status_update() {
        let conn = test_conn();
        let rem = Remediation {
            id: "rem-1".to_string(),
            scan_id: None,
            created_at: "2026-05-05T00:00:00Z".to_string(),
            applied_at: None,
            description: "Add HSTS header".to_string(),
            status: "pending".to_string(),
        };
        insert_remediation(&conn, &rem).unwrap();
        update_remediation_status(&conn, "rem-1", "applied", Some("2026-05-05T01:00:00Z")).unwrap();

        let all = list_remediations_for_scan(&conn, "no-scan").unwrap();
        assert_eq!(all.len(), 0); // wrong scan_id → 0

        // Re-query without scan filter
        let mut stmt = conn
            .prepare("SELECT status FROM remediations WHERE id = 'rem-1'")
            .unwrap();
        let status: String = stmt.query_row([], |r| r.get(0)).unwrap();
        assert_eq!(status, "applied");
    }
}
