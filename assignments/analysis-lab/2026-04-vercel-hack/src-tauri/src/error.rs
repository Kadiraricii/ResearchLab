use thiserror::Error;

/// Application-wide error type.
/// Must implement `serde::Serialize` because Tauri commands return `Result<T, AppError>`.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Rate limit exceeded, retry after {retry_after_secs}s")]
    RateLimit { retry_after_secs: u64 },

    #[error("Vercel API error {status}: {message}")]
    ApiError { status: u16, message: String },

    #[error("Client not configured — call set_vercel_token first")]
    NotConfigured,

    #[error("Analysis error: {0}")]
    Analysis(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Timeout waiting for API response")]
    Timeout,

    #[error("Database error: {0}")]
    Db(#[from] rusqlite::Error),
}

// Tauri requires errors in commands to be Serialize.
// thiserror fields (like `#[from] reqwest::Error`) are NOT Serialize themselves,
// so we implement it manually by serializing as a { kind, message } object.
// NOTE: Must use std::result::Result here — our `Result<T>` alias would clash.
impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("AppError", 2)?;
        s.serialize_field("kind", self.kind_str())?;
        s.serialize_field("message", &self.to_string())?;
        s.end()
    }
}

impl AppError {
    fn kind_str(&self) -> &'static str {
        match self {
            AppError::Http(_) => "Http",
            AppError::Json(_) => "Json",
            AppError::RateLimit { .. } => "RateLimit",
            AppError::ApiError { .. } => "ApiError",
            AppError::NotConfigured => "NotConfigured",
            AppError::Analysis(_) => "Analysis",
            AppError::Io(_) => "Io",
            AppError::Timeout => "Timeout",
            AppError::Db(_) => "Db",
        }
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
