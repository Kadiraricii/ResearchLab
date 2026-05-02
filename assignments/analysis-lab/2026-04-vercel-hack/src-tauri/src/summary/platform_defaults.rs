use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PlatformDefaults {
    pub hsts_enabled_by_default: bool,
    pub csp_enabled_by_default: bool,
    pub default_powered_by_header: String,
    pub public_env_prefix: String,
    pub sourcemap_production_default: bool,
}

pub fn get_defaults() -> PlatformDefaults {
    PlatformDefaults {
        hsts_enabled_by_default: false,
        csp_enabled_by_default: false,
        default_powered_by_header: "X-Powered-By: Next.js".to_string(),
        public_env_prefix: "NEXT_PUBLIC_".to_string(),
        sourcemap_production_default: false, // Next.js 13+ behavior
    }
}
