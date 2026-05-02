pub mod vercel_config;
pub mod next_config;
pub mod platform_defaults;

use tauri::command;

#[command]
pub fn parse_vercel_config(content: String) -> Result<vercel_config::VercelConfig, String> {
    vercel_config::parse(&content)
}

#[command]
pub fn parse_next_config(content: String) -> Result<next_config::NextConfigSummary, String> {
    next_config::analyze(&content)
}

#[command]
pub fn get_platform_defaults() -> platform_defaults::PlatformDefaults {
    platform_defaults::get_defaults()
}
