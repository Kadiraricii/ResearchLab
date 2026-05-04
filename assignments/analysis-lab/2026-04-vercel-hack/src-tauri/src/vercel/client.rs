use std::sync::Arc;
use std::time::Duration;

use dashmap::DashMap;
use reqwest::{Client, StatusCode};
use tokio::sync::Semaphore;

use crate::error::{AppError, Result};
use super::types::CacheEntry;

const BASE_URL: &str = "https://api.vercel.com";

/// Maximum concurrent in-flight HTTP requests.
/// Vercel's public rate limit is ~100 req/s; 10 concurrent is conservative and safe.
const MAX_CONCURRENT_REQUESTS: usize = 10;

/// Cache TTL in seconds. After this, entries are re-fetched from the API.
const CACHE_TTL_SECS: u64 = 60;

/// Maximum number of automatic retries on transient failures (5xx, 429).
const MAX_RETRIES: u32 = 3;

/// HTTP client for the Vercel REST API with:
/// - Semaphore-based concurrency limiting
/// - Exponential-backoff retry for 429 / 5xx responses
/// - TTL-based in-memory response cache (DashMap)
///
/// This struct is `Send + Sync` because:
/// - `reqwest::Client`: Send + Sync (guaranteed by reqwest)
/// - `String`: Send + Sync
/// - `Arc<Semaphore>`: Send + Sync
/// - `DashMap<_, _>`: Send + Sync (DashMap's core design guarantee)
pub struct VercelClient {
    http: Client,
    token: String,
    semaphore: Arc<Semaphore>,
    cache: DashMap<String, CacheEntry>,
}

impl VercelClient {
    /// Create a new client with the given Vercel API token.
    pub fn new(token: String) -> Result<Self> {
        let http = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("vercel-security-analyzer/0.1")
            .build()
            .map_err(AppError::Http)?;

        Ok(Self {
            http,
            token,
            semaphore: Arc::new(Semaphore::new(MAX_CONCURRENT_REQUESTS)),
            cache: DashMap::new(),
        })
    }

    /// Perform a GET request to `path` with optional query parameters.
    ///
    /// Returns the full response body as a `serde_json::Value` so callers can
    /// extract the specific field they need (e.g., `["projects"]`, `["envs"]`).
    ///
    /// Behavior:
    /// - Checks the in-memory cache first; returns cached value if still valid
    /// - Acquires a semaphore permit before sending (limits concurrency)
    /// - Retries on 429 (respecting `Retry-After` header) and 5xx (exponential backoff)
    /// - Caches successful responses for `CACHE_TTL_SECS` seconds
    pub async fn get(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<serde_json::Value> {
        // Build a deterministic cache key from path + query params
        let cache_key = build_cache_key(path, query);

        // Fast path: return cached response before acquiring the semaphore
        if let Some(entry) = self.cache.get(&cache_key) {
            if entry.is_valid() {
                return serde_json::from_str(&entry.json).map_err(AppError::Json);
            }
        }

        // Slow path: acquire permit then send HTTP request
        // `_permit` is held until this function returns, enforcing the concurrency limit.
        let _permit = self
            .semaphore
            .acquire()
            .await
            .map_err(|_| AppError::Analysis("Semaphore unexpectedly closed".to_string()))?;

        // Build the full URL with query params appended — all our params are ASCII,
        // so simple string concatenation is safe and avoids .query() compatibility issues.
        let url = build_url(BASE_URL, path, query);
        let mut attempt = 0u32;

        loop {
            let response = self
                .http
                .get(&url)
                .bearer_auth(&self.token)
                .send()
                .await
                .map_err(AppError::Http)?;

            let status = response.status();

            match status {
                StatusCode::OK => {
                    let text = response.text().await.map_err(AppError::Http)?;
                    // Validate JSON before caching
                    let value: serde_json::Value =
                        serde_json::from_str(&text).map_err(AppError::Json)?;
                    self.cache
                        .insert(cache_key, CacheEntry::new(text, CACHE_TTL_SECS));
                    return Ok(value);
                }

                StatusCode::UNAUTHORIZED => {
                    return Err(AppError::ApiError {
                        status: 401,
                        message: "Unauthorized — check your Vercel API token".to_string(),
                    });
                }

                StatusCode::FORBIDDEN => {
                    return Err(AppError::ApiError {
                        status: 403,
                        message: "Forbidden — insufficient token permissions".to_string(),
                    });
                }

                StatusCode::NOT_FOUND => {
                    return Err(AppError::ApiError {
                        status: 404,
                        message: format!("Resource not found: {path}"),
                    });
                }

                StatusCode::TOO_MANY_REQUESTS => {
                    let retry_after = parse_retry_after(&response);
                    if attempt >= MAX_RETRIES {
                        return Err(AppError::RateLimit {
                            retry_after_secs: retry_after,
                        });
                    }
                    // Cap wait at 60s to avoid excessive blocking in tests
                    let wait = Duration::from_secs(retry_after.min(60));
                    tokio::time::sleep(wait).await;
                }

                s if s.is_server_error() => {
                    if attempt >= MAX_RETRIES {
                        let body = response.text().await.unwrap_or_default();
                        return Err(AppError::ApiError {
                            status: s.as_u16(),
                            message: body,
                        });
                    }
                    // Exponential backoff: 1s → 2s → 4s
                    let backoff = Duration::from_secs(2u64.pow(attempt));
                    tokio::time::sleep(backoff).await;
                }

                s => {
                    let body = response.text().await.unwrap_or_default();
                    return Err(AppError::ApiError {
                        status: s.as_u16(),
                        message: body,
                    });
                }
            }

            attempt += 1;
        }
    }

    /// Invalidate all cache entries whose key contains `path`.
    pub fn invalidate_cache_for(&self, path: &str) {
        self.cache.retain(|k, _| !k.contains(path));
    }

    /// Clear the entire response cache (e.g., after a token change).
    pub fn clear_cache(&self) {
        self.cache.clear();
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn build_cache_key(path: &str, query: &[(&str, &str)]) -> String {
    build_url("", path, query)
}

/// Appends query params to a base URL. All our query values are ASCII (IDs, limits)
/// so percent-encoding is not required.
fn build_url(base: &str, path: &str, query: &[(&str, &str)]) -> String {
    if query.is_empty() {
        format!("{base}{path}")
    } else {
        let qs: String = query
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&");
        format!("{base}{path}?{qs}")
    }
}

fn parse_retry_after(response: &reqwest::Response) -> u64 {
    response
        .headers()
        .get("retry-after")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(60)
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_cache_key_without_query() {
        let key = build_cache_key("/v9/projects", &[]);
        assert_eq!(key, "/v9/projects");
    }

    #[test]
    fn builds_cache_key_with_query() {
        let key = build_cache_key("/v9/projects", &[("limit", "100"), ("teamId", "team_1")]);
        assert!(key.contains("limit=100"));
        assert!(key.contains("teamId=team_1"));
    }

    #[test]
    fn new_client_succeeds_with_any_token() {
        let client = VercelClient::new("test_token".to_string());
        assert!(client.is_ok());
    }
}
