use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Kullanım: {} <cikis_dizini>", args[0]);
        std::process::exit(1);
    }

    let out_dir = &args[1];
    let _ = fs::create_dir_all(out_dir);

    let json_data = r#"{
        "status": "success",
        "score": 8.5,
        "vulnerabilities": [
            { "id": "ENV_LEAK", "severity": "HIGH", "message": "NEXT_PUBLIC_SECRET bulundu" }
        ]
    }"#;

    let json_path = format!("{}/rapor.json", out_dir);
    fs::write(&json_path, json_data).expect("JSON raporu yazılamadı");
    println!("[OK] JSON raporu oluşturuldu: {}", json_path);

    let html_data = format!(
        r#"<!DOCTYPE html>
<html>
<head><title>Güvenlik Raporu</title><style>body {{ font-family: sans-serif; padding: 20px; }} .high {{ color: red; }}</style></head>
<body>
    <h1>Vercel Güvenlik Tarama Raporu</h1>
    <h2>Skor: 8.5 / 10.0</h2>
    <ul>
        <li class="high">[HIGH] ENV_LEAK: NEXT_PUBLIC_SECRET bulundu</li>
    </ul>
</body>
</html>"#
    );

    let html_path = format!("{}/rapor.html", out_dir);
    fs::write(&html_path, html_data).expect("HTML raporu yazılamadı");
    println!("[OK] HTML raporu oluşturuldu: {}", html_path);
    
    println!("[UYARI] PDF rapor oluşturma (opsiyonel) şu an atlandı.");
}
