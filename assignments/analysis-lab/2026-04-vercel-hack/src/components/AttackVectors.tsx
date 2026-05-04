import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

// ─── Types ────────────────────────────────────────────────────────────────────

type RiskLevel = "Critical" | "High" | "Medium" | "Low" | "Info";
type Filter    = "All" | RiskLevel;

interface Vulnerability {
  id: string;
  title: string;
  description: string;
  risk_level: RiskLevel;
  affected_component: string;
  remediation: string;
}

interface AnalysisReport {
  vulnerabilities: Vulnerability[];
  score: number;
}

interface AttackVectorsProps {
  onReportReady: (report: AnalysisReport) => void;
}

// ─── Constants ────────────────────────────────────────────────────────────────

const SORT_ORDER: Record<RiskLevel, number> = {
  Critical: 0, High: 1, Medium: 2, Low: 3, Info: 4,
};

const FILTERS: Filter[] = ["All", "Critical", "High", "Medium", "Low"];

const SCORE_COLOR = (s: number) =>
  s >= 8 ? "var(--critical)" : s >= 6 ? "var(--high)" : s >= 4 ? "var(--medium)" : "var(--low)";

const SCORE_LABEL = (s: number) =>
  s >= 8 ? "Kritik" : s >= 6 ? "Yüksek" : s >= 4 ? "Orta" : s > 0 ? "Düşük" : "Temiz";

// ─── Sub-components ───────────────────────────────────────────────────────────

function ScoreRing({ score }: { score: number }) {
  const r    = 44;
  const circ = 2 * Math.PI * r;
  const offset = circ - (score / 10) * circ;
  const color  = SCORE_COLOR(score);
  return (
    <div className="score-svg-wrap">
      <svg viewBox="0 0 100 100" className="score-svg" style={{ width: "100%", height: "100%" }}>
        <circle cx="50" cy="50" r={r} className="ring-bg" />
        <circle cx="50" cy="50" r={r} className="ring-fill"
          style={{ stroke: color, strokeDasharray: circ, strokeDashoffset: offset }} />
      </svg>
      <div className="score-inner">
        <span className="score-num" style={{ color }}>{score.toFixed(1)}</span>
        <span className="score-cap">{SCORE_LABEL(score)}</span>
      </div>
    </div>
  );
}

