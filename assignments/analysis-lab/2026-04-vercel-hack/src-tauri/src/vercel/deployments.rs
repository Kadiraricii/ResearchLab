use crate::error::{AppError, Result};
use super::{client::VercelClient, types::VercelDeployment};

/// Fetch recent deployments for a project.
/// Vercel returns: `{ "deployments": [...], "pagination": { ... } }`
pub async fn list_deployments(
    client: &VercelClient,
    project_id: &str,
) -> Result<Vec<VercelDeployment>> {
    let raw = client
        .get(
            "/v6/deployments",
            &[("projectId", project_id), ("limit", "20")],
        )
        .await?;

    let array = raw["deployments"].as_array().ok_or_else(|| {
        AppError::Analysis("Missing 'deployments' field in Vercel API response".to_string())
    })?;

    serde_json::from_value(serde_json::Value::Array(array.clone())).map_err(AppError::Json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_minimal_deployment() {
        let json = serde_json::json!({
            "id": "dpl_abc123",
            "name": "my-app",
            "url": "my-app-abc123.vercel.app"
        });
        let dep: VercelDeployment = serde_json::from_value(json).unwrap();
        assert_eq!(dep.id, "dpl_abc123");
        assert_eq!(dep.url.as_deref(), Some("my-app-abc123.vercel.app"));
    }

    #[test]
    fn handles_missing_optional_fields() {
        let json = serde_json::json!({
            "id": "dpl_xyz",
            "name": "test-app"
        });
        let dep: VercelDeployment = serde_json::from_value(json).unwrap();
        assert!(dep.state.is_none());
        assert!(dep.target.is_none());
    }
}
