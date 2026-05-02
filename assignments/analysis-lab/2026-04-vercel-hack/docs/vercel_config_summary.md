# Vercel Yapılandırma Dosyaları Teknik Özeti

Vercel platformunda projelerin davranışları, yönlendirmeleri ve güvenlik politikaları genellikle proje kök dizinindeki belirli dosyalar aracılığıyla yapılandırılır.

## 1. `vercel.json`
Vercel projesinin temel yapılandırma dosyasıdır. Eski projelerde veya Next.js dışı framework'lerde sıklıkla kullanılır.

- **`headers`:** İstemciye gönderilen HTTP başlıklarını tanımlar. Güvenlik başlıkları (HSTS, CSP, X-Frame-Options vb.) buraya eklenir.
- **`redirects`:** Gelen istekleri HTTP 301/302 ile başka bir URL'ye yönlendirir. Open Redirect zafiyetlerini önlemek için katı regex kuralları yazılmalıdır.
- **`rewrites`:** İstek URL'sini değiştirmeden içeriği başka bir yoldan (veya dış API'den) sunar. Hatalı kullanımı SSRF veya iç ağ/API ifşasına yol açabilir (Path Traversal).
- **`crons`:** Zamanlanmış görevleri tetikleyen endpoint'leri belirtir. Bu endpoint'lerin yetkisiz tetiklenmesini engellemek için Vercel Cron Secret kullanılmalıdır.
- **`cleanUrls` & `trailingSlash`:** SEO ve yönlendirme standartları.

## 2. `next.config.js` / `next.config.mjs`
Next.js projelerinde kullanılan ana yapılandırma dosyasıdır. Vercel ortamında `vercel.json` yerine geçer ve daha esnektir.

- **Güvenlik Özellikleri:**
  - `poweredByHeader: false` -> HTTP yanıtından `X-Powered-By: Next.js` bilgisini kaldırarak bilgi ifşasını azaltır.
  - `headers()` async fonksiyonu ile dinamik güvenlik başlıkları atanabilir.
  - `redirects()` ve `rewrites()` ile `vercel.json` benzeri yönlendirmeler yönetilir.
- **Image Optimization:** Harici domainlerden görsel yüklenmesine izin verilir. Bu domainler `images.remotePatterns` altında kesin (strict) kurallarla belirlenmelidir, aksi takdirde SSRF benzeri maliyet/kaynak sömürüsü ataklarına açık hale gelir.

## 3. `.vercelignore`
Deployment sırasında Vercel sunucularına **yüklenmemesi** gereken dosyaları belirtir.

- **Önemi:** Test veritabanları (`*.sqlite`), geliştirme ortamı ayar dosyaları, lokal `.env` dosyaları veya özel dokümanların yanlışlıkla derleme ortamına gitmesini ve bir güvenlik ihlali sonucu ifşa olmasını engeller.

## 4. `middleware.ts`
Vercel Edge Network üzerinde çalışan, Next.js veya Vercel projelerinde istekleri daha ana uygulamaya ulaşmadan önce kesen (intercept) katmandır.

- **Kullanım Alanları:**
  - JWT token doğrulama.
  - Geo-IP tabanlı engelleme (Rate Limiting veya Blacklisting).
  - Güvenlik başlıklarının dinamik olarak eklenmesi.
- **Risk:** Eğer middleware'deki mantıksal hata nedeniyle kritik route'lar atlanırsa (bypass), uygulama yetkisiz erişimlere açık hale gelir. Edge ortamında çalıştığı için NPM paket desteği sınırlıdır (sadece Edge uyumlu kütüphaneler çalışır).
