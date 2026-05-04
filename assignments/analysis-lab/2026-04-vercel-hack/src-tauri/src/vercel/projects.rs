use crate::error::{AppError, Result};
use super::{client::VercelClient, types::VercelProject};

/// Fetch all projects accessible by the configured token.
/// Vercel returns: `{ "projects": [...], "pagination": { ... } }`
pub async fn list_projects(client: &VercelClient) -> Result<Vec<VercelProject>> {
    let raw = client.get("/v9/projects", &[("limit", "100")]).await?;

    let array = raw["projects"].as_array().ok_or_else(|| {
        AppError::Analysis("Missing 'projects' field in Vercel API response".to_string())
    })?;

    serde_json::from_value(serde_json::Value::Array(array.clone())).map_err(AppError::Json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_minimal_project() {
        let json = serde_json::json!({
            "id": "prj_abc123",
            "name": "my-app"
        });
        let project: VercelProject = serde_json::from_value(json).unwrap();
        assert_eq!(project.id, "prj_abc123");
        assert_eq!(project.name, "my-app");
        assert!(project.framework.is_none());
    }

    #[test]
    fn deserializes_full_project() {
        let json = serde_json::json!({
            "id": "prj_abc123",
            "name": "my-app",
            "accountId": "acct_1",
            "framework": "nextjs",
            "createdAt": 1700000000000i64
        });
        let project: VercelProject = serde_json::from_value(json).unwrap();
        assert_eq!(project.framework.as_deref(), Some("nextjs"));
        assert!(project.created_at.is_some());
    }
}
