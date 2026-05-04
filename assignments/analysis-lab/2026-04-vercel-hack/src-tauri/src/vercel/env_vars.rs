use crate::error::{AppError, Result};
use super::{client::VercelClient, types::VercelEnvVar};

/// Fetch environment variables for a project.
/// Vercel returns: `{ "envs": [...] }`
///
/// NOTE: Values of sensitive/encrypted variables are redacted by the API
/// (the `value` field will be None or empty). Only non-sensitive plain vars
/// have readable values in this response.
pub async fn list_env_vars(
    client: &VercelClient,
    project_id: &str,
) -> Result<Vec<VercelEnvVar>> {
    let path = format!("/v10/projects/{project_id}/env");
    let raw = client.get(&path, &[]).await?;

    let array = raw["envs"].as_array().ok_or_else(|| {
        AppError::Analysis("Missing 'envs' field in Vercel API response".to_string())
    })?;

    serde_json::from_value(serde_json::Value::Array(array.clone())).map_err(AppError::Json)
}

/// Convert a list of env vars into a KEY=VALUE string format
/// suitable for feeding into the existing string-based analyzers.
/// Sensitive vars (no readable value) are included as KEY= (empty value)
/// so analyzers can still detect dangerous key names like NEXT_PUBLIC_SECRET.
pub fn to_env_file_content(vars: &[VercelEnvVar]) -> String {
    vars.iter()
        .map(|v| {
            let val = v.value.as_deref().unwrap_or("");
            format!("{}={}", v.key, val)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_plain_env_var() {
        let json = serde_json::json!({
            "id": "env_abc",
            "key": "DATABASE_URL",
            "type": "plain",
            "target": ["production"],
            "value": "postgres://localhost/mydb"
        });
        let var: VercelEnvVar = serde_json::from_value(json).unwrap();
        assert_eq!(var.key, "DATABASE_URL");
        assert_eq!(var.value.as_deref(), Some("postgres://localhost/mydb"));
    }

    #[test]
    fn deserializes_sensitive_env_var_without_value() {
        let json = serde_json::json!({
            "id": "env_xyz",
            "key": "STRIPE_SECRET_KEY",
            "type": "sensitive",
            "sensitive": true
        });
        let var: VercelEnvVar = serde_json::from_value(json).unwrap();
        assert_eq!(var.sensitive, Some(true));
        assert!(var.value.is_none());
    }

    #[test]
    fn converts_vars_to_env_file_format() {
        let vars = vec![
            VercelEnvVar {
                id: "1".to_string(),
                key: "NEXT_PUBLIC_API_KEY".to_string(),
                env_type: None,
                target: None,
                git_branch: None,
                sensitive: None,
                value: Some("sk-test-123".to_string()),
                updated_at: None,
            },
            VercelEnvVar {
                id: "2".to_string(),
                key: "DATABASE_URL".to_string(),
                env_type: Some("sensitive".to_string()),
                target: None,
                git_branch: None,
                sensitive: Some(true),
                value: None,
                updated_at: None,
            },
        ];

        let content = to_env_file_content(&vars);
        assert!(content.contains("NEXT_PUBLIC_API_KEY=sk-test-123"));
        assert!(content.contains("DATABASE_URL="));
    }
}
