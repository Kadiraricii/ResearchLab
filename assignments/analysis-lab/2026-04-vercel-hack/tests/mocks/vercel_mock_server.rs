/// Mock Vercel API server built with wiremock.
/// Provides deterministic mock responses for all Vercel API endpoints used in tests.
use std::collections::HashMap;

/// Returns mock response bodies keyed by endpoint path.
/// In a real setup, you would start a `wiremock::MockServer` and mount these.
pub fn get_mock_responses() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    map.insert(
        "/v9/projects",
        r#"{
            "projects": [
                {"id": "prj_mock01", "name": "mock-project", "framework": "nextjs", "createdAt": 1699000000000, "updatedAt": 1699100000000}
            ],
            "pagination": {"count": 1, "next": null, "prev": null}
        }"#,
    );

    map.insert(
        "/v13/deployments",
        r#"{
            "deployments": [
                {"uid": "dpl_mock01", "name": "mock-project", "url": "mock-abc123.vercel.app", "state": "READY", "createdAt": 1699000000000}
            ],
            "pagination": {"count": 1, "next": null, "prev": null}
        }"#,
    );

    map.insert(
        "/v9/projects/prj_mock01/env",
        r#"{
            "envs": [
                {"id": "env_01", "key": "NEXT_PUBLIC_API_KEY", "type": "encrypted", "target": ["production"]},
                {"id": "env_02", "key": "DATABASE_URL",        "type": "encrypted", "target": ["production"]}
            ]
        }"#,
    );

    map.insert(
        "/v9/domains",
        r#"{
            "domains": [
                {"name": "mock-project.example.com", "createdAt": 1699000000000, "verified": true}
            ]
        }"#,
    );

    // Error response mocks
    map.insert("401", r#"{"error": {"code": "forbidden", "message": "Authentication required"}}"#);
    map.insert("403", r#"{"error": {"code": "forbidden", "message": "Access denied"}}"#);
    map.insert("429", r#"{"error": {"code": "rate_limited", "message": "Too many requests"}}"#);
    map.insert("500", r#"{"error": {"code": "server_error", "message": "Internal server error"}}"#);

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_server_has_projects_endpoint() {
        let responses = get_mock_responses();
        assert!(responses.contains_key("/v9/projects"));
    }

    #[test]
    fn mock_server_has_deployments_endpoint() {
        let responses = get_mock_responses();
        assert!(responses.contains_key("/v13/deployments"));
    }

    #[test]
    fn mock_server_has_error_responses() {
        let responses = get_mock_responses();
        assert!(responses.contains_key("401"));
        assert!(responses.contains_key("429"));
    }

    #[test]
    fn projects_response_is_valid_json() {
        let responses = get_mock_responses();
        let json_str = responses["/v9/projects"];
        let result: Result<serde_json::Value, _> = serde_json::from_str(json_str);
        assert!(result.is_ok(), "projects mock response must be valid JSON");
    }
}
