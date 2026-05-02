pub fn get_recommendations() -> Vec<String> {
    vec![
        "Environment Variables: NEXT_PUBLIC_ önekini sadece tamamen açık verilerde kullanın.".to_string(),
        "Headers: HSTS, CSP, X-Frame-Options, X-Content-Type-Options güvenlik başlıklarını mutlaka ekleyin.".to_string(),
        "Source Maps: productionBrowserSourceMaps değerini false olarak ayarlayın.".to_string(),
        "CORS: Wildcard (*) kullanmaktan kaçının, erişimi spesifik domainlerle sınırlandırın.".to_string(),
        "Serverless: Tüm /api rotalarınızda Zod vb. bir kütüphane ile girdi doğrulaması (Input Validation) yapın.".to_string()
    ]
}
