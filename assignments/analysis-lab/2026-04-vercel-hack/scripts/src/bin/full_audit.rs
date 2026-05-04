use std::process::Command;

fn main() {
    println!("=== TAM GÜVENLİK DENETİMİ (FULL AUDIT) BAŞLIYOR ===");

    // Env scan
    println!("\n[1] Environment Variables Taraniyor...");
    let _ = Command::new("cargo")
        .args(["run", "--bin", "scan_env", "--", "../.env"])
        .status();

    // Headers scan
    println!("\n[2] Security Headers Taraniyor...");
    let _ = Command::new("cargo")
        .args(["run", "--bin", "check_headers", "--", "../configs/vercel.json"])
        .status();

    // Source maps scan
    println!("\n[3] Source Maps Taraniyor...");
    let _ = Command::new("cargo")
        .args(["run", "--bin", "source_map_check", "--", "../configs/next.config.js"])
        .status();

    // DNS check
    println!("\n[4] DNS Takeover Taraniyor...");
    let _ = Command::new("cargo")
        .args(["run", "--bin", "dns_check"])
        .status();

    println!("\n=== DENETİM TAMAMLANDI ===");
}
