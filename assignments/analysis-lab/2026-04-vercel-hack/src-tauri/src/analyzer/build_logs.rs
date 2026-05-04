use super::{RiskLevel, Vulnerability};

/// Lowercase patterns that indicate hardcoded secrets in build commands or env values.
const SECRET_PATTERNS: &[&str] = &[
    "sk-", "pk-", "token=", "secret=", "password=", "passwd=",
    "api_key=", "apikey=", "access_key=", "auth_token=", "bearer ",
    "private_key=", "client_secret=",
];

/// Lowercase key substrings that identify sensitive environment variable names.
const SENSITIVE_KEY_FRAGMENTS: &[&str] = &[
    "secret", "token", "password", "passwd", "api_key", "apikey",
    "private_key", "access_key", "auth",
];

pub fn analyze(vercel_json: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();

    if vercel_json.is_empty() {
        return vulns;
    }

    let Ok(val) = serde_json::from_str::<serde_json::Value>(vercel_json) else {
        return vulns;
    };

    // Check 1: buildCommand contains secret-like patterns
    if let Some(build_cmd) = val.get("buildCommand").and_then(|c| c.as_str()) {
        let lower = build_cmd.to_lowercase();
        if SECRET_PATTERNS.iter().any(|p| lower.contains(p)) {
            vulns.push(Vulnerability {
                id: "BLDL_01".to_string(),
                title: "Build Komutunda Gizli Bilgi Tespiti".to_string(),
                description: format!(
                    "buildCommand içinde gizli bilgi içerdiği düşünülen bir pattern \
                     tespit edildi. Build komutları log olarak saklanır ve yetkisiz \
                     kişilerce görülebilir: '{build_cmd}'"
                ),
                risk_level: RiskLevel::Critical,
                affected_component: "vercel.json buildCommand".to_string(),
                remediation:
                    "Gizli anahtarları asla build komutuna gömmeyın. \
                     Vercel Dashboard > Settings > Environment Variables bölümüne \
                     ekleyin ve komutta $SECRET_NAME şeklinde referans verin."
                        .to_string(),
            });
        }
    }

    // Check 2: env section contains hardcoded secret values (not @references)
    //
    // In vercel.json the `env` field is meant to hold @secret-reference syntax.
    // A plain string value (not starting with @) that has a sensitive-sounding key
    // means the secret is committed to the repository.
    if let Some(env_obj) = val.get("env").and_then(|e| e.as_object()) {
        for (key, value) in env_obj {
            let val_str = value.as_str().unwrap_or("");

            // Skip empty values and proper @-references
            if val_str.is_empty() || val_str.starts_with('@') {
                continue;
            }

            let lower_key = key.to_lowercase();
            let lower_val = val_str.to_lowercase();

            let key_is_sensitive =
                SENSITIVE_KEY_FRAGMENTS.iter().any(|f| lower_key.contains(f));
            let val_looks_like_secret =
                SECRET_PATTERNS.iter().any(|p| lower_val.starts_with(p));

            if key_is_sensitive || val_looks_like_secret {
                vulns.push(Vulnerability {
                    id: "BLDL_02".to_string(),
                    title: "vercel.json İçinde Hardcoded Gizli Değer".to_string(),
                    description: format!(
                        "vercel.json `env` bölümünde '{key}' değişkeni @referans \
                         yerine doğrudan bir değer içeriyor. Bu dosya sürüm \
                         kontrolüne dahilse gizli bilgi ifşa olur."
                    ),
                    risk_level: RiskLevel::High,
                    affected_component: "vercel.json env".to_string(),
                    remediation:
                        "Gizli değerleri vercel.json'dan kaldırın. \
                         Vercel Dashboard > Settings > Environment Variables \
                         bölümünü kullanın veya @secret-name referans sözdizimini \
                         tercih edin."
                            .to_string(),
                });
                // One vulnerability per env section is sufficient — avoid noise
                break;
            }
        }
    }

    vulns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_secret_in_build_command() {
        let json = r#"{"buildCommand":"npm run build -- --token=sk-abc123"}"#;
        assert!(analyze(json).iter().any(|v| v.id == "BLDL_01"));
    }

    #[test]
    fn no_flag_for_normal_build_command() {
        let json = r#"{"buildCommand":"npm run build"}"#;
        assert_eq!(analyze(json).len(), 0);
    }

    #[test]
    fn flags_hardcoded_secret_in_env() {
        let json = r#"{"env":{"API_SECRET_KEY":"hardcoded-plaintext-value"}}"#;
        assert!(analyze(json).iter().any(|v| v.id == "BLDL_02"));
    }

    #[test]
    fn no_flag_for_env_reference() {
        let json = r#"{"env":{"API_KEY":"@my-secret-ref"}}"#;
        assert_eq!(analyze(json).len(), 0);
    }

    #[test]
    fn no_flag_for_non_sensitive_env_key() {
        let json = r#"{"env":{"APP_NAME":"my-app"}}"#;
        assert_eq!(analyze(json).len(), 0);
    }

    #[test]
    fn handles_empty_input() {
        assert_eq!(analyze("").len(), 0);
    }

    #[test]
    fn handles_invalid_json() {
        assert_eq!(analyze("{bad json}").len(), 0);
    }
}
