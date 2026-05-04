use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

// ─── Pagination ───────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Pagination {
    pub count: u32,
    pub next: Option<u64>,
    pub prev: Option<u64>,
}

// ─── Project Types ────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VercelProject {
    pub id: String,
    pub name: String,
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<i64>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
    pub framework: Option<String>,
    #[serde(rename = "nodeVersion")]
    pub node_version: Option<String>,
    pub link: Option<serde_json::Value>,
    #[serde(rename = "latestDeployments")]
    pub latest_deployments: Option<Vec<DeploymentRef>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeploymentRef {
    pub id: String,
    pub url: Option<String>,
    pub state: Option<String>,
    pub target: Option<String>,
}

// ─── Deployment Types ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VercelDeployment {
    pub id: String,
    pub url: Option<String>,
    pub name: String,
    pub state: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<i64>,
    #[serde(rename = "readyAt")]
    pub ready_at: Option<i64>,
    pub target: Option<String>,
    #[serde(rename = "inspectorUrl")]
    pub inspector_url: Option<String>,
    pub meta: Option<std::collections::HashMap<String, String>>,
}

// ─── Domain Types ─────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VercelDomain {
    pub name: String,
    pub verified: Option<bool>,
    #[serde(rename = "serviceType")]
    pub service_type: Option<String>,
    pub nameservers: Option<Vec<String>>,
    #[serde(rename = "intendedNameservers")]
    pub intended_nameservers: Option<Vec<String>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<i64>,
    #[serde(rename = "expiresAt")]
    pub expires_at: Option<i64>,
}

// ─── Environment Variable Types ───────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VercelEnvVar {
    pub id: String,
    pub key: String,
    #[serde(rename = "type")]
    pub env_type: Option<String>,
    /// Deployment targets: ["production", "preview", "development"]
    pub target: Option<Vec<String>>,
    #[serde(rename = "gitBranch")]
    pub git_branch: Option<String>,
    /// true for sensitive/encrypted vars whose value is redacted in the API response
    pub sensitive: Option<bool>,
    /// Value is None for sensitive vars (API redacts them)
    pub value: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
}

// ─── Internal Cache Entry ─────────────────────────────────────────────────────

/// Caches a serialized JSON string alongside its expiry instant.
/// Storing as String avoids type-erasure complications with Box<dyn Any>.
pub struct CacheEntry {
    pub json: String,
    pub expires_at: Instant,
}

impl CacheEntry {
    pub fn new(json: String, ttl_secs: u64) -> Self {
        Self {
            json,
            expires_at: Instant::now() + Duration::from_secs(ttl_secs),
        }
    }

    pub fn is_valid(&self) -> bool {
        Instant::now() < self.expires_at
    }
}
