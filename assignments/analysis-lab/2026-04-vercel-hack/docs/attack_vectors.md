# Vercel Saldırı Vektörleri ve Risk Analizi

Bu belge, Vercel platformu üzerinde barındırılan uygulamalardaki yapılandırma eksikliklerinden kaynaklanabilecek 12 temel saldırı vektörünü ve bunlara karşılık gelen risk seviyelerini listeler.

## 1. Environment Variable Sızması (`NEXT_PUBLIC_` İfşası)
- **Risk Seviyesi:** Kritik
- **Açıklama:** API anahtarları veya veritabanı şifreleri yanlışlıkla `NEXT_PUBLIC_` (veya `VITE_`) ön ekiyle tanımlandığında, bu değişkenler frontend JavaScript bundle'ına düz metin olarak gömülür. Kötü niyetli bir kişi kaynak kodları inceleyerek bu kritik bilgileri kolayca ele geçirebilir (Vercel Hack Nisan 2026 olayının merkezindeki zafiyet).

## 2. Source Map İfşası (Production `.map` Dosyaları)
- **Risk Seviyesi:** Yüksek
- **Açıklama:** Source map'ler (kaynak haritaları), minify edilmiş kodun orijinal halini geri getirmek için kullanılır. Üretim ortamında(`.map` dosyaları) tarayıcıya sunulursa, uygulamanın özel (proprietary) kodları, yorum satırları ve geliştirme ortamı yolları (path) ifşa olur.

## 3. Serverless Function Injection
- **Risk Seviyesi:** Kritik
- **Açıklama:** `/api` altında çalışan Serverless Function'larda girdi doğrulama eksikliği (No Input Validation). URL parametreleri veya HTTP gövdesi (body) doğrudan işletim sistemi komutlarına, SQL sorgularına veya NoSQL sorgularına beslenirse RCE (Remote Code Execution) veya SQLi oluşabilir.

## 4. Open Redirect Zafiyeti
- **Risk Seviyesi:** Orta
- **Açıklama:** `vercel.json` veya `next.config.js` içindeki `redirects` yapılandırmaları, regex (düzenli ifadeler) hataları içeriyorsa saldırganlar URL parametrelerini manipüle ederek kullanıcıları oltalama (phishing) sitelerine yönlendirebilir.

## 5. CORS Misconfiguration
- **Risk Seviyesi:** Yüksek
- **Açıklama:** API route'larında `Access-Control-Allow-Origin: *` veya kimlik doğrulama gerektiren (credentials) endpoint'lerde çok geniş alan adlarına (wildcard domain) izin verilmesi. Kullanıcıların hassas verilerinin kötü niyetli siteler üzerinden çalınmasına yol açar.

## 6. Header Injection / Eksik Güvenlik Başlıkları
- **Risk Seviyesi:** Orta
- **Açıklama:** HSTS, X-Frame-Options (Clickjacking koruması) veya Content-Security-Policy (XSS koruması) başlıklarının `vercel.json` üzerinde tanımlanmaması. Tarayıcı tabanlı saldırılara zemin hazırlar.

## 7. Path Traversal (Rewrites Aracılığıyla İç API İfşası)
- **Risk Seviyesi:** Yüksek
- **Açıklama:** `rewrites` kurallarında hatalı joker karakter (`*`) kullanımı veya `../` dizilimlerinin süzülmemesi sonucu, kullanıcıların erişmemesi gereken dahili (internal) backend mikroservislerine erişim sağlaması durumu (SSRF).

## 8. Preview Deployment'lara Yetkisiz Erişim
- **Risk Seviyesi:** Yüksek
- **Açıklama:** Vercel Preview URL'leri rastgele üretilir ancak "Vercel Authentication" özelliği kapatılmışsa (Vercel Protection By-pass), bu URL'i ele geçiren veya tahmin eden saldırganlar henüz yayınlanmamış yeni özelliklere ve geliştirme veritabanlarına ulaşabilir.

## 9. Build Log'larında Hassas Bilgi Sızması
- **Risk Seviyesi:** Orta
- **Açıklama:** Derleme işlemi (Next.js build süreci) sırasında `console.log` veya hata mesajlarının Vercel konsoluna kritik şifreler basması. Takım üyelerinden birinin hesabı ele geçirilirse, loglardan yatay hareket (lateral movement) yapılabilir.

## 10. DNS Takeover (Dangling CNAME)
- **Risk Seviyesi:** Kritik
- **Açıklama:** Vercel'e yönlendirilmiş bir CNAME kaydının Vercel panelinden silinmesi ancak DNS sağlayıcısında (Cloudflare vb.) unutulması. Saldırgan aynı domaini kendi Vercel hesabına ekleyerek domaini ele geçirebilir (Subdomain Takeover).

## 11. Middleware Bypass Senaryoları
- **Risk Seviyesi:** Kritik
- **Açıklama:** Edge ortamında çalışan `middleware.ts` dosyasında yazılan kimlik doğrulama veya engelleme kurallarında oluşan regex mantık hataları. Belirli path manipülasyonlarıyla (örn: `/admin/.` veya büyük/küçük harf hileleri) koruma atlatılabilir.

## 12. API Route'larda Authentication Eksikliği
- **Risk Seviyesi:** Yüksek
- **Açıklama:** Middleware veya sayfa bazlı koruma (Frontend route protection) yapılıp, Serverless Function API uçlarında (Backend) aynı kimlik doğrulamanın (Token/Session Check) unutulması. API uçları dış dünyaya her zaman açıktır (Direct API Access).
