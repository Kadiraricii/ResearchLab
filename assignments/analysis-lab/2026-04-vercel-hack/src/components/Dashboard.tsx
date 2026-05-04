// Dashboard — Ana Sayfa
// Shows incident context, analyzer capabilities, and navigation CTAs.

interface DashboardProps {
  onNavigate: (tab: string) => void;
}

const STATS = [
  { num: "12", label: "Analiz Modülü", sub: "Paralel çalışır (rayon)" },
  { num: "6",  label: "Zafiyet Kategorisi", sub: "CORS, SSL, DNS, SSRF…" },
  { num: "20+", label: "Zafiyet ID'si", sub: "ENV_01 – BLDL_02" },
  { num: "0ms", label: "Ağ Gecikmesi", sub: "Tamamen yerel analiz" },
];

const VECTORS = [
  { id: "ENV",  color: "var(--critical)", label: "Env Maruziyeti",       desc: "NEXT_PUBLIC_ ve hassas token tespiti" },
  { id: "HDR",  color: "var(--high)",     label: "Güvenlik Başlıkları",  desc: "CSP, HSTS, X-Frame-Options eksikliği" },
  { id: "CORS", color: "var(--high)",     label: "CORS Yapılandırması",  desc: "Wildcard origin ve credentials riski" },
  { id: "REDIR",color: "var(--high)",     label: "Open Redirect",        desc: "Kullanıcı kontrollü dış yönlendirme" },
  { id: "REWR", color: "var(--high)",     label: "SSRF via Rewrites",    desc: "İç ağ IP/domain proxy tespiti" },
  { id: "DNS",  color: "var(--medium)",   label: "DNS / HTTPS",          desc: "HTTP→HTTPS zorlaması ve wildcard route" },
  { id: "SSL",  color: "var(--medium)",   label: "SSL / HSTS",           desc: "max-age düşüklüğü, includeSubDomains" },
  { id: "SRVL", color: "var(--medium)",   label: "Serverless",           desc: "Deprecated runtime, yüksek maxDuration" },
  { id: "MDLW", color: "var(--medium)",   label: "Middleware",           desc: "Auth zinciri atlama, korumasız rota" },
  { id: "PREV", color: "var(--low)",      label: "Preview Auth",         desc: "public:true, github entegrasyon devre dışı" },
  { id: "BLDL", color: "var(--low)",      label: "Build Logs",           desc: "Hardcoded secret, env bölümünde açık değer" },
  { id: "SRMP", color: "var(--low)",      label: "Source Maps",          desc: "Üretimde .map dosyası maruziyeti" },
];

export function Dashboard({ onNavigate }: DashboardProps) {
  return (
    <div className="page">
      {/* ── Incident badge ── */}
      <div className="incident-badge">
        ● APRIL 2026 — VERCEL HACK / CONTEXT.AI SUPPLY CHAIN INCIDENT
      </div>

      {/* ── Hero ── */}
      <div className="ph">
        <h1 className="ph-title">Vercel Security Analyzer</h1>
        <p className="ph-desc" style={{ maxWidth: 600 }}>
          Nisan 2026'daki <strong style={{ color: "var(--text)" }}>context-ai</strong> tedarik
          zinciri saldırısını inceleyen bu araç; Vercel proje yapılandırmalarını 12 saldırı
          vektörüne karşı yerel olarak tarar, zafiyet skorlar ve güvenli şablonlar üretir.
        </p>
      </div>

      {/* ── Stats ── */}
      <div className="stat-grid">
        {STATS.map((s) => (
          <div key={s.label} className="stat-card">
            <span className="stat-card-num" style={{ color: "var(--accent)" }}>{s.num}</span>
            <span className="stat-card-lbl">{s.label}</span>
            <span className="stat-card-sub">{s.sub}</span>
          </div>
        ))}
      </div>

      {/* ── Info box ── */}
      <div className="info-box">
        <span className="info-box-icon">
          <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor">
            <path d="M8 0a8 8 0 1 0 0 16A8 8 0 0 0 8 0zm.75 12h-1.5V7h1.5v5zm0-6.5h-1.5v-1.5h1.5v1.5z"/>
          </svg>
        </span>
        <span>
          <strong style={{ color: "var(--text)" }}>Saldırı Mekanizması:</strong>{" "}
          context-ai v2.4.1 paketi, katkıda bulunanın ele geçirilen kimlik bilgileri
          ile zehirlendi. Kötü amaçlı kod, Vercel dashboard'da <em>Sensitive</em> olarak
          işaretlenmemiş ortam değişkenlerini tarayarak dışarı sızdırdı. Bu araç aynı
          yapılandırma hatalarını statik analiz ile tespit eder.
        </span>
      </div>

      {/* ── Risk matrix ── */}
      <div className="card" style={{ marginBottom: 16 }}>
        <p className="card-label">Risk Matrisi — 12 Analiz Modülü</p>
        <div style={{ display: "grid", gridTemplateColumns: "repeat(3, 1fr)", gap: 8 }}>
          {VECTORS.map((v) => (
            <div
              key={v.id}
              style={{
                background: "var(--surface-2)",
                border: `1px solid var(--border-sub)`,
                borderLeft: `3px solid ${v.color}`,
                borderRadius: 5,
                padding: "10px 12px",
              }}
            >
              <div style={{ display: "flex", alignItems: "center", gap: 8, marginBottom: 4 }}>
                <span style={{ fontFamily: "var(--mono)", fontSize: "var(--t-xs)", color: v.color, fontWeight: 700 }}>{v.id}</span>
                <span style={{ fontSize: "var(--t-xs)", fontWeight: 600, color: "var(--text)" }}>{v.label}</span>
              </div>
              <p style={{ fontSize: "var(--t-xs)", color: "var(--text-subtle)", margin: 0, lineHeight: 1.45 }}>{v.desc}</p>
            </div>
          ))}
        </div>
      </div>

      {/* ── CTAs ── */}
      <div style={{ display: "flex", gap: 10, flexWrap: "wrap" }}>
        <button className="btn btn-primary" onClick={() => onNavigate("scanner")}>
          <svg width="13" height="13" viewBox="0 0 16 16" fill="none">
            <circle cx="7" cy="7" r="5.5" stroke="currentColor" strokeWidth="1.6" />
            <path d="M11.5 11.5L14.5 14.5" stroke="currentColor" strokeWidth="1.6" strokeLinecap="round" />
          </svg>
          Taramaya Başla
        </button>
        <button className="btn btn-outline" onClick={() => onNavigate("settings")}>
          API Token Ayarla
        </button>
        <button className="btn btn-outline" onClick={() => onNavigate("hardening")}>
          Sertleştirme Rehberi
        </button>
      </div>
    </div>
  );
}
