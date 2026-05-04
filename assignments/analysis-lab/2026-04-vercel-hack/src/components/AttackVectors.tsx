import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

// ─── Types ────────────────────────────────────────────────────────────────────

type RiskLevel = "Critical" | "High" | "Medium" | "Low" | "Info";

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

// ─── Constants ────────────────────────────────────────────────────────────────

const SORT_ORDER: Record<RiskLevel, number> = {
  Critical: 0,
  High: 1,
  Medium: 2,
  Low: 3,
  Info: 4,
};

const SCORE_LABEL: (s: number) => string = (s) =>
  s >= 8 ? "Kritik" : s >= 6 ? "Yüksek" : s >= 4 ? "Orta" : s > 0 ? "Düşük" : "Temiz";

const SCORE_COLOR: (s: number) => string = (s) =>
  s >= 8
    ? "var(--critical)"
    : s >= 6
    ? "var(--high)"
    : s >= 4
    ? "var(--medium)"
    : "var(--low)";

// ─── Sub-components ───────────────────────────────────────────────────────────

function ScoreRing({ score }: { score: number }) {
  const r = 44;
  const circ = 2 * Math.PI * r;
  const offset = circ - (score / 10) * circ;
  const color = SCORE_COLOR(score);

  return (
    <div className="score-svg-wrap">
      <svg
        viewBox="0 0 100 100"
        className="score-svg"
        style={{ width: "100%", height: "100%" }}
      >
        <circle cx="50" cy="50" r={r} className="ring-bg" />
        <circle
          cx="50"
          cy="50"
          r={r}
          className="ring-fill"
          style={{ stroke: color, strokeDasharray: circ, strokeDashoffset: offset }}
        />
      </svg>
      <div className="score-inner">
        <span className="score-num" style={{ color }}>
          {score.toFixed(1)}
        </span>
        <span className="score-cap">{SCORE_LABEL(score)}</span>
      </div>
    </div>
  );
}

function RiskBadge({ level }: { level: RiskLevel }) {
  return (
    <span className={`badge badge-${level.toLowerCase()}`}>{level}</span>
  );
}

function VulnCard({ v }: { v: Vulnerability }) {
  const cls = v.risk_level.toLowerCase();
  return (
    <div className="vuln-card">
      <div className={`vuln-stripe stripe-${cls}`} />
      <div className="vuln-head">
        <span className="vuln-id">{v.id}</span>
        <span className="vuln-title">{v.title}</span>
        <RiskBadge level={v.risk_level} />
      </div>
      <p className="vuln-desc">{v.description}</p>
      <div className="vuln-foot">
        <div className="vuln-meta">
          <span className="meta-k">Bileşen</span>
          <span className="meta-v">{v.affected_component}</span>
        </div>
        <div className="vuln-meta" style={{ flex: 1 }}>
          <span className="meta-k">Çözüm</span>
          <span className="meta-v fix">{v.remediation}</span>
        </div>
      </div>
    </div>
  );
}

function ScanIcon() {
  return (
    <svg width="13" height="13" viewBox="0 0 16 16" fill="none">
      <circle cx="7" cy="7" r="5.5" stroke="currentColor" strokeWidth="1.6" />
      <path d="M11.5 11.5L14.5 14.5" stroke="currentColor" strokeWidth="1.6" strokeLinecap="round" />
      <path d="M5 7L7 9L10 5" stroke="currentColor" strokeWidth="1.4" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  );
}

// ─── Main component ───────────────────────────────────────────────────────────

