use super::{RiskLevel, Vulnerability};

const SENSITIVE_BUILD_KEYWORDS: &[&str] = &[
    "SECRET", "TOKEN", "PASSWORD", "API_KEY", "PRIVATE_KEY", "AUTH",
];

pub fn analyze(vercel_json: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();

    if vercel_json.is_empty() {
        return vulns;
    }

    // Check build commands for hardcoded secrets
    let upper = vercel_json.to_uppercase();
    if upper.contains("\"BUILDCOMMAND\"") || upper.contains("BUILD_COMMAND") || upper.contains("INSTALLCOMMAND") {
        for kw in SENSITIVE_BUILD_KEYWORDS {
            if upper.contains(kw) {
                vulns.push(Vulnerability {
                    id: "BLD_01".to_string(),
                    title: "Build Komutunda Hassas Bilgi Tespiti".to_string(),
                    description: format!(
                        "Build yapılandırması içinde '{}' gibi hassas bir anahtar kelime tespit edildi. \
                         Bu bilgi build loglarına sızarak ifşa olabilir.",
                        kw
                    ),
                    risk_level: RiskLevel::High,
                    affected_component: "Build Pipeline".to_string(),
                    remediation:
                        "Hassas verileri doğrudan build komutlarına yazmayın. \
                         Bunun yerine Vercel Dashboard üzerinden 'Sensitive' olarak \
                         işaretlenmiş environment variable kullanın."
                            .to_string(),
                });
                // Only report once per config, avoid duplicate vulns
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
    fn detects_secret_in_build_command() {
        // Lowercase key but uppercase content — our analyzer upper-cases the whole string
        let json = r#"{"buildCommand":"npm run build -- --secret=my_secret_value"}"#;
        let vulns = analyze(json);
        assert!(vulns.iter().any(|v| v.id == "BLD_01"), "should detect secret in build cmd");
    }

    #[test]
    fn no_vuln_for_clean_build_command() {
        let json = r#"{"buildCommand":"npm run build"}"#;
        let vulns = analyze(json);
        assert_eq!(vulns.len(), 0);
    }

    #[test]
    fn empty_input_returns_no_vulns() {
        assert_eq!(analyze("").len(), 0);
    }

    #[test]
    fn no_build_key_returns_no_vuln() {
        let json = r#"{"headers":[]}"#;
        assert_eq!(analyze(json).len(), 0);
    }

    #[test]
    fn only_one_vuln_even_if_multiple_keywords_present() {
        // Multiple sensitive keywords in one build command should only yield 1 BLD_01
        let json = r#"{"buildCommand":"npm run build --secret=s1 --token=t1 --password=p1"}"#;
        let vulns = analyze(json);
        let bld_count = vulns.iter().filter(|v| v.id == "BLD_01").count();
        assert_eq!(bld_count, 1, "should produce at most one BLD_01 per config");
    }
}
