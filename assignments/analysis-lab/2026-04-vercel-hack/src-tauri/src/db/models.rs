use serde::{Deserialize, Serialize};

// ─── Scan ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scan {
    pub id: String,
    pub created_at: String,
    pub score: f64,
    pub finding_count: i64,
    pub vercel_json: Option<String>,
    pub next_config: Option<String>,
    pub env_content: Option<String>,
}

// ─── Finding ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub scan_id: String,
    pub title: String,
    pub description: String,
    pub risk_level: String,
    pub affected_component: String,
    pub remediation: String,
}

// ─── Project ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub vercel_id: Option<String>,
    pub added_at: String,
}

// ─── Report ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub id: String,
    pub scan_id: Option<String>,
    pub created_at: String,
    pub format: String, // "json" | "html"
    pub content: String,
}

// ─── Remediation ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Remediation {
    pub id: String,
    pub scan_id: Option<String>,
    pub created_at: String,
    pub applied_at: Option<String>,
    pub description: String,
    pub status: String, // "pending" | "applied" | "skipped"
}
