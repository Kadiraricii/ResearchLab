use super::{RiskLevel, Vulnerability};

const SENSITIVE_KEYWORDS: &[&str] = &[
    "SECRET", "TOKEN", "PASSWORD", "KEY", "AUTH", "DB_URL", "PRIVATE",
    "CREDENTIAL", "API_SECRET", "SIGNING",
];

pub fn analyze(env_content: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();

    for line in env_content.lines() {
        let trimmed = line.trim();

        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let is_public = trimmed.starts_with("NEXT_PUBLIC_") || trimmed.starts_with("VITE_");
        if !is_public {
            continue;
        }

        let upper_line = trimmed.to_uppercase();
        let var_name = trimmed.split('=').next().unwrap_or("").trim();

        // BUG FIX: Use `.any()` so each line produces at most ONE vulnerability,
        // regardless of how many keywords it matches. Previously the inner loop
        // would push a duplicate entry for every matching keyword.
        if SENSITIVE_KEYWORDS.iter().any(|kw| upper_line.contains(kw)) {
            vulns.push(Vulnerability {
                id: "ENV_01".to_string(),
                title: "Hassas Veri İfşası (NEXT_PUBLIC_)".to_string(),
                description: format!(
                    "Frontend ortamında ifşa olabilecek riskli bir anahtar bulundu: {}",
                    var_name
                ),
                risk_level: RiskLevel::Critical,
                affected_component: "Environment Variables".to_string(),
                remediation:
                    "Sadece backend'de kullanılması gereken anahtarlardan NEXT_PUBLIC_ veya \
                     VITE_ ön ekini kaldırın."
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
    fn detects_next_public_secret() {
        let env = "NEXT_PUBLIC_SECRET_KEY=sk-1234";
        let vulns = analyze(env);
        assert_eq!(vulns.len(), 1, "should produce exactly one vuln per sensitive line");
    }

    #[test]
    fn does_not_duplicate_for_multiple_keywords() {
        // Line matches SECRET, KEY, TOKEN — must still produce 1 vuln
        let env = "NEXT_PUBLIC_SECRET_TOKEN_KEY=abc";
        let vulns = analyze(env);
        assert_eq!(vulns.len(), 1);
    }

    #[test]
    fn detects_vite_prefix() {
        let env = "VITE_API_TOKEN=my_token";
        let vulns = analyze(env);
        assert_eq!(vulns.len(), 1);
    }

    #[test]
    fn ignores_non_sensitive_public_var() {
        let env = "NEXT_PUBLIC_APP_NAME=MyApp";
        let vulns = analyze(env);
        assert_eq!(vulns.len(), 0);
    }

    #[test]
    fn ignores_non_public_sensitive_var() {
        let env = "DATABASE_SECRET=abc123";
        let vulns = analyze(env);
        assert_eq!(vulns.len(), 0, "non-public vars are fine to have sensitive names");
    }

    #[test]
    fn ignores_comments() {
        let env = "# NEXT_PUBLIC_SECRET_KEY=ignored";
        let vulns = analyze(env);
        assert_eq!(vulns.len(), 0);
    }

    #[test]
    fn counts_multiple_distinct_sensitive_lines() {
        let env = "NEXT_PUBLIC_API_KEY=k1\nNEXT_PUBLIC_SECRET=s1";
        let vulns = analyze(env);
        assert_eq!(vulns.len(), 2);
    }
}
