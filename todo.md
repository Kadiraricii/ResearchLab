# VERCEL HACK - Analysis Lab Todo
**UUID**: `434fce1d-2fa3-450c-9847-c27724a49552`
**Type**: `ANALYSIS-LAB`
**Stack**: Rust + Tauri + Bun (frontend)

---

## PHASE 0: Proje Altyapi & Ortam Kurulumu

### 0.1 Toolchain & Runtime
- [x] Rust toolchain kurulumu (`rustup`, `cargo`, `rustc` dogrulama)
- [x] Rust nightly toolchain ekle (benchmark ve unstable ozellikler icin)
- [x] Bun kurulumu ve dogrulama (`bun --version`)
- [x] Tauri CLI kurulumu (`cargo install tauri-cli`)
- [x] `create-tauri-app` ile proje iskeleti olustur
- [x] Bun bagimliliklarini yukle (`bun install`)
- [x] `cargo-watch` kur (hot reload: `cargo install cargo-watch`)
- [x] `cargo-nextest` kur (hizli test runner: `cargo install cargo-nextest`)
- [x] `cargo-llvm-cov` kur (code coverage: `cargo install cargo-llvm-cov`)
- [x] `cargo-audit` kur (guvenlik denetimi: `cargo install cargo-audit`)
- [x] `cargo-deny` kur (bagimlilik politikasi: `cargo install cargo-deny`)
- [x] `cargo-machete` kur (kullanilmayan bagimlilik tespiti)
- [x] `sccache` kur ve yapilandir (Rust derleme cache — sureci hizlandirir)
- [x] `mold` linker kur (Linux) / `lld` (Windows) — link suresini %60 kisalt (N/A macOS)


