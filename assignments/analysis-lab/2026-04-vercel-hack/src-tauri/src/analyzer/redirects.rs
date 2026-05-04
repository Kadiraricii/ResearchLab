use super::{RiskLevel, Vulnerability};

pub fn analyze(vercel_json: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();

    if vercel_json.is_empty() {
        return vulns;
    }

    // FIX: Parse JSON properly instead of string-matching across the entire document.
    // The old approach had false positives: two unrelated redirect objects where one
    // had an external destination and another had a wildcard source would both match.
    let Ok(val) = serde_json::from_str::<serde_json::Value>(vercel_json) else {
        return vulns;
    };

    let Some(redirects) = val.get("redirects").and_then(|r| r.as_array()) else {
        return vulns;
    };

    for redirect in redirects {
        let source = redirect
            .get("source")
            .and_then(|s| s.as_str())
            .unwrap_or("");
        let destination = redirect
            .get("destination")
            .and_then(|d| d.as_str())
            .unwrap_or("");

        // Open redirect: external URL that uses a user-controlled capture group.
        // Both conditions must be in the SAME redirect rule.
        let is_external_dest =
            destination.starts_with("http://") || destination.starts_with("https://");
        let source_has_wildcard = source.contains("(.*)") || source.contains("/:path*");
        let dest_uses_capture =
            destination.contains("$1") || destination.contains(":path*");

        if is_external_dest && source_has_wildcard && dest_uses_capture {
            vulns.push(Vulnerability {
                id: "REDIR_01".to_string(),
                title: "Olası Open Redirect Zafiyeti".to_string(),
                description: format!(
                    "Redirect kuralında dış URL'ye kullanıcı kontrollü yol yönlendirmesi tespit \
                     edildi. Source: '{source}' → Destination: '{destination}'"
                ),
                risk_level: RiskLevel::High,
                affected_component: "vercel.json Redirects".to_string(),
                remediation:
                    "Yönlendirmelerde kullanıcı girdisini (.*) doğrudan external destination \
                     olarak kullanmaktan kaçının. Hedef URL'yi sabit tutun."
                        .to_string(),
            });
        }
    }

    vulns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_open_redirect() {
        let json = r#"{"redirects":[{"source":"/go/(.*)", "destination":"https://external.com/$1"}]}"#;
        let vulns = analyze(json);
        assert_eq!(vulns.len(), 1);
    }

    #[test]
    fn no_false_positive_fixed_external_url() {
        // External destination but no user-controlled capture — NOT open redirect
        let json = r#"{"redirects":[{"source":"/old", "destination":"https://example.com/new"}]}"#;
        let vulns = analyze(json);
        assert_eq!(vulns.len(), 0);
    }

    #[test]
    fn no_false_positive_when_patterns_in_separate_rules() {
        // Old bug: would trigger because both patterns exist in the document,
        // even though they're in different rules.
        let json = r#"{
            "redirects":[
                {"source":"/fixed", "destination":"https://fixed.com"},
                {"source":"/api/(.*)", "destination":"/api-v2/$1"}
            ]
        }"#;
        let vulns = analyze(json);
        assert_eq!(vulns.len(), 0);
    }

    #[test]
    fn handles_empty_input() {
        assert_eq!(analyze("").len(), 0);
    }

    #[test]
    fn handles_invalid_json() {
        assert_eq!(analyze("{not: valid}").len(), 0);
    }
}
