use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Kullanım: {} <.env_dosyasi>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Dosya okunamadı!");

    println!("Scanning {} for sensitive NEXT_PUBLIC_ variables...", file_path);
    
    let sensitive_keywords = ["SECRET", "TOKEN", "PASSWORD", "KEY"];
    let mut found = false;

    for line in contents.lines() {
        if line.starts_with("NEXT_PUBLIC_") {
            let upper_line = line.to_uppercase();
            for kw in sensitive_keywords {
                if upper_line.contains(kw) {
                    println!("[TEHLIKE] Hassas olabilecek public degisken tespit edildi: {}", line.split('=').next().unwrap_or(""));
                    found = true;
                }
            }
        }
    }

    if !found {
        println!("[OK] Görünürde hassas public değişken bulunamadı.");
    }
}
