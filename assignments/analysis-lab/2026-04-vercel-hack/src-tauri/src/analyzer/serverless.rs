use super::{RiskLevel, Vulnerability};

/// Node.js runtimes no longer receiving security updates.
const DEPRECATED_RUNTIMES: &[&str] = &["nodejs12.x", "nodejs14.x", "nodejs16.x"];

pub fn analyze(vercel_json: &str) -> Vec<Vulnerability> {
    let mut vulns = Vec::new();

    if vercel_json.is_empty() {
        return vulns;
    }

    let Ok(val) = serde_json::from_str::<serde_json::Value>(vercel_json) else {
        return vulns;
    };

    let Some(functions) = val.get("functions").and_then(|f| f.as_object()) else {
        return vulns;
    };

    for (func_name, func_config) in functions {
        // Check 1: maxDuration > 60s — DoS and billing abuse risk
        if let Some(max_dur) = func_config.get("maxDuration").and_then(|d| d.as_u64()) {
            if max_dur > 60 {
                vulns.push(Vulnerability {
                    id: "SRVL_01".to_string(),
                    title: "Aşırı Uzun Fonksiyon Süresi".to_string(),
                    description: format!(
                        "'{func_name}' fonksiyonunun maxDuration değeri {max_dur}s olarak \
                         ayarlanmış. Yüksek değerler DoS saldırılarına ve beklenmedik \
                         faturalara yol açabilir."
                    ),
                    risk_level: RiskLevel::Medium,
                    affected_component: "vercel.json Functions".to_string(),
                    remediation: "maxDuration değerini ihtiyaç duyulan minimuma (genellikle \
                                  ≤30s) indirin. Uzun süren işlemleri arka plan kuyruğuna \
                                  taşıyın."
                        .to_string(),
                });
            }
        }

        // Check 2: memory > 1024 MB — billing/DoS risk if not justified
        if let Some(memory) = func_config.get("memory").and_then(|m| m.as_u64()) {
            if memory > 1024 {
                vulns.push(Vulnerability {
                    id: "SRVL_02".to_string(),
                    title: "Yüksek Bellek Limiti".to_string(),
                    description: format!(
                        "'{func_name}' fonksiyonuna {memory} MB bellek ayrılmış. \
                         Gereğinden fazla bellek, DoS saldırısı ve aşırı faturalama \
                         riskini artırır."
                    ),
                    risk_level: RiskLevel::Low,
                    affected_component: "vercel.json Functions".to_string(),
                    remediation: "memory değerini fonksiyonun gerçek ihtiyacına göre \
                                  belirleyin. Varsayılan 1024 MB çoğu kullanım için \
                                  yeterlidir."
                        .to_string(),
                });
            }
        }

        // Check 3: deprecated Node.js runtime — no longer receives security patches
        if let Some(runtime) = func_config.get("runtime").and_then(|r| r.as_str()) {
            if DEPRECATED_RUNTIMES.iter().any(|dep| runtime.starts_with(dep)) {
                vulns.push(Vulnerability {
                    id: "SRVL_03".to_string(),
                    title: "Kullanımdan Kaldırılmış Runtime".to_string(),
                    description: format!(
                        "'{func_name}' fonksiyonu '{runtime}' runtime'ını kullanıyor. \
                         Bu sürüm güvenlik güncellemesi almamaktadır ve bilinen \
                         zafiyetler içerebilir."
                    ),
                    risk_level: RiskLevel::High,
                    affected_component: "vercel.json Functions".to_string(),
                    remediation:
                        "Runtime'ı aktif LTS sürümüne (nodejs20.x veya nodejs22.x) \
                         yükseltin."
                            .to_string(),
                });
            }
        }
    }

    vulns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_high_max_duration() {
        let json = r#"{"functions":{"api/heavy.js":{"maxDuration":300}}}"#;
        assert!(analyze(json).iter().any(|v| v.id == "SRVL_01"));
    }

    #[test]
    fn no_flag_for_acceptable_duration() {
        let json = r#"{"functions":{"api/fast.js":{"maxDuration":10}}}"#;
        assert!(!analyze(json).iter().any(|v| v.id == "SRVL_01"));
    }

    #[test]
    fn flags_high_memory() {
        let json = r#"{"functions":{"api/heavy.js":{"memory":3008}}}"#;
        assert!(analyze(json).iter().any(|v| v.id == "SRVL_02"));
    }

    #[test]
    fn no_flag_for_default_memory() {
        let json = r#"{"functions":{"api/fn.js":{"memory":1024}}}"#;
        assert!(!analyze(json).iter().any(|v| v.id == "SRVL_02"));
    }

    #[test]
    fn flags_deprecated_runtime() {
        let json = r#"{"functions":{"api/old.js":{"runtime":"nodejs14.x"}}}"#;
        assert!(analyze(json).iter().any(|v| v.id == "SRVL_03"));
    }

    #[test]
    fn no_flag_for_current_runtime() {
        let json = r#"{"functions":{"api/fn.js":{"runtime":"nodejs20.x"}}}"#;
        assert!(!analyze(json).iter().any(|v| v.id == "SRVL_03"));
    }

    #[test]
    fn handles_empty_input() {
        assert_eq!(analyze("").len(), 0);
    }

    #[test]
    fn handles_no_functions_key() {
        assert_eq!(analyze(r#"{"redirects":[]}"#).len(), 0);
    }
}
