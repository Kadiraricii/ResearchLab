use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Kullanım: {} <vercel_json_dosyasi>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).unwrap_or_default();

    println!("Scanning {} for security headers...", file_path);
    
    let required_headers = [
        "Strict-Transport-Security",
        "X-Frame-Options",
        "X-Content-Type-Options",
        "Content-Security-Policy"
    ];

    for header in required_headers {
        if !contents.contains(header) {
            println!("[UYARI] Eksik güvenlik başlığı tespit edildi: {}", header);
        }
    }
}
