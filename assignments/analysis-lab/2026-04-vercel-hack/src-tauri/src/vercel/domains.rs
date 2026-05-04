use crate::error::{AppError, Result};
use super::{client::VercelClient, types::VercelDomain};

/// Fetch all domains registered on the account.
/// Vercel returns: `{ "domains": [...], "pagination": { ... } }`
pub async fn list_domains(client: &VercelClient) -> Result<Vec<VercelDomain>> {
    let raw = client.get("/v5/domains", &[("limit", "100")]).await?;

    let array = raw["domains"].as_array().ok_or_else(|| {
        AppError::Analysis("Missing 'domains' field in Vercel API response".to_string())
    })?;

    serde_json::from_value(serde_json::Value::Array(array.clone())).map_err(AppError::Json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_domain() {
        let json = serde_json::json!({
            "name": "example.com",
            "verified": true,
            "serviceType": "external"
        });
        let domain: VercelDomain = serde_json::from_value(json).unwrap();
        assert_eq!(domain.name, "example.com");
        assert_eq!(domain.verified, Some(true));
    }

    #[test]
    fn handles_unverified_domain() {
        let json = serde_json::json!({
            "name": "staging.example.com",
            "verified": false
        });
        let domain: VercelDomain = serde_json::from_value(json).unwrap();
        assert_eq!(domain.verified, Some(false));
        assert!(domain.nameservers.is_none());
    }
}