export function AttackVectors() {
  const [vercelJson, setVercelJson]   = useState("");
  const [nextConfig, setNextConfig]   = useState("");
  const [envContent, setEnvContent]   = useState("");
  const [report, setReport]           = useState<AnalysisReport | null>(null);
  const [loading, setLoading]         = useState(false);
  const [error, setError]             = useState<string | null>(null);

  const handleScan = async () => {
    setLoading(true);
    setError(null);
    try {
      // Command name matches lib.rs: run_analysis(vercel_json, next_config, env_content)
      const res = await invoke<AnalysisReport>("run_analysis", {
        vercelJson,
        nextConfig,
        envContent,
      });
      setReport(res);
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

  const counts = {
    critical: sorted.filter((v) => v.risk_level === "Critical").length,
    high:     sorted.filter((v) => v.risk_level === "High").length,
    medium:   sorted.filter((v) => v.risk_level === "Medium").length,
    low:      sorted.filter((v) => v.risk_level === "Low").length,
  };

  return (
    <div className="page">
      {/* ── Page header ── */}
      <div className="ph">
        <h1 className="ph-title">Zafiyet Tarayıcı</h1>
        <p className="ph-desc">
          12 saldırı vektörüne karşı Vercel yapılandırmalarını analiz edin.
        </p>
      </div>

      {/* ── Input card ── */}
      <div className="card">
        <p className="card-label">Yapılandırma Dosyaları</p>
        <div className="input-grid">
          <div className="input-group">
            <label className="input-label">
              <span className="file-tag">vercel.json</span>
            </label>
            <textarea
              className="code-ta"
              placeholder={'{\n  "headers": [],\n  "redirects": []\n}'}
              value={vercelJson}
              onChange={(e) => setVercelJson(e.target.value)}
            />
          </div>
          <div className="input-group">
            <label className="input-label">
              <span className="file-tag">next.config.js</span>
            </label>
            <textarea
              className="code-ta"
              placeholder={"module.exports = {\n  poweredByHeader: false\n}"}
              value={nextConfig}
              onChange={(e) => setNextConfig(e.target.value)}
            />
          </div>
          <div className="input-group">
            <label className="input-label">
              <span className="file-tag">.env.local</span>
            </label>
            <textarea
              className="code-ta"
              placeholder={"NEXT_PUBLIC_API_KEY=...\nSECRET_TOKEN=..."}
              value={envContent}
              onChange={(e) => setEnvContent(e.target.value)}
            />
          </div>
        </div>

        <button
          className="btn btn-primary"
          onClick={handleScan}
          disabled={loading}
        >
          {loading ? <span className="spin" /> : <ScanIcon />}
          {loading ? "Taranıyor…" : "Sistemi Tara"}
        </button>
      </div>

      {/* ── Error ── */}
      {error && (
        <div className="err-bar">
          <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor" style={{ flexShrink: 0, marginTop: 1 }}>
            <path d="M8 1a7 7 0 1 0 0 14A7 7 0 0 0 8 1zm.75 10.25h-1.5v-1.5h1.5v1.5zm0-3h-1.5v-4h1.5v4z" />
          </svg>
          {error}
        </div>
      )}

      {/* ── Report ── */}
      {report && (
        <>
          <div className="div" />

          {/* Score + summary */}
          <div className="score-wrap">
            <ScoreRing score={report.score} />
            <div className="score-meta">
              <h2>
                {report.vulnerabilities.length === 0
                  ? "Zafiyet Bulunamadı"
                  : `${report.vulnerabilities.length} Zafiyet Tespit Edildi`}
              </h2>
              <p>
                {report.vulnerabilities.length === 0
                  ? "Tebrikler — taranan yapılandırmalarda bilinen bir güvenlik açığı bulunamadı."
                  : "Bulgular önem sırasına göre listelendi. Kritik ve Yüksek seviyeli bulguları öncelikli olarak giderin."}
              </p>

              {report.vulnerabilities.length > 0 && (
                <div className="sev-counts">
                  {counts.critical > 0 && (
                    <div className="sev-count">
                      <span className="sev-n" style={{ color: "var(--critical)" }}>{counts.critical}</span>
                      <span className="sev-lbl">Kritik</span>
                    </div>
                  )}
                  {counts.high > 0 && (
                    <div className="sev-count">
                      <span className="sev-n" style={{ color: "var(--high)" }}>{counts.high}</span>
                      <span className="sev-lbl">Yüksek</span>
                    </div>
                  )}
                  {counts.medium > 0 && (
                    <div className="sev-count">
                      <span className="sev-n" style={{ color: "var(--medium)" }}>{counts.medium}</span>
                      <span className="sev-lbl">Orta</span>
                    </div>
                  )}
                  {counts.low > 0 && (
                    <div className="sev-count">
                      <span className="sev-n" style={{ color: "var(--low)" }}>{counts.low}</span>
                      <span className="sev-lbl">Düşük</span>
                    </div>
                  )}
                </div>
              )}
            </div>
          </div>

          {/* Vuln list */}
          {report.vulnerabilities.length === 0 ? (
            <div className="empty">
              <div className="empty-icon">✓</div>
              <h3>Güvenli Yapılandırma</h3>
              <p>12 saldırı vektörünün hiçbirinde güvenlik açığı tespit edilmedi.</p>
            </div>
          ) : (
            <div className="vulns">
              {sorted.map((v, i) => (
                <VulnCard key={`${v.id}-${i}`} v={v} />
              ))}
            </div>
          )}
        </>
      )}
    </div>
  );
}
