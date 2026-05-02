use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NextConfigSummary {
    pub powered_by_header: Option<bool>,
    pub has_headers_function: bool,
    pub has_redirects_function: bool,
    pub has_rewrites_function: bool,
    pub remote_patterns_found: bool,
}

pub fn analyze(content: &str) -> Result<NextConfigSummary, String> {
    // Basic regex-based or string-matching analysis for demonstration
    // In a real scenario, an AST parser like SWC would be safer.
    
    let mut summary = NextConfigSummary::default();
    
    if content.contains("poweredByHeader: false") {
        summary.powered_by_header = Some(false);
    } else if content.contains("poweredByHeader: true") {
        summary.powered_by_header = Some(true);
    }
    
    summary.has_headers_function = content.contains("async headers()");
    summary.has_redirects_function = content.contains("async redirects()");
    summary.has_rewrites_function = content.contains("async rewrites()");
    summary.remote_patterns_found = content.contains("remotePatterns");
    
    Ok(summary)
}
