# Vercel Varsayılan Güvenlik Davranışları

Geliştiriciler özel bir ayar yapmadığı takdirde, Vercel platformu aşağıdaki varsayılan davranışları sergiler. Bunları bilmek, zafiyet analizi için kritiktir.

## 1. Varsayılan HTTP Başlık (Header) Politikası
Vercel varsayılan olarak **hiçbir gelişmiş güvenlik başlığı eklemez**.
- **HSTS (Strict-Transport-Security):** Varsayılan olarak eklenmez. Projelerin `next.config.js` veya `vercel.json` üzerinden eklemesi gerekir.
- **CSP (Content-Security-Policy):** Geliştirici eklemediği sürece yollanmaz (XSS'e davetiye çıkarır).
- **X-Powered-By:** Next.js projelerinde varsayılan olarak `X-Powered-By: Next.js` gönderilir (Information Disclosure).

## 2. Varsayılan CORS Davranışı
- Standart `/api` (Serverless Functions) route'ları varsayılan olarak **CORS başlıkları göndermez**. Bu, tarayıcıların başka domainlerden yapılan istekleri reddetmesini sağlar (güvenli varsayılan).
- Ancak geliştiriciler genellikle bu sorunu çözmek için `Access-Control-Allow-Origin: *` şeklinde güvensiz yapılandırmalar kurarlar.

## 3. Ortam Değişkenleri ve `NEXT_PUBLIC_` İfşası
- Next.js (ve diğer frameworkler VITE vb. `VITE_` ile) projelerinde, isminde `NEXT_PUBLIC_` öneki (prefix) bulunan ortam değişkenleri, derleme aşamasında (build time) doğrudan **istemciye gönderilen JavaScript bundle'larının içine gömülür**.
- **Kritik Risk:** API gizli anahtarları, veritabanı şifreleri vb. yanlışlıkla `NEXT_PUBLIC_` ile tanımlanırsa, uygulama yayınlandığı an herkes tarafından okunabilir hale gelir.

## 4. Source Map Varsayılan Erişim Durumu
- Next.js ve Vite projeleri derleme sırasında `.map` (Source Map) dosyaları üretir.
- **Vercel Davranışı:** Next.js 13+ versiyonlarında production ortamında `.map` dosyaları tarayıcıya sunulmaz. Ancak özel bir build komutu veya farklı bir framework kullanılıyorsa, kaynak kodlar (orijinal Typescript ve yorum satırları dahil) ifşa olabilir.
- Next.js için `.map` ifşasını test etmek: `https://domain.com/_next/static/chunks/main-*.js.map` dosyasına erişim denenmelidir.

## 5. `.env` Dosyaları ve Deployment
- Vercel CLI, varsayılan olarak yerel dizindeki `.env`, `.env.local` dosyalarını okur ancak bunları derleme sunucusuna (deployment) **yüklemez**. Vercel, kendi panelinde tanımlanan değişkenleri (Vercel Environment Variables UI) kullanır.
- Ancak `.env` dosyası Git deposuna (commitlenerek) eklenmişse, bu kod ile beraber depolanır ve ifşa riski taşır.
