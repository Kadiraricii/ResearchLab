use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{error::{AppError, Result}, vercel::VercelClient};

/// Shared application state managed by Tauri.
///
/// Design rationale:
/// - `Arc<RwLock<Option<Arc<VercelClient>>>>` provides:
///   - Multiple concurrent readers (active scans checking client existence)
///   - Single exclusive writer (set_vercel_token replacing the client)
///   - None state before any token is configured
///   - Inner Arc<VercelClient> so commands can clone the client out of the lock
///     and hold it beyond the RwLock guard's lifetime
///
/// - We use `tokio::sync::RwLock` (NOT `std::sync::RwLock`) because Tauri commands
///   are async functions; holding a std MutexGuard across an `.await` point
///   would fail to compile (the guard is not Send).
pub struct AppState {
    client: Arc<RwLock<Option<Arc<VercelClient>>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            client: Arc::new(RwLock::new(None)),
        }
    }

    /// Replace the current Vercel client with a new one using the given token.
    /// The old client (and its cache) is dropped when this function returns.
    pub async fn set_token(&self, token: String) -> Result<()> {
        let new_client = VercelClient::new(token)?;
        let mut guard = self.client.write().await;
        *guard = Some(Arc::new(new_client));
        Ok(())
    }

    /// Get the current client, returning an error if no token has been configured.
    pub async fn get_client(&self) -> Result<Arc<VercelClient>> {
        let guard = self.client.read().await;
        guard.clone().ok_or(AppError::NotConfigured)
    }

    /// Check whether a token has been set without returning the client.
    pub async fn is_configured(&self) -> bool {
        self.client.read().await.is_some()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
