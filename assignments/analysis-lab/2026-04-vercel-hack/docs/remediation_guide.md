# Vercel Sertleştirme ve Düzeltme Rehberi

Zafiyet analiz motorumuzun tespit ettiği zafiyetleri kapatmak için uygulamanız gereken adım adım sertleştirme prosedürleri aşağıda listelenmiştir.

## 1. Environment Variable Güvenliği
- **Kural 1 (`NEXT_PUBLIC_`):** Hiçbir zaman şifre (password), token veya API key gibi hassas verilerinizi `NEXT_PUBLIC_` ön eki ile adlandırmayın. Sadece istemci tarafında (tarayıcı) gerçekten ihtiyaç duyulan veriler bu ön eki almalıdır.
- **Kural 2 (Server-side Yalnızca):** Hassas token'ları sadece sunucu tarafında (server-side) çalışan dosyalarda tutun.
- **Kural 3 (Env Scope):** Vercel Dashboard üzerinden (Project > Settings > Environment Variables) tüm hassas değişkenleri eklerken sağ taraftaki ayarlardan **"Sensitive"** kutucuğunu işaretleyin. Değişkenleri Production, Preview ve Development olmak üzere doğru ortamlara atayın.

## 2. HTTP Güvenlik Başlıkları Yapılandırması
`vercel.json` dosyanızda varsayılan olarak bulunmayan aşağıdaki güvenlik başlıklarını (headers) tanımlamalısınız:
- `Strict-Transport-Security` (HSTS): Tüm trafiğin HTTPS üzerinden akmasını zorunlu kılar. (`max-age=63072000; includeSubDomains; preload`)
- `Content-Security-Policy` (CSP): Sayfada çalışacak kaynakları belirleyerek XSS saldırılarını önler. (`default-src 'self'`)
- `X-Frame-Options`: Sitenizin başka bir sitede iframe içinde gösterilmesini engeller. (`DENY` veya `SAMEORIGIN`)
- `X-Content-Type-Options`: Tarayıcıların MIME türlerini tahmin etmesini engeller. (`nosniff`)
- `Referrer-Policy`: Sitenizin URL detaylarının aktarılmasını sınırlar. (`strict-origin-when-cross-origin`)
- `Permissions-Policy`: Tarayıcı özelliklerinin kullanımını kısıtlar.

## 3. Source Map Koruması
- **Kural 1:** Üretim ortamında (Production) kaynak kodlarınızın (Source Maps) ifşa olmasını önlemek için source map'leri devre dışı bırakın.
- **Kural 2:** `next.config.js` dosyanızdaki `productionBrowserSourceMaps` değerini açıkça `false` olarak ayarlayın.

## 4. CORS Sertleştirme (Cross-Origin Resource Sharing)
- **Kural 1:** API endpointlerinizde hiçbir zaman `Access-Control-Allow-Origin: *` şeklinde joker (wildcard) karakter kullanmayın. Yalnızca güvendiğiniz alan adlarını whitelist olarak tanımlayın.
- **Kural 2:** Eğer credentials (çerezler vb.) ile istek yapılıyorsa wildcard kullanımına kesinlikle izin vermeyin.

## 5. Serverless Function Güvenliği
- **Girdi Doğrulama:** `api/` klasörü altındaki tüm fonksiyonlarınıza gelen verileri mutlaka bir şema doğrulama kütüphanesi (Zod, Joi vb.) ile doğrulayın.
- **Rate Limiting:** DDoS saldırılarını ve gereksiz faturalandırmayı engellemek için rate limiting (istek sınırlandırma) uygulayın.
- **Authentication Middleware:** Tüm API rotalarını yetkilendirme (Authentication) middleware'inden geçirerek doğrudan erişimi kısıtlayın.

## 6. Preview Deployment Koruması
- **Vercel Authentication:** Vercel Dashboard üzerinden Vercel Authentication'ı aktif ederek sadece ekibinizin Preview bağlantılarına erişmesini sağlayın.
- **Password Protection:** Alternatif olarak, kritik projeleriniz için genel bir şifre koruması ayarlayın.

## 7. DNS Güvenliği
- **Dangling CNAME Temizliği:** Vercel projenizi sildiğinizde, DNS sağlayıcınızdan (Cloudflare, Route53 vb.) CNAME kayıtlarını da sildiğinizden emin olun.
- **DNSSEC:** Alan adınızın ele geçirilmesini önlemek için DNSSEC özelliğini aktif edin.

## 8. Build Pipeline Güvenliği
- **Maskeleme:** Build süreci boyunca Vercel loglarında hassas bilgilerin maskelendiğinden emin olun.
- **Secret Yönetimi:** Build komutlarında (örn: `npm run build`) API anahtarlarını düz metin olarak geçmek yerine doğrudan gizli environment variable'ları kullanın.
