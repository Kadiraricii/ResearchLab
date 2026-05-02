# Vercel Platform Mimari Analizi

Bu doküman, Vercel platformunun temel mimari bileşenlerini ve bunların güvenlik üzerindeki etkilerini analiz eder.

## 1. Serverless Functions Yapısı
Vercel'de Node.js, Go, Python, Ruby ve Rust destekli sunucusuz işlevler (Serverless Functions) çalıştırılabilir.
- **Çalışma Ortamı:** İşlevler AWS Lambda tabanlı izole edilmiş kapsayıcı (container) ortamlarında çalışır.
- **Geçici Dosya Sistemi:** `/tmp` dizini dışında dosya sistemi salt okunurdur. Bu durum kalıcı malware barındırmayı zorlaştırır ancak bellek içi (in-memory) saldırıları engellemez.
- **Süre Sınırı:** İşlevlerin çalışma süresi sınırlıdır (Hobby: 10s, Pro: 60s/300s).
- **Güvenlik Çıkarımı:** Uygulama kodu, veritabanı veya harici API anahtarları gibi ortam değişkenlerine anında erişebilir. İşlev içindeki bir RCE (Remote Code Execution) zafiyeti, bu değişkenlerin ifşasına yol açabilir.

## 2. Edge Network / CDN Katmanı
Vercel Edge Network, Vercel'in global içerik dağıtım ağıdır. 
- **Edge Middleware:** İstekler ana sunucuya veya serverless işlevlere ulaşmadan önce, Edge lokasyonlarında V8 isolate ortamında (Node.js API'lerinin bir alt kümesi) çalışan Edge Middleware tarafından karşılanır.
- **Güvenlik Çıkarımı:** Middleware, kimlik doğrulama, IP engelleme ve güvenlik başlıklarının (Security Headers) eklenmesi için ideal bir katmandır. Ancak, Middleware hatalı yapılandırılırsa veya atlatılırsa (bypass), yetkisiz erişimlere açık kapı bırakabilir.

## 3. Build Pipeline ve Deployment Süreci
Vercel, GitHub, GitLab veya Bitbucket depolarından her push işleminde otomatik build (derleme) alır.
- **Immutable Deployments:** Her derleme yeni, benzersiz ve değiştirilemez bir URL üretir (`preview` deployment).
- **Güvenlik Çıkarımı:** Geliştiriciler üretim öncesi hataları görebilir, ancak bu "preview" URL'leri sızarsa, yetkisiz kişiler henüz yayınlanmamış özellikleri veya potansiyel zafiyetleri inceleyebilir. Vercel Authentication aktif edilmezse bu ciddi bir risk oluşturur. Derleme loglarında sızabilecek hassas veriler de ayrı bir saldırı vektörüdür.

## 4. Environment Variables Yönetimi
Ortam değişkenleri (Environment Variables), Vercel paneli üzerinden yönetilir.
- **Kapsam (Scope):** Production, Preview ve Development olarak üç farklı kapsama sahiptir.
- **Sensitive vs Non-Sensitive:** "Sensitive" olarak işaretlenen değişkenler, Vercel panelinde asla düz metin olarak gösterilmez (sadece şifreli saklanır ve build/runtime sırasında çözülür). Non-sensitive olanlar ise panelden okunabilir durumdadır.
- **Güvenlik Çıkarımı:** April 2026 Vercel Hack olayında görüldüğü üzere, "Sensitive" olmayan verilerin çalınması çok daha kolaydır. Tüm parolalar, anahtarlar ve kritik bağlantı dizeleri "Sensitive" olarak işaretlenmelidir.
