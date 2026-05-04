use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    db::Database,
    error::{AppError, Result},
    vercel::VercelClient,
};

/// Shared application state managed by Tauri.
///
/// Design rationale:
/// - `Arc<RwLock<Option<Arc<VercelClient>>>>` gives concurrent reads (multiple
///   scans checking the token) and exclusive writes (set_vercel_token).
/// - `Database` is wrapped in `Mutex` (via its own internal field) and in an
///   `Arc` so commands can cheaply clone the reference without cloning the DB.
pub struct AppState {
    client: Arc<RwLock<Option<Arc<VercelClient>>>>,
    pub db: Arc<Database>,
}

impl AppState {
    pub fn new(db: Database) -> Self {
        Self {
            client: Arc::new(RwLock::new(None)),
            db: Arc::new(db),
        }
    }

    /// Replace the current Vercel client with a new one using the given token.
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
