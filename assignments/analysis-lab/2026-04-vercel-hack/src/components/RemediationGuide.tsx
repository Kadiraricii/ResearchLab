import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

// ─── Types ────────────────────────────────────────────────────────────────────

interface RemediationReport {
  secure_vercel_json: string;
  secure_next_config: string;
  recommendations: string[];
  hardening_score: number;
}

// ─── Sub-components ───────────────────────────────────────────────────────────

function HardeningScore({ score }: { score: number }) {
  const pct   = Math.min((score / 10) * 100, 100);
  const color =
    score >= 7 ? "var(--low)" : score >= 4 ? "var(--medium)" : "var(--critical)";
  const label = score >= 7 ? "İyi" : score >= 4 ? "Orta" : "Zayıf";

  return (
    <>
      <div className="hard-row">
        <span className="hard-label">Sertleştirme Skoru</span>
        <span>
          <span className="hard-score" style={{ color }}>
            {score.toFixed(1)}
          </span>
          <span style={{ fontSize: "var(--t-sm)", color: "var(--text-muted)", marginLeft: 4 }}>
            / 10
          </span>
          <span
            style={{
              marginLeft: 10,
              fontSize: "var(--t-sm)",
              fontWeight: 700,
              color,
            }}
          >
            {label}
          </span>
        </span>
      </div>
      <div className="hard-bar-wrap">
        <div
          className="hard-bar-fill"
          style={{ width: `${pct}%`, background: color }}
        />
      </div>
    </>
  );
}

function FixIcon() {
  return (
    <svg width="13" height="13" viewBox="0 0 16 16" fill="none">
      <path
        d="M3 8.5L6.5 12L13 4"
        stroke="currentColor"
        strokeWidth="1.6"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  );
}

// ─── Main component ───────────────────────────────────────────────────────────

export function RemediationGuide() {
  const [vercelJson, setVercelJson] = useState("");
  const [nextConfig, setNextConfig] = useState("");
  const [envContent, setEnvContent] = useState("");
  const [report, setReport]         = useState<RemediationReport | null>(null);
  const [loading, setLoading]       = useState(false);
  const [error, setError]           = useState<string | null>(null);

  const handleFix = async () => {
    setLoading(true);
    setError(null);
    try {
      // Command name matches lib.rs: generate_remediation_report(vercel_json, next_config, env_content)
      const res = await invoke<RemediationReport>("generate_remediation_report", {
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

  return (
    <div className="page">
      {/* ── Page header ── */}
      <div className="ph">
        <h1 className="ph-title">Sertleştirme Rehberi</h1>
        <p className="ph-desc">
          Mevcut yapılandırmanızı girin; güvenli şablonlar ve öneriler üretelim.
        </p>
      </div>

      {/* ── Input card ── */}
      <div className="card">
        <p className="card-label">Mevcut Yapılandırma</p>
        <div className="input-grid">
          <div className="input-group">
            <label className="input-label">
              <span className="file-tag">vercel.json</span>
            </label>
            <textarea
              className="code-ta"
              placeholder={'{\n  "headers": []\n}'}
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
              placeholder={"module.exports = {}"}
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
              placeholder={"NEXT_PUBLIC_API_KEY=..."}
              value={envContent}
              onChange={(e) => setEnvContent(e.target.value)}
            />
          </div>
        </div>

        <button
          className="btn btn-green"
          onClick={handleFix}
          disabled={loading}
        >
          {loading ? <span className="spin" style={{ borderTopColor: "transparent" }} /> : <FixIcon />}
          {loading ? "Üretiliyor…" : "Güvenli Yapılandırma Üret"}
        </button>
      </div>

      {/* ── Error ── */}
      {error && <div className="err-bar">{error}</div>}

      {/* ── Report ── */}
      {report && (
        <>
          <div className="div" />

          {/* Hardening score */}
          <div className="card" style={{ marginBottom: 12 }}>
            <HardeningScore score={report.hardening_score} />
          </div>

          {/* Recommendations */}
          {report.recommendations.length > 0 && (
            <div className="card" style={{ marginBottom: 12 }}>
              <p className="card-label">Önerilen Aksiyonlar</p>
              <ul className="rec-list">
                {report.recommendations.map((rec, i) => (
                  <li key={i} className="rec-item">
                    <span className="rec-arrow">→</span>
                    {rec}
                  </li>
                ))}
              </ul>
            </div>
          )}

          {/* Secure templates */}
          <div className="two-col">
            <div className="code-block">
              <div className="code-block-hdr">
                <span className="code-block-name">vercel.json — Güvenli Şablon</span>
              </div>
              <div className="code-block-body">
                <pre>{report.secure_vercel_json}</pre>
              </div>
            </div>
            <div className="code-block">
              <div className="code-block-hdr">
                <span className="code-block-name">next.config.js — Güvenli Şablon</span>
              </div>
              <div className="code-block-body">
                <pre>{report.secure_next_config}</pre>
              </div>
            </div>
          </div>
        </>
      )}
    </div>
  );
}
