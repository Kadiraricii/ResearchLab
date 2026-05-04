use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Kullanım: {} <next_config_js_dosyasi>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).unwrap_or_default();

    println!("Scanning {} for source map config...", file_path);
    
    if contents.contains("productionBrowserSourceMaps: true") {
        println!("[TEHLIKE] Source map ifşası aktif! (productionBrowserSourceMaps: true)");
    } else {
        println!("[OK] Source map sızıntı riski tespit edilmedi.");
    }
}