function VulnCard({ v }: { v: Vulnerability }) {
  const [expanded, setExpanded] = useState(false);
  const cls = v.risk_level.toLowerCase();
  return (
    <div className="vuln-card" onClick={() => setExpanded((e) => !e)}>
      <div className={`vuln-stripe stripe-${cls}`} />
      <div className="vuln-head">
        <span className="vuln-id">{v.id}</span>
        <span className="vuln-title">{v.title}</span>
        <span className={`badge badge-${cls}`}>{v.risk_level}</span>
        <svg
          width="12" height="12" viewBox="0 0 12 12" fill="none"
          style={{ flexShrink: 0, color: "var(--text-subtle)", transition: "transform 150ms", transform: expanded ? "rotate(180deg)" : "none" }}
        >
          <path d="M2 4L6 8L10 4" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" />
        </svg>
      </div>
      <p className="vuln-desc">{v.description}</p>
      {expanded && (
        <div className="vuln-expand">
          <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 10 }}>
            <div>
              <div className="vuln-expand-title">Etkilenen Bileşen</div>
              <div className="vuln-expand-body">{v.affected_component}</div>
            </div>
            <div>
              <div className="vuln-expand-title">Çözüm Önerisi</div>
              <div className="vuln-expand-body" style={{ color: "var(--low)" }}>{v.remediation}</div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

// ─── Main component ───────────────────────────────────────────────────────────

export function AttackVectors({ onReportReady }: AttackVectorsProps) {
  const [vercelJson, setVercelJson] = useState("");
  const [nextConfig, setNextConfig] = useState("");
  const [envContent, setEnvContent] = useState("");
  const [report, setReport]         = useState<AnalysisReport | null>(null);
  const [filter, setFilter]         = useState<Filter>("All");
  const [loading, setLoading]       = useState(false);
  const [error, setError]           = useState<string | null>(null);

  const handleScan = async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await invoke<AnalysisReport>("run_analysis", {
        vercelJson, nextConfig, envContent,
      });
      setReport(res);
      setFilter("All");
      onReportReady(res);
    } catch (e: unknown) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setLoading(false);
    }
  };

  const sorted = report
    ? [...report.vulnerabilities].sort(
        (a, b) => SORT_ORDER[a.risk_level] - SORT_ORDER[b.risk_level]
      )
    : [];

  const filtered = filter === "All" ? sorted : sorted.filter((v) => v.risk_level === filter);

  const counts: Record<string, number> = {
    Critical: sorted.filter((v) => v.risk_level === "Critical").length,
    High:     sorted.filter((v) => v.risk_level === "High").length,
    Medium:   sorted.filter((v) => v.risk_level === "Medium").length,
    Low:      sorted.filter((v) => v.risk_level === "Low").length,
  };

  return (
    <div className="page">
      <div className="ph">
        <h1 className="ph-title">Zafiyet Tarayıcı</h1>
        <p className="ph-desc">12 saldırı vektörüne karşı Vercel yapılandırmalarını analiz edin.</p>
      </div>

      {/* ── Inputs ── */}
      <div className="card">
        <p className="card-label">Yapılandırma Dosyaları</p>
        <div className="input-grid">
          <div className="input-group">
            <label className="input-label"><span className="file-tag">vercel.json</span></label>
            <textarea className="code-ta"
              placeholder={'{\n  "headers": [],\n  "redirects": []\n}'}
              value={vercelJson} onChange={(e) => setVercelJson(e.target.value)} />
          </div>
          <div className="input-group">
            <label className="input-label"><span className="file-tag">next.config.js</span></label>
            <textarea className="code-ta"
              placeholder={"module.exports = {\n  poweredByHeader: false\n}"}
              value={nextConfig} onChange={(e) => setNextConfig(e.target.value)} />
          </div>
          <div className="input-group">
            <label className="input-label"><span className="file-tag">.env.local</span></label>
            <textarea className="code-ta"
              placeholder={"NEXT_PUBLIC_API_KEY=...\nSECRET_TOKEN=..."}
              value={envContent} onChange={(e) => setEnvContent(e.target.value)} />
          </div>
        </div>
        <button className="btn btn-primary" onClick={handleScan} disabled={loading}>
          {loading ? <span className="spin" /> : (
            <svg width="13" height="13" viewBox="0 0 16 16" fill="none">
              <circle cx="7" cy="7" r="5.5" stroke="currentColor" strokeWidth="1.6" />
              <path d="M11.5 11.5L14.5 14.5" stroke="currentColor" strokeWidth="1.6" strokeLinecap="round" />
            </svg>
          )}
          {loading ? "Taranıyor…" : "Sistemi Tara"}
        </button>
      </div>

      {error && <div className="err-bar">{error}</div>}

      {/* ── Report ── */}
      {report && (
        <>
          <div className="div" />

          {/* Score */}
          <div className="score-wrap">
            <ScoreRing score={report.score} />
            <div className="score-meta">
              <h2>{report.vulnerabilities.length === 0 ? "Zafiyet Bulunamadı" : `${report.vulnerabilities.length} Zafiyet Tespit Edildi`}</h2>
              <p>{report.vulnerabilities.length === 0
                ? "Taranan yapılandırmalarda bilinen bir güvenlik açığı bulunamadı."
                : "Bulgular önem sırasına göre listelendi. Kritik ve Yüksek bulguları öncelikli giderin."}</p>
              {report.vulnerabilities.length > 0 && (
                <div className="sev-counts">
                  {(["Critical", "High", "Medium", "Low"] as const).map((lvl) =>
                    counts[lvl] > 0 ? (
                      <div key={lvl} className="sev-count">
                        <span className="sev-n" style={{ color: `var(--${lvl.toLowerCase()})` }}>{counts[lvl]}</span>
                        <span className="sev-lbl">{{ Critical: "Kritik", High: "Yüksek", Medium: "Orta", Low: "Düşük" }[lvl]}</span>
                      </div>
                    ) : null
                  )}
                </div>
              )}
            </div>
          </div>

          {/* Filter pills */}
          {report.vulnerabilities.length > 0 && (
            <div className="filter-row">
              <span className="filter-lbl">Filtre</span>
              {FILTERS.map((f) => {
                const isActive = filter === f;
                const cnt = f === "All" ? sorted.length : (counts[f] ?? 0);
                if (f !== "All" && cnt === 0) return null;
                return (
                  <button
                    key={f}
                    className={`filter-pill ${isActive ? `active-${f.toLowerCase()}` : ""}`}
                    onClick={() => setFilter(f)}
                  >
                    {f === "All" ? "Tümü" : ({ Critical: "Kritik", High: "Yüksek", Medium: "Orta", Low: "Düşük" } as Record<string, string>)[f] ?? f}
                    <span className="pill-count">{cnt}</span>
                  </button>
                );
              })}
            </div>
          )}

          {/* Vuln list */}
          {filtered.length === 0 ? (
            <div className="empty">
              <div className="empty-icon">✓</div>
              <h3>Güvenli Yapılandırma</h3>
              <p>12 saldırı vektörünün hiçbirinde güvenlik açığı tespit edilmedi.</p>
            </div>
          ) : (
            <div className="vulns">
              {filtered.map((v, i) => <VulnCard key={`${v.id}-${i}`} v={v} />)}
            </div>
          )}
        </>
      )}
    </div>
  );
}
