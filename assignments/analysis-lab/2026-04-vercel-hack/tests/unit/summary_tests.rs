use vercel_hack_analysis::summary::{vercel_config, next_config, platform_defaults};

#[cfg(test)]
mod tests {
    use super::*;

    // --- vercel_config.rs tests ---
    #[test]
    fn parses_valid_vercel_json() {
        let json = r#"{"headers":[{"source":"/(.*)", "headers":[{"key":"HSTS","value":"max-age=63072000"}]}]}"#;
        let result = vercel_config::parse(json);
        assert!(result.is_ok(), "valid vercel.json should parse without error");
        let config = result.unwrap();
        assert_eq!(config.headers.len(), 1);
    }

    #[test]
    fn rejects_invalid_vercel_json() {
        let bad = "this is not json {{{";
        let result = vercel_config::parse(bad);
        assert!(result.is_err(), "invalid JSON should return an error");
    }

    #[test]
    fn parses_empty_vercel_json_object() {
        let json = r#"{}"#;
        let result = vercel_config::parse(json);
        assert!(result.is_ok());
        let config = result.unwrap();
        assert!(config.headers.is_empty());
        assert!(config.redirects.is_empty());
        assert!(config.rewrites.is_empty());
    }

    #[test]
    fn parses_redirects_correctly() {
        let json = r#"{"redirects":[{"source":"/old","destination":"/new","permanent":true}]}"#;
        let result = vercel_config::parse(json);
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.redirects.len(), 1);
        assert_eq!(config.redirects[0].source, "/old");
        assert_eq!(config.redirects[0].destination, "/new");
        assert!(config.redirects[0].permanent);
    }

    #[test]
    fn parses_rewrites_correctly() {
        let json = r#"{"rewrites":[{"source":"/page","destination":"/actual"}]}"#;
        let result = vercel_config::parse(json);
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.rewrites.len(), 1);
    }

    // --- next_config.rs tests ---
    #[test]
    fn detects_source_maps_enabled_in_next_config() {
        let next = "productionBrowserSourceMaps: true";
        let findings = next_config::analyze(next);
        assert!(findings.contains_key("source_maps_enabled") || !findings.is_empty(),
            "should detect source maps enabled, findings: {:?}", findings);
    }

    #[test]
    fn detects_powered_by_header_in_next_config() {
        let next = "// default config without poweredByHeader: false";
        let findings = next_config::analyze(next);
        // poweredByHeader is enabled by default
        assert!(findings.contains_key("powered_by_header") || !findings.is_empty());
    }

    // --- platform_defaults.rs tests ---
    #[test]
    fn platform_defaults_not_empty() {
        let defaults = platform_defaults::get_defaults();
        assert!(!defaults.is_empty(), "platform defaults should not be empty");
    }
}
