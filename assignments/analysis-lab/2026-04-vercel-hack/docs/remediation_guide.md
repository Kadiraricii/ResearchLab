# Vercel Sertleştirme ve Düzeltme Rehberi

Zafiyet analiz motorumuzun tespit ettiği zafiyetleri kapatmak için uygulamanız gereken adım adım sertleştirme prosedürleri aşağıda listelenmiştir.

## 1. Environment Variable Güvenliği
- **Kural 1:** Hiçbir zaman şifre (password), token veya API key gibi hassas verilerinizi `NEXT_PUBLIC_` ön eki ile adlandırmayın. Bu prefix, değişkenin doğrudan frontend JavaScript kodlarına (tarayıcıya) gönderilmesine neden olur.
- **Kural 2:** Vercel Dashboard üzerinden (Project > Settings > Environment Variables) tüm hassas değişkenleri eklerken sağ taraftaki ayarlardan **"Sensitive"** kutucuğunu işaretleyin. Bu sayede bu değişkenler build loglarında maskelenecektir.

## 2. HTTP Güvenlik Başlıkları Yapılandırması
`vercel.json` veya `next.config.js` dosyanızda varsayılan olarak bulunmayan aşağıdaki güvenlik başlıklarını (headers) tanımlamalısınız:
- `Strict-Transport-Security` (HSTS): Tüm trafiğin sadece HTTPS üzerinden akmasını zorunlu kılar. (`max-age=63072000; includeSubDomains; preload`)
- `Content-Security-Policy` (CSP): Sayfada çalışacak olan script ve kaynakların kaynağını belirleyerek XSS saldırılarını önler. (`default-src 'self'`)
- `X-Frame-Options`: Sitenizin başka bir sitede iframe içinde gösterilmesini engeller. (`DENY` veya `SAMEORIGIN`)
- `X-Content-Type-Options`: Tarayıcıların MIME türlerini tahmin etmesini engeller. (`nosniff`)
- `Referrer-Policy`: Dış bağlantılara tıklanıldığında sitenizin URL detaylarının aktarılmasını sınırlar. (`strict-origin-when-cross-origin`)
- `Permissions-Policy`: Tarayıcı özelliklerinin (Kamera, Mikrofon vb.) gereksiz kullanımını kısıtlar.

## 3. Source Map Koruması
Üretim ortamında (Production) kaynak kodlarınızın (Source Maps) ifşa olmasını önlemek için `next.config.js` dosyanızdaki `productionBrowserSourceMaps` değerini `false` olarak ayarlayın veya bu satırı tamamen silin.

## 4. CORS Sertleştirme (Cross-Origin Resource Sharing)
API endpointlerinizde hiçbir zaman `Access-Control-Allow-Origin: *` şeklinde joker (wildcard) karakter kullanmayın. Yalnızca güvendiğiniz alan adlarına (whitelist) izin verin.

## 5. Serverless Function Güvenliği
`api/` klasörü altındaki tüm fonksiyonlarınıza gelen verileri mutlaka bir şema doğrulama kütüphanesi (Zod, Yup vb.) ile doğrulayın (Input Validation). Doğrulanmamış hiçbir veriyi veritabanı sorgularına veya işletim sistemi komutlarına dahil etmeyin.

## 6. Preview Deployment Koruması
Vercel'in "Vercel Authentication" veya "Password Protection" özelliklerini kullanarak geliştirme sürümlerinize (Preview) yetkisiz kişilerin erişmesini ve sızdırılmış verileri test etmesini engelleyin.