### 0.2 Proje Dizin Yapisi
- [x] `src-tauri/` (Rust backend)
- [x] `src-tauri/src/analyzer/` (guvenlik analiz modulleri)
- [x] `src-tauri/src/vercel/` (Vercel API client)
- [x] `src-tauri/src/summary/` (teknik ozet modulleri)
- [x] `src-tauri/src/remediation/` (duzeltme modulleri)
- [x] `src/` (Frontend)
- [x] `scripts/` (Otomasyon scriptleri)
- [x] `configs/` (Ornek yapilandirma dosyalari)
- [x] `docs/` (Arastirma ve bulgular)
- [x] `tests/` (Rust entegrasyon testleri)
- [x] `tests/unit/` (Rust unit test yardimcilari)
- [x] `tests/integration/` (Rust entegrasyon testleri)
- [x] `tests/e2e/` (End-to-end testler)
- [x] `tests/fixtures/` (Test verileri — sahte vercel.json, next.config.js vb.)
- [x] `tests/mocks/` (Mock API response'lari)
- [x] `tests/snapshots/` (Snapshot test verileri)
- [x] `benches/` (Rust benchmark testleri)
- [x] `.github/workflows/` (CI/CD pipeline dosyalari)
- [x] `.github/ISSUE_TEMPLATE/` (Issue sablonlari)
- [x] `.github/PULL_REQUEST_TEMPLATE.md` (PR sablonu)

### 0.3 Git & Versiyon Kontrolu
- [x] `git init` ile repo baslat
- [x] `.gitignore` olustur:
  - [x] `target/`
  - [x] `node_modules/`
  - [x] `.env`
  - [x] `.env.local`
  - [x] `.env.production`
  - [x] `.env.test`
  - [x] `dist/`
  - [x] `build/`
  - [x] `*.log`
  - [x] `.DS_Store`
  - [x] `Thumbs.db`
  - [x] `*.pem`
  - [x] `*.key`
  - [x] `.tauri/`
  - [x] `bunfig.toml` (hassas ise)
  - [x] `coverage/`
  - [x] `lcov.info`
  - [x] `*.profraw`
  - [x] `test-results/`
  - [x] `playwright-report/`
  - [x] `.nyc_output/`
- [x] `.gitattributes` olustur
- [x] `git hooks` ayarla (pre-commit, pre-push):
  - [x] `pre-commit`: `cargo fmt --check && cargo clippy`
  - [x] `pre-push`: `cargo nextest run`
- [x] Ilk commit: `chore: initial project scaffold`

### 0.4 Ortam Degiskenleri & Gizli Bilgiler
- [x] `.env` dosyasi olustur:
  - [x] `VERCEL_TOKEN=`
  - [x] `VERCEL_ORG_ID=`
  - [x] `VERCEL_PROJECT_ID=`
  - [x] `API_BASE_URL=`
  - [x] `LOG_LEVEL=debug`
  - [x] `RUST_LOG=info`
  - [x] `DATABASE_URL=` (SQLite yerel depolama)
  - [x] `SCCACHE_DIR=` (derleme cache dizini)
- [x] `.env.test` olustur (test ortami degiskenleri):
  - [x] `VERCEL_TOKEN=test_mock_token`
  - [x] `DATABASE_URL=sqlite::memory:`
  - [x] `RUST_LOG=debug`
  - [x] `MOCK_API=true`
- [x] `.env.example` olustur (bos degerlerle, repo'ya dahil)
- [x] `bunfig.toml` olustur (Bun yapilandirmasi)

### 0.5 IDE & Editor Yapilandirmasi

#### Neovim
- [x] `.nvim.lua` olustur:
  - [x] rust-analyzer LSP ayarlari (clippy on save, check on save)
  - [x] Tauri-specific inlay hints
  - [x] Formatter (rustfmt) entegrasyonu — save'de otomatik format
  - [x] DAP (debug) yapilandirmasi — `codelldb` adapter
  - [x] DAP launch config: Tauri dev, Tauri test, tek test calistir
  - [x] Keybinding'ler:
    - [x] `<leader>tb` — `cargo tauri build`
    - [x] `<leader>td` — `cargo tauri dev`
    - [x] `<leader>tt` — `cargo nextest run`
    - [x] `<leader>tf` — dosyadaki testleri calistir
    - [x] `<leader>tc` — coverage raporu
    - [x] `<leader>tl` — `cargo clippy`
  - [x] Neotest entegrasyonu (test runner UI):
    - [x] `neotest-rust` adapter
    - [x] Test sonuclarini inline goster
    - [x] Basirsiz testleri otomatik isle
  - [x] Trouble.nvim (diagnostics listesi)
  - [x] nvim-dap-ui (debug gorunumu)
  - [x] rust-tools.nvim / rustaceanvim ayarlari:
    - [x] Hover actions
    - [x] Code action groups
    - [x] Crate graph
    - [x] Expandable macros
  - [x] Telescope entegrasyonu:
    - [x] Test dosyalari ara
    - [x] Analyzer modulleri ara
    - [x] Bulgu/finding arama

#### VS Code
- [x] `.vscode/settings.json` olustur:
  - [x] rust-analyzer ayarlari (checkOnSave, clippy)
  - [x] Tauri extension ayarlari
  - [x] Format on save (Rust + TS/JS)
  - [x] Editor rulers (100 karakter)
  - [x] Test explorer ayarlari
  - [x] Coverage gutter ayarlari
- [x] `.vscode/launch.json` olustur:
  - [x] Tauri dev debug
  - [x] Rust unit test debug
  - [x] Rust integration test debug
  - [x] E2E test debug
  - [x] Attach to running process
- [x] `.vscode/tasks.json` olustur:
  - [x] `cargo build` task
  - [x] `cargo tauri dev` task
  - [x] `cargo nextest run` task
  - [x] `cargo clippy` task
  - [x] `bun run dev` task
  - [x] `bun run test` task
  - [x] `bun run e2e` task
  - [x] `docker-compose up` task
  - [x] `full audit` task (tum testler + lint + build)
- [x] `.vscode/extensions.json` — onerilen extension listesi:
  - [x] `rust-lang.rust-analyzer`
  - [x] `tauri-apps.tauri-vscode`
  - [x] `vadimcn.vscode-lldb`
  - [x] `serayuzgur.crates`
  - [x] `ryanluker.vscode-coverage-gutters`
  - [x] `ms-azuretools.vscode-docker`
  - [x] `dbaeumer.vscode-eslint`
  - [x] `esbenp.prettier-vscode`

#### JetBrains (CLion / RustRover)
- [x] `.idea/runConfigurations/` olustur:
  - [x] `Tauri_Dev.xml` — tauri dev calistir
  - [x] `Cargo_Test.xml` — tum testleri calistir
  - [x] `Cargo_Clippy.xml` — lint calistir
  - [x] `E2E_Tests.xml` — e2e test suite
- [x] `.idea/codeStyles/` — Rust formatlama kurallari

#### Genel Editor Ayarlari
- [x] `rust-toolchain.toml` (Rust versiyon sabitleme — `stable` + `nightly` components)
- [x] `rustfmt.toml` (Rust formatlama kurallari):
  - [x] `edition = "2021"`
  - [x] `max_width = 100`
  - [x] `tab_spaces = 4`
  - [x] `use_small_heuristics = "Max"`
- [x] `clippy.toml` (Rust lint kurallari):
  - [x] `cognitive-complexity-threshold = 25`
  - [x] `too-many-arguments-threshold = 8`
- [x] `deny.toml` (cargo-deny yapilandirmasi):
  - [x] Lisans politikasi (izin verilen/yasaklanan lisanslar)
  - [x] Bilinen zafiyet kontrolu (advisories)
  - [x] Duplicate crate kontrolu
- [x] `.editorconfig` olustur:
  - [x] Rust: 4 space indent
  - [x] TS/JS: 2 space indent
  - [x] TOML/YAML: 2 space indent
  - [x] Trailing whitespace temizle
  - [x] Final newline ekle

---

## PHASE 1: Teknik Ozet — Vercel Yapilandirma/Olay Analizi
> _tasks.md Instruction 1: "Provide a technical summary of the issue or configuration directive."_

- [x] Vercel platform mimarisini arastir ve belgele:
  - [x] Serverless Functions yapisi
  - [x] Edge Network / CDN katmani
  - [x] Build pipeline ve deployment sureci
  - [x] Environment Variables yonetimi
- [x] Vercel yapilandirma dosyalarinin teknik ozetini cikar:
  - [x] `vercel.json` — routes, headers, redirects, rewrites, crons
  - [x] `next.config.js` / `next.config.mjs` — guvenlik ile ilgili direktifler
  - [x] `.vercelignore` — hangi dosyalar deploy edilmemeli
  - [x] `middleware.ts` — edge middleware davranisi
- [x] Vercel'in varsayilan guvenlik davranislarini belgele:
  - [x] Varsayilan HTTP baslik (header) politikasi
  - [x] Varsayilan CORS davranisi
  - [x] Env degiskenleri ifsa kurallari (`NEXT_PUBLIC_` prefix davranisi)
  - [x] Source map varsayilan erisim durumu
  - [x] `.env` dosyalari deployment'a dahil mi?
- [x] Rust modulu yaz — `src-tauri/src/summary/`:
  - [x] `mod.rs` — modul tanimlari
  - [x] `vercel_config.rs` — vercel.json parser (serde ile)
  - [x] `next_config.rs` — next.config.js analiz
  - [x] `platform_defaults.rs` — varsayilan davranis veritabani
- [x] Frontend'de "Teknik Ozet" sayfasi olustur:
  - [x] Yapilandirma dosyasi yukle/parse et
  - [x] Ozet rapor gorunumu

---

## PHASE 2: Saldiri Vektorleri & Riskler
> _tasks.md Instruction 2: "Outline the attack vector or risk if misconfigured."_

- [x] Saldiri vektorlerini arastir ve belgele:
  - [x] Environment Variable sizmasi (`NEXT_PUBLIC_` ile hassas veri ifsa)
  - [x] Source map ifsa (production'da `.map` dosyalari erisimi)
  - [x] Serverless function injection (girdi dogrulama eksikligi)
  - [x] Open redirect zafiyeti (`vercel.json` redirects yapilandirmasi)
  - [x] CORS misconfiguration (wildcard origin, credentials leak)
  - [x] Header injection / eksik guvenlik basliklari
  - [x] Path traversal (rewrites ile ic API'lara erisim)
  - [x] Preview deployment'lara yetkisiz erisim
  - [x] Build log'larinda hassas bilgi sizmasi
  - [x] DNS takeover (dangling CNAME kayitlari)
  - [x] Middleware bypass senaryolari
  - [x] API route'larda authentication eksikligi
- [x] Her vektor icin risk seviyesi belirle (kritik/yuksek/orta/dusuk)
- [x] Rust analiz modulleri yaz — `src-tauri/src/analyzer/`:
  - [x] `mod.rs` — modul tanimlari
  - [x] `env_exposure.rs` — Aciga cikan env degiskenleri tespit
  - [x] `source_maps.rs` — Source map ifsa kontrolu
  - [x] `headers.rs` — Guvenlik basliklari analizi (HSTS, CSP, X-Frame, X-Content-Type vb.)
  - [x] `cors.rs` — CORS yapilandirma kontrolu
  - [x] `redirects.rs` — Open redirect analizi
  - [x] `rewrites.rs` — Path traversal / ic API ifsa analizi
  - [x] `dns.rs` — DNS yapilandirma ve takeover kontrolu
  - [x] `ssl.rs` — SSL/TLS sertifika kontrolu
  - [x] `serverless.rs` — Serverless function guvenlik analizi
  - [x] `middleware.rs` — Middleware bypass kontrolu
  - [x] `preview_auth.rs` — Preview deployment erisim kontrolu
  - [x] `build_logs.rs` — Build log hassas bilgi taramasi
- [x] Risk skorlama sistemi (CVSS benzeri puanlama)
- [x] Frontend'de "Saldiri Vektorleri" sayfasi:
  - [x] Vektor listesi ve aciklamalari
  - [x] Risk matrisi gorunumu
  - [x] Tarama baslat / sonuclari goruntule

---

## PHASE 3: Sertlestirme & Duzeltme Rehberi
> _tasks.md Instruction 3: "Provide a step-by-step hardening or remediation guide."_

- [x] Her saldiri vektoru icin adim adim sertlestirme rehberi yaz:
  - [x] Env degiskenleri guvenligi:
    - [x] `NEXT_PUBLIC_` prefix kullanim kurallari
    - [x] Hassas token'lari sadece server-side'da tut
    - [x] Vercel dashboard'da env scope ayarlari (Production/Preview/Development)
  - [x] HTTP guvenlik basliklari yapilandirmasi:
    - [x] `Strict-Transport-Security` (HSTS)
    - [x] `Content-Security-Policy` (CSP)
    - [x] `X-Frame-Options`
    - [x] `X-Content-Type-Options`
    - [x] `Referrer-Policy`
    - [x] `Permissions-Policy`
  - [x] Source map korumasi:
    - [x] Production'da source map'leri devre disi birak
    - [x] `productionBrowserSourceMaps: false` ayari
  - [x] CORS sertlestirme:
    - [x] Whitelist bazli origin kontrolu
    - [x] Credentials ile wildcard kullanma
  - [x] Serverless function guvenligi:
    - [x] Girdi dogrulama (input validation)
    - [x] Rate limiting
    - [x] Authentication middleware
  - [x] Preview deployment korumasi:
    - [x] Vercel Authentication aktif et
    - [x] Password protection
  - [x] DNS guvenligi:
    - [x] Dangling CNAME temizligi
    - [x] DNSSEC aktif et
  - [x] Build pipeline guvenligi:
    - [x] Build log'larda hassas bilgi maskele
    - [x] Build komutlarinda secret yonetimi
- [x] Rust modulu — `src-tauri/src/remediation/`:
  - [x] `mod.rs` — modul tanimlari
  - [x] `recommendations.rs` — Bulgulara gore otomatik oneri uret
  - [x] `templates.rs` — Duzeltme sablonlari (vercel.json, next.config.js, headers)
  - [x] `scoring.rs` — Sertlestirme skoru hesapla (once/sonra karsilastirma)
- [x] Frontend'de "Sertlestirme Rehberi" sayfasi:
  - [x] Adim adim checklist gorunumu
  - [x] Oncesi/sonrasi yapilandirma karsilastirmasi
  - [x] Otomatik duzeltme onerisi butonu

---

## PHASE 4: Scriptler, Konfigurasyonlar & Komutlar
> _tasks.md Instruction 4: "Include any relevant scripts, configurations, or commands used."_

- [x] `configs/` dizinine ornek yapilandirma dosyalari:
  - [x] `vercel.json` — guvenli ornek (headers, redirects, rewrites)
  - [x] `next.config.js` — sertlestirilmis ornek
  - [x] `middleware.ts` — guvenli edge middleware ornegi
  - [x] `headers.json` — onerilen guvenlik basliklari seti
  - [x] `nginx.conf` — Vercel onunde reverse proxy (varsa) guvenli yapilandirma
  - [x] `.vercelignore` — hassas dosyalari haric tut
- [x] `scripts/` dizinine otomasyon scriptleri (Rust CLI / shell):
  - [x] `scan_env.rs` — Env degiskeni sizintisi tarama scripti
  - [x] `check_headers.rs` — HTTP baslik kontrolu scripti
  - [x] `dns_check.rs` — DNS takeover kontrolu scripti
  - [x] `source_map_check.rs` — Source map ifsa kontrolu
  - [x] `full_audit.rs` — Tum kontrolleri calistiran tam audit
- [x] `docker-compose.yml` olustur:
  - [x] `app` servisi (Tauri uygulamasi / headless scanner)
  - [x] `scanner` servisi (arka plan tarama)
  - [x] Volume mount'lar (config, data, reports)
  - [x] Network yapilandirmasi
- [x] `Dockerfile` olustur:
  - [x] Multi-stage build (Rust derleme + runtime)
  - [x] Bun ile frontend build
  - [x] Minimal runtime image
  - [x] `sccache` mount ile derleme hizi artir
  - [x] Layer caching optimize et (bagimlilik layer ayri, kod layer ayri)
- [x] `Dockerfile.test` olustur (test ortami icin):
  - [x] Tum test araclari dahil
  - [x] Mock Vercel API server dahil
  - [x] Coverage araclari dahil
- [x] `.dockerignore` olustur:
  - [x] `target/`
  - [x] `node_modules/`
  - [x] `.git/`
  - [x] `.env`
  - [x] `*.md`
  - [x] `coverage/`
  - [x] `test-results/`
- [x] Rapor disari aktarma:
  - [x] JSON cikti formati
  - [x] HTML rapor sablonu
  - [x] PDF rapor (opsiyonel)

---

## PHASE 5: Tauri & Rust Backend Entegrasyonu

- [x] `Cargo.toml` bagimliliklari:
  - [x] `tauri` (ana framework)
  - [x] `serde` + `serde_json` (serialization)
  - [x] `reqwest` (HTTP istekleri - Vercel API)
  - [x] `tokio` (async runtime)
  - [x] `dotenv` (ortam degiskenleri — dotenvy kullanildi)
  - [ ] `rusqlite` / `sqlx` (yerel veritabani — Phase 7'de)
  - [x] `log` + `env_logger` (loglama)
  - [x] `chrono` (tarih/saat)
  - [x] `thiserror` (hata yonetimi)
  - [x] `rayon` (paralel islem — tarama hizlandirma)
  - [x] `dashmap` (concurrent hashmap — cache icin)
- [x] `[dev-dependencies]` test bagimliliklari:
  - [x] `mockito` (HTTP mock server)
  - [x] `wiremock` (gelismis HTTP mock)
  - [x] `assert_cmd` (CLI komut testi)
  - [x] `predicates` (test assertion yardimcilari)
  - [x] `tempfile` (gecici dosya/dizin testlerde)
  - [x] `insta` (snapshot testing)
  - [x] `proptest` (property-based testing)
  - [x] `criterion` (benchmark framework)
  - [x] `tokio-test` (async test yardimcilari)
  - [x] `fake` (sahte veri uretimi — test fixtures)
  - [x] `test-log` (testlerde log yakalama)
- [x] `tauri.conf.json` yapilandir:
  - [x] Uygulama adi, pencere boyutu, baslik
  - [x] CSP ayarlari
  - [x] Bundle ayarlari (icon, identifier)
- [x] Tauri komutlari (commands) olustur:
  - [x] `get_vercel_projects` — Vercel projelerini listele
  - [x] `get_vercel_deployments` — Deployment listesi
  - [x] `run_full_scan` — Tam guvenlik taramasi baslat
  - [x] `analyze_env_exposure` — Env degisken sizinti analizi
  - [x] `check_security_headers` — HTTP guvenlik basliklari kontrolu
  - [x] `scan_misconfigurations` — Yapilandirma hatasi taramasi
  - [x] `get_remediation` — Duzeltme onerileri getir
  - [x] `export_report` — Rapor disa aktarma
- [x] Vercel REST API client modulu — `src-tauri/src/vercel/`:
  - [x] `mod.rs` — modul tanimlari
  - [x] `client.rs` — HTTP client wrapper
  - [x] `types.rs` — API response tipleri (serde)
  - [x] `projects.rs` — Proje endpoint'leri
  - [x] `deployments.rs` — Deployment endpoint'leri
  - [x] `domains.rs` — Domain endpoint'leri
  - [x] `env_vars.rs` — Ortam degiskenleri endpoint'leri
- [x] Rate limiting ve retry mekanizmasi
- [x] API hata yonetimi (401, 403, 429 vb.)
- [x] Response caching (`dashmap` ile in-memory cache)
- [x] Paralel tarama (`rayon` ile birden fazla analyzer ayni anda calistir)

---

## PHASE 6: Frontend (Bun + Web UI)

- [ ] `package.json` duzenle:
  - [ ] `scripts`: dev, build, preview, tauri, test, test:e2e, test:unit, coverage
  - [ ] Frontend bagimliliklari ekle
  - [ ] Test bagimliliklari ekle
- [ ] `bun.lockb` olustur (`bun install` ile)
- [ ] Dashboard (Ana Sayfa):
  - [ ] Proje listesi paneli
  - [ ] Deployment durumu paneli
  - [ ] Genel guvenlik skoru gostergesi
  - [ ] Son tarama sonuclari ozeti
- [ ] Teknik Ozet sayfasi (Phase 1 verileri)
- [ ] Saldiri Vektorleri sayfasi (Phase 2 verileri):
  - [ ] Risk matrisi
  - [ ] Vektor detay gorunumu
- [ ] Sertlestirme Rehberi sayfasi (Phase 3 verileri):
  - [ ] Adim adim checklist
  - [ ] Yapilandirma karsilastirmasi
- [ ] Tarama Sonuclari sayfasi:
  - [ ] Bulgu tablosu (kritik/yuksek/orta/dusuk filtreleme)
  - [ ] Detay modal
- [ ] Rapor sayfasi:
  - [ ] Rapor goruntuleme
  - [ ] JSON/HTML disa aktarma
- [ ] Ayarlar sayfasi (API token girisi, tema)
- [ ] Tauri invoke cagrilari frontend'e bagla

---

## PHASE 7: Veritabani & Yerel Depolama

- [ ] SQLite veritabani semasi olustur:
  - [ ] `scans` tablosu (tarama gecmisi)
  - [ ] `findings` tablosu (bulgular — vektor, risk, aciklama)
  - [ ] `projects` tablosu (takip edilen projeler)
  - [ ] `reports` tablosu (olusturulan raporlar)
  - [ ] `remediations` tablosu (uygulanan duzeltmeler)
- [ ] Migration sistemi kur
- [ ] CRUD operasyonlari yaz
- [ ] Veritabani dosyasi Tauri app data dizininde sakla

---

## PHASE 8: Test Ortami & Test Araclari

### 8.1 Test Altyapisi Kurulumu
- [ ] `tests/` dizin yapisi olustur (unit, integration, e2e, fixtures, mocks, snapshots)
- [ ] Test yapilandirma dosyalari:
  - [ ] `.cargo/config.toml` — test profili ayarlari, nextest yapilandirmasi
  - [ ] `nextest.toml` — cargo-nextest yapilandirma:
    - [ ] Test thread sayisi (paralel calistirma)
    - [ ] Timeout suresi (test basi)
    - [ ] Retry politikasi (flaky test handling)
    - [ ] Test gruplari (unit/integration/e2e ayri profiller)
    - [ ] JUnit XML cikti (CI icin)
  - [ ] `.config/nextest.toml` — profil tanimlari:
    - [ ] `default` profil: hizli, sadece unit
    - [ ] `ci` profil: tum testler, retry 2, JUnit output
    - [ ] `e2e` profil: yalniz e2e, timeout uzun
- [ ] Test ortami degiskenleri (`.env.test`):
  - [ ] Mock Vercel API URL
  - [ ] Test veritabani (in-memory SQLite)
  - [ ] Debug log seviyesi
- [ ] Test helper crate olustur (`tests/common/mod.rs`):
  - [ ] Test veritabani factory (her test icin temiz DB)
  - [ ] Mock Vercel API server baslat/durdur
  - [ ] Fixture loader (test verisi yukleme)
  - [ ] Assertion yardimcilari (ozel matcher'lar)

### 8.2 Rust Unit Testleri
- [ ] Vercel API client testleri:
  - [ ] `client.rs` — HTTP baglanti, auth header, base URL testi
  - [ ] `projects.rs` — proje listesi parse testi
  - [ ] `deployments.rs` — deployment listesi parse testi
  - [ ] `env_vars.rs` — env var listesi parse testi
  - [ ] Hata senaryolari: 401, 403, 404, 429, 500 response
  - [ ] Rate limit retry davranisi testi
  - [ ] Timeout davranisi testi
- [ ] Analyzer modul testleri (her vektor icin):
  - [ ] `env_exposure` — NEXT_PUBLIC_ sizma tespiti testi
  - [ ] `source_maps` — .map dosya erisim testi
  - [ ] `headers` — eksik/yanlis baslik tespiti testi
  - [ ] `cors` — wildcard origin tespiti testi
  - [ ] `redirects` — open redirect tespiti testi
  - [ ] `rewrites` — path traversal tespiti testi
  - [ ] `dns` — dangling CNAME tespiti testi
  - [ ] `ssl` — sertifika dogrulama testi
  - [ ] `serverless` — injection zafiyet tespiti testi
  - [ ] `middleware` — bypass senaryosu tespiti testi
  - [ ] `preview_auth` — yetkisiz erisim tespiti testi
  - [ ] `build_logs` — hassas bilgi tespiti testi
- [ ] Remediation modul testleri:
  - [ ] `recommendations.rs` — dogru oneri uretimi testi
  - [ ] `templates.rs` — sablon cikti dogrulama testi
  - [ ] `scoring.rs` — skor hesaplama testi
- [ ] Summary modul testleri:
  - [ ] `vercel_config.rs` — vercel.json parse testi (gecerli/gecersiz)
  - [ ] `next_config.rs` — next.config.js analiz testi
  - [ ] `platform_defaults.rs` — varsayilan degerlerin dogrulugu
- [ ] Veritabani CRUD testleri:
  - [ ] Scan CRUD (olustur, oku, guncelle, sil)
  - [ ] Finding CRUD
  - [ ] Report CRUD
  - [ ] Migration testi (sema yukariya/asagiya)

### 8.3 Snapshot Testleri (insta crate)
- [ ] Analyzer ciktilarini snapshot olarak kaydet:
  - [ ] Her analyzer icin "temiz yapilandirma" snapshot
  - [ ] Her analyzer icin "zafiyetli yapilandirma" snapshot
- [ ] Rapor ciktisi snapshot testi:
  - [ ] JSON rapor formati snapshot
  - [ ] HTML rapor formati snapshot
- [ ] Vercel config parser snapshot:
  - [ ] Ornek vercel.json parse sonucu snapshot
  - [ ] Ornek next.config.js parse sonucu snapshot
- [ ] Snapshot guncelleme komutu: `cargo insta review`

### 8.4 Property-Based Testler (proptest)
- [ ] Vercel config parser fuzzing:
  - [ ] Rastgele vercel.json olustur, parser crash etmemeli
  - [ ] Rastgele header degerleri, header analyzer crash etmemeli
  - [ ] Rastgele URL redirect pattern'leri, redirect analyzer crash etmemeli
- [ ] Risk skorlama:
  - [ ] Rastgele bulgu kombinasyonlari, skor her zaman 0-100 arasi olmali
  - [ ] Bos bulgu listesi, skor 100 (temiz) olmali
- [ ] Env exposure:
  - [ ] Rastgele env degisken isimleri, NEXT_PUBLIC_ prefix dogru tespit edilmeli

### 8.5 Rust Entegrasyon Testleri (`tests/integration/`)
- [ ] Tam tarama pipeline testi:
  - [ ] Mock Vercel API baslat → tum analyzer'lari calistir → rapor uret → dogrula
- [ ] Veritabani entegrasyonu:
  - [ ] Tarama yap → sonuclari DB'ye kaydet → DB'den oku → dogrula
- [ ] API client + Analyzer entegrasyonu:
  - [ ] API'dan veri cek → analyzer'dan gecir → sonuc dogrula
- [ ] Rapor export entegrasyonu:
  - [ ] Tarama sonuclari → JSON dosyaya yaz → dosyayi oku → dogrula
  - [ ] Tarama sonuclari → HTML dosyaya yaz → HTML valid mi kontrol et
- [ ] CLI entegrasyon testi (`assert_cmd`):
  - [ ] `full_audit` komutu dogru cikti uretiyor mu
  - [ ] Hatali token ile dogru hata mesaji veriyor mu
  - [ ] `--help` flag'i cikti dogrulama

### 8.6 Frontend Unit Testleri
- [ ] Test framework kur:
  - [ ] `bun:test` (Bun native test runner) yapilandir
  - [ ] `@testing-library/dom` ekle
  - [ ] `@testing-library/user-event` ekle
  - [ ] `happy-dom` / `jsdom` ekle (DOM simulasyonu)
- [ ] Component testleri:
  - [ ] Dashboard component testi
  - [ ] Risk matrisi component testi
  - [ ] Tarama sonuclari tablosu testi
  - [ ] Ayarlar formu testi
  - [ ] Rapor goruntuleme testi
- [ ] Tauri invoke mock:
  - [ ] `@tauri-apps/api/core` mock'u olustur
  - [ ] Her komut icin mock response tanimla
  - [ ] Hata senaryolari (Tauri backend erisim yok)
- [ ] State management testleri:
  - [ ] Store/state guncellenme testi
  - [ ] API cagri sonucu state'e yansima testi

### 8.7 E2E (End-to-End) Testleri
- [ ] E2E framework kur:
  - [ ] Playwright kur (`bun add -D @playwright/test`)
  - [ ] `playwright.config.ts` olustur:
    - [ ] Tauri webview URL (localhost:1420)
    - [ ] Screenshot on failure
    - [ ] Video recording (basarisiz testler)
    - [ ] Trace recording
    - [ ] Timeout: 30 saniye
    - [ ] Retries: 1 (CI'da 2)
  - [ ] WebDriver / Tauri driver kurulumu (native pencere testi icin)
- [ ] E2E test senaryolari:
  - [ ] **Uygulama acilis testi**:
    - [ ] Uygulama aciliyor mu
    - [ ] Dashboard yukleniyor mu
    - [ ] Ilk yukleme suresi < 3 saniye
  - [ ] **API Token giris akisi**:
    - [ ] Ayarlar sayfasina git
    - [ ] Token gir ve kaydet
    - [ ] Gecersiz token hata mesaji goster
    - [ ] Token sonrasi proje listesi yukleniyor mu
  - [ ] **Tam tarama akisi (golden path)**:
    - [ ] Proje sec
    - [ ] "Tarama Baslat" butonuna tikla
    - [ ] Ilerleme gostergesi gorunuyor mu
    - [ ] Tarama tamamlandi bildirimi
    - [ ] Sonuclar sayfasinda bulgular listelenyor mu
    - [ ] Risk seviyeleri dogru renkte gorunuyor mu
  - [ ] **Bulgu filtreleme**:
    - [ ] Kritik filtresi
    - [ ] Yuksek filtresi
    - [ ] Orta filtresi
    - [ ] Dusuk filtresi
    - [ ] Tum filtreler acik/kapali
  - [ ] **Sertlestirme rehberi akisi**:
    - [ ] Bulguya tikla → duzeltme onerisi gorunuyor mu
    - [ ] Adim adim checklist isletilebiliyor mu
    - [ ] Yapilandirma karsilastirmasi dogru gorunuyor mu
  - [ ] **Rapor disa aktarma akisi**:
    - [ ] JSON export butonu → dosya indiriliyor mu
    - [ ] HTML export butonu → dosya indiriliyor mu
    - [ ] Export edilen dosya icerigi dogru mu
  - [ ] **Hata senaryolari**:
    - [ ] API erisim yok (network hatasi) → hata mesaji
    - [ ] Token suresi dolmus (401) → yeniden giris yonlendirme
    - [ ] Bos proje listesi → "proje bulunamadi" mesaji
  - [ ] **Responsive UI testi**:
    - [ ] 1920x1080 gorunumu
    - [ ] 1366x768 gorunumu
    - [ ] 1024x768 gorunumu
- [ ] E2E test verileri:
  - [ ] Mock Vercel API server olustur (MSW — Mock Service Worker):
    - [ ] `/v9/projects` endpoint mock
    - [ ] `/v13/deployments` endpoint mock
    - [ ] `/v9/projects/:id/env` endpoint mock
    - [ ] Hata response mock'lari (401, 403, 429, 500)
  - [ ] Fixture dosyalari (`tests/fixtures/`):
    - [ ] `valid_projects.json` — ornek proje listesi
    - [ ] `valid_deployments.json` — ornek deployment listesi
    - [ ] `vulnerable_vercel.json` — zafiyetli yapilandirma
    - [ ] `secure_vercel.json` — guvenli yapilandirma
    - [ ] `mixed_findings.json` — karisik bulgu seti

### 8.8 Benchmark Testleri (criterion)
- [ ] `benches/` dizini olustur:
  - [ ] `analyzer_bench.rs` — her analyzer'in tarama suresi
  - [ ] `parser_bench.rs` — vercel.json / next.config.js parse suresi
  - [ ] `report_bench.rs` — rapor olusturma suresi
  - [ ] `db_bench.rs` — veritabani okuma/yazma suresi
  - [ ] `full_scan_bench.rs` — tam audit suresi (tum analyzer'lar)
- [ ] Benchmark baseline olustur (ilk olcum)
- [ ] Benchmark CI'da calistir, regression tespit et

### 8.9 Kod Coverage
- [ ] `cargo-llvm-cov` yapilandir:
  - [ ] `cargo llvm-cov --html` — HTML coverage raporu
  - [ ] `cargo llvm-cov --lcov --output-path lcov.info` — lcov formati
  - [ ] Coverage threshold belirle: minimum %70 satir, %60 branch
- [ ] Frontend coverage:
  - [ ] `bun test --coverage` yapilandir
  - [ ] Coverage threshold: minimum %60
- [ ] Coverage raporunu CI'da olustur ve artifact olarak sakla
- [ ] Codecov / Coveralls entegrasyonu (opsiyonel)

### 8.10 Test Araclari & Yardimcilar
- [ ] Mock Vercel API server (`tests/mocks/vercel_mock_server.rs`):
  - [ ] `wiremock` ile tam mock API
  - [ ] Tum endpoint'ler icin response tanimlari
  - [ ] Dinamik response (istek parametresine gore)
  - [ ] Rate limit simulasyonu
  - [ ] Gecikme simulasyonu (yavas API testi)
- [ ] Test fixture generator (`tests/fixtures/generator.rs`):
  - [ ] Rastgele vercel.json uret
  - [ ] Rastgele next.config.js uret
  - [ ] Rastgele env degisken seti uret
  - [ ] Belirli zafiyet profili ile fixture uret
- [ ] Test reporter:
  - [ ] JUnit XML cikti (CI entegrasyonu)
  - [ ] HTML test raporu (yerel gelistirme)
  - [ ] Terminal ozet raporu (cargo-nextest)

---

## PHASE 9: CI/CD Pipeline

### 9.1 GitHub Actions — Ana CI Pipeline
- [ ] `.github/workflows/ci.yml` olustur:
  - [ ] **Trigger**: push (main, develop), pull_request
  - [ ] **Cache**:
    - [ ] Rust target/ dizini cache (`actions/cache`)
    - [ ] `~/.cargo/registry` cache
    - [ ] `sccache` cache
    - [ ] `node_modules/` cache (Bun)
    - [ ] Docker layer cache
  - [ ] **Job: lint**
    - [ ] `cargo fmt --check` — formatlama kontrolu
    - [ ] `cargo clippy -- -D warnings` — lint (uyarilar hata sayilsin)
    - [ ] `cargo deny check` — lisans ve zafiyet kontrolu
    - [ ] `cargo machete` — kullanilmayan bagimlilik kontrolu
    - [ ] Frontend lint (eslint, prettier)
  - [ ] **Job: test-unit**
    - [ ] `cargo nextest run --profile ci -E 'kind(lib) | kind(bin)'` — unit testler
    - [ ] JUnit XML cikti → test summary
    - [ ] Coverage raporu olustur
  - [ ] **Job: test-integration**
    - [ ] `cargo nextest run --profile ci -E 'kind(test)'` — entegrasyon testleri
    - [ ] Mock API server ile calistir
    - [ ] JUnit XML cikti
  - [ ] **Job: test-frontend**
    - [ ] `bun test` — frontend unit testler
    - [ ] Coverage raporu
  - [ ] **Job: test-e2e**
    - [ ] Docker compose ile test ortami kur
    - [ ] `bun run test:e2e` — Playwright E2E testler
    - [ ] Basarisiz test screenshot/video → artifact
    - [ ] Playwright raporu → artifact
  - [ ] **Job: build**
    - [ ] `cargo build --release` — Rust release build
    - [ ] `bun run build` — frontend build
    - [ ] `cargo tauri build` — Tauri bundle (depends on: lint, test-unit)
    - [ ] Build artifact'lerini yukle
  - [ ] **Job: security-audit**
    - [ ] `cargo audit` — bilinen Rust zafiyet kontrolu
    - [ ] `bun audit` — npm zafiyet kontrolu (varsa)
    - [ ] SAST tarama (semgrep veya cargo-geiger)
  - [ ] **Job: benchmark** (sadece main branch'e merge'de)
    - [ ] `cargo bench` calistir
    - [ ] Benchmark sonuclarini onceki ile karsilastir
    - [ ] Regression varsa uyar

### 9.2 GitHub Actions — Release Pipeline
- [ ] `.github/workflows/release.yml` olustur:
  - [ ] **Trigger**: tag push (`v*`)
  - [ ] **Matrix build**:
    - [ ] Windows (x86_64-pc-windows-msvc)
    - [ ] Linux (x86_64-unknown-linux-gnu)
    - [ ] macOS Intel (x86_64-apple-darwin)
    - [ ] macOS ARM (aarch64-apple-darwin)
  - [ ] Her platform icin Tauri bundle olustur
  - [ ] GitHub Release olustur ve artifact'leri ekle
  - [ ] Changelog otomatik olustur (git-cliff veya conventional-changelog)

### 9.3 GitHub Actions — Dependency Update
- [ ] `.github/workflows/deps.yml` (haftalik):
  - [ ] `cargo update` calistir
  - [ ] `bun update` calistir
  - [ ] Degisiklik varsa PR ac
  - [ ] CI testlerini otomatik calistir

### 9.4 GitHub Actions — Docker
- [ ] `.github/workflows/docker.yml`:
  - [ ] Docker image build
  - [ ] Docker image test (container icinde test calistir)
  - [ ] Docker image push (ghcr.io veya Docker Hub)
  - [ ] Image guvenlik taramasi (trivy)

### 9.5 Pre-commit & Git Hooks
- [ ] `lefthook.yml` veya `.husky/` yapilandir:
  - [ ] `pre-commit`:
    - [ ] `cargo fmt --check`
    - [ ] `cargo clippy -- -D warnings`
    - [ ] Frontend lint
  - [ ] `pre-push`:
    - [ ] `cargo nextest run --profile default`
    - [ ] `bun test`
  - [ ] `commit-msg`:
    - [ ] Conventional commit format kontrolu

### 9.6 CI Performans Optimizasyonlari
- [ ] Rust incremental compilation CI'da aktif et
- [ ] `sccache` CI'da yapilandir (S3/GCS backend)
- [ ] Parallel job sayisini optimize et
- [ ] Test splitting (buyuk test suite'leri paralel job'lara bol)
- [ ] Gereksiz job'lari atla (path-based filtering):
  - [ ] Sadece `src-tauri/` degistiyse Rust testlerini calistir
  - [ ] Sadece `src/` degistiyse frontend testlerini calistir
  - [ ] Sadece `docs/` degistiyse hicbir test calistirma

---

## PHASE 10: Performans & Surec Hizlandirma

### 10.1 Derleme Hizlandirma
- [ ] `sccache` yapilandir (Rust compilation cache):
  - [ ] Yerel: disk cache
  - [ ] CI: S3/GCS remote cache (paylasilmis)
- [ ] Linker degistir:
  - [ ] Linux: `mold` linker (`-C link-arg=-fuse-ld=mold`)
  - [ ] Windows: `lld` linker
  - [ ] macOS: `lld` veya `zld`
- [ ] `.cargo/config.toml` optimize et:
  - [ ] `[build] jobs = 8` (paralel derleme is sayisi)
  - [ ] `[profile.dev] opt-level = 0` (dev build hizi)
  - [ ] `[profile.dev.package."*"] opt-level = 2` (bagimliliklar optimize)
  - [ ] `[profile.test] opt-level = 1` (test build dengeli)
  - [ ] `[profile.release] lto = "thin"` (release build boyutu)
  - [ ] `[profile.release] codegen-units = 1` (release optimizasyon)
  - [ ] `[profile.release] strip = true` (binary boyutu kucult)
- [ ] Workspace yapisi degerlendir (crate splitting ile paralel derleme artir)
- [ ] `cargo build --timings` ile derleme darbogazlarini tespit et

### 10.2 Test Hizlandirma
- [ ] `cargo-nextest` ile paralel test calistir (varsayilan: CPU cekirdek sayisi kadar thread)
- [ ] Test gruplari tanimla (birbiriyle catisan testler seri, diger testler paralel)
- [ ] Test veritabani: in-memory SQLite kullan (disk I/O sifir)
- [ ] Mock server: `wiremock` ile ayni process icinde (network overhead yok)
- [ ] Fixture'lari lazy_static ile bir kez yukle, testler arasi paylas
- [ ] Buyuk entegrasyon testlerini `#[ignore]` ile isaretle, CI'da ayri job'da calistir
- [ ] Flaky test tespiti: `cargo-nextest` retry + raporlama

### 10.3 Frontend Hizlandirma
- [ ] Bun native bundler kullan (webpack/vite yerine — daha hizli)
- [ ] Hot module replacement (HMR) yapilandir
- [ ] Dev server: Bun ile (Node.js'den hizli)
- [ ] Tree shaking aktif (kullanilmayan kod cikar)

### 10.4 Tarama Hizlandirma (Runtime)
- [ ] `rayon` ile paralel analyzer calistirma:
  - [ ] Tum analyzer'lar ayni anda tarama yapsin
  - [ ] Sonuclari topla ve birlestir
- [ ] `reqwest` connection pool (ayni sunucuya tekrar baglanti acma)
- [ ] HTTP/2 multiplexing aktif et (birden fazla istek ayni baglanti)
- [ ] Async I/O: tum disk ve network islemleri async (`tokio`)
- [ ] Result cache: ayni URL'ye tekrar istek atma (`dashmap` ile TTL cache)

---

## PHASE 11: Derleme & Dagitim

- [ ] `tauri build` ile release build test et
- [ ] Windows installer (.msi / .exe) olustur
- [ ] Linux AppImage / .deb olustur
- [ ] macOS .dmg olustur (varsa)
- [ ] Auto-update mekanizmasi yapilandir (Tauri updater)
- [ ] Uygulama ikonu ve splash screen

---

## PHASE 12: Dokumantasyon (tasks.md Arastirma Ciktilari)

- [ ] `README.md` olustur:
  - [ ] Proje aciklamasi
  - [ ] Kurulum adimlari
  - [ ] Kullanim kilavuzu
  - [ ] Ekran goruntuleri
  - [ ] Test calistirma komutlari
  - [ ] CI/CD badge'leri (build, test, coverage)
- [ ] `CONTRIBUTING.md` olustur:
  - [ ] Gelistirme ortami kurulumu
  - [ ] Test yazma rehberi
  - [ ] Commit mesaj formati
  - [ ] PR sureci
- [ ] `docs/technical-summary.md` — Phase 1 teknik ozet raporu
- [ ] `docs/attack-vectors.md` — Phase 2 saldiri vektorleri analizi
- [ ] `docs/hardening-guide.md` — Phase 3 sertlestirme rehberi
- [ ] `docs/scripts-and-configs.md` — Phase 4 script/konfigurasyon dokumantasyonu
- [ ] `docs/testing.md` — Test stratejisi ve nasil test yazilir
- [ ] `docs/ci-cd.md` — CI/CD pipeline dokumantasyonu
- [ ] `tasks.md` icerisindeki "Your Research Here" bolumunu doldur:
  - [ ] Teknik ozet sonuclarini yaz
  - [ ] Saldiri vektorleri ve riskleri listele
  - [ ] Adim adim sertlestirme rehberini ekle
  - [ ] Kullanilan tum script, config ve komutlari ekle

---

## PHASE 13: Son Kontroller

### 13.1 Guvenlik Dogrulama
- [ ] Tum `.env` dosyalari `.gitignore`'da mi? Dogrula
- [ ] Hassas bilgi sizintisi taramasi (`cargo audit`, `bun audit`)
- [ ] `cargo deny check` basarili mi?
- [ ] Docker image guvenlik taramasi (trivy)
- [ ] SAST tarama sonuclari temiz mi?

### 13.2 Build Dogrulama
- [ ] `cargo build --release` basarili mi?
- [ ] `bun run build` basarili mi?
- [ ] `cargo tauri build` basarili mi?
- [ ] Docker container build & run basarili mi?
- [ ] Cross-platform derleme basarili mi? (Windows, Linux, macOS)

### 13.3 Test Dogrulama
- [ ] `cargo nextest run` — tum Rust testler gecti mi?
- [ ] `bun test` — tum frontend testler gecti mi?
- [ ] `bun run test:e2e` — tum E2E testler gecti mi?
- [ ] `cargo bench` — benchmark'lar regression gostermiyor mu?
- [ ] Coverage threshold'lar karsilaniyor mu? (Rust >= %70, Frontend >= %60)
- [ ] Snapshot testler guncel mi? (`cargo insta review`)

### 13.4 CI/CD Dogrulama
- [ ] CI pipeline basarili calisyor mu? (push & PR trigger)
- [ ] Release pipeline tag ile tetiklenip artifact uretiyor mu?
- [ ] Docker pipeline image build & push yapiyor mu?
- [ ] Cache'ler dogru calisiyor mu? (ikinci calistirma daha hizli mi?)
- [ ] Path-based filtering dogru calisiyor mu?

### 13.5 Fonksiyonel Dogrulama
- [ ] Uygulama acilip calisiyor mu? (smoke test)
- [ ] Vercel API baglantisi calisiyor mu?
- [ ] Tum analiz modulleri dogru sonuc uretiyor mu?
- [ ] Sertlestirme onerileri dogru mu?
- [ ] Rapor disa aktarma calisiyor mu?
- [ ] `tasks.md` tamamen dolduruldu mu?

---

> **Not**: Her maddeyi tamamladikca `[ ]` -> `[x]` olarak isaretle.
> Phase sirasina uy — PHASE 0'dan basla, PHASE 13'e kadar ilerle.
> PHASE 0-4: tasks.md talimatlarina karsilik gelir.
> PHASE 8-10: Test, CI/CD ve performans — bunlari PHASE 5-7 ile paralel ilerletebilirsin.
