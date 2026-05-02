use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VercelConfig {
    #[serde(default)]
    pub headers: Vec<HeaderRule>,
    #[serde(default)]
    pub redirects: Vec<RedirectRule>,
    #[serde(default)]
    pub rewrites: Vec<RewriteRule>,
    #[serde(default)]
    pub crons: Vec<CronJob>,
    pub clean_urls: Option<bool>,
    pub trailing_slash: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderRule {
    pub source: String,
    pub headers: Vec<HeaderKeyValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderKeyValue {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedirectRule {
    pub source: String,
    pub destination: String,
    #[serde(default)]
    pub permanent: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RewriteRule {
    pub source: String,
    pub destination: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CronJob {
    pub path: String,
    pub schedule: String,
}

pub fn parse(content: &str) -> Result<VercelConfig, String> {
    serde_json::from_str(content).map_err(|e| format!("Failed to parse vercel.json: {}", e))
}
