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
          <span className="hard-score" style={{ color }}>{score.toFixed(1)}</span>
          <span style={{ fontSize: "var(--t-sm)", color: "var(--text-muted)", marginLeft: 4 }}>/ 10</span>
          <span style={{ marginLeft: 10, fontSize: "var(--t-sm)", fontWeight: 700, color }}>{label}</span>
        </span>
      </div>
      <div className="hard-bar-wrap">
        <div className="hard-bar-fill" style={{ width: `${pct}%`, background: color }} />
      </div>
    </>
  );
}

function ChecklistProgress({ done, total }: { done: number; total: number }) {
  const pct = total === 0 ? 0 : Math.round((done / total) * 100);
  const color = pct === 100 ? "var(--low)" : pct >= 50 ? "var(--medium)" : "var(--accent)";
  return (
    <div style={{ marginBottom: 16 }}>
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "baseline", marginBottom: 6 }}>
        <span style={{ fontSize: "var(--t-xs)", fontWeight: 700, color: "var(--text-subtle)", letterSpacing: "0.06em", textTransform: "uppercase" }}>
          Checklist İlerlemesi
        </span>
        <span style={{ fontSize: "var(--t-sm)", fontWeight: 700, color }}>
          {done}/{total} ({pct}%)
        </span>
      </div>
      <div className="hard-bar-wrap">
        <div
          className="hard-bar-fill"
          style={{ width: `${pct}%`, background: color, transition: "width 350ms var(--ease)" }}
        />
      </div>
    </div>
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
  const [done, setDone]             = useState<Set<number>>(new Set());

  const handleFix = async () => {
    setLoading(true);
    setError(null);
    setDone(new Set());
    try {
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

  const toggleDone = (i: number) =>
    setDone((prev) => {
      const next = new Set(prev);
      next.has(i) ? next.delete(i) : next.add(i);
      return next;
    });

  const total = report?.recommendations.length ?? 0;
  const doneCount = done.size;

  return (
    <div className="page">
      <div className="ph">
        <h1 className="ph-title">Sertleştirme Rehberi</h1>
        <p className="ph-desc">
          Mevcut yapılandırmanızı girin; güvenli şablonlar ve interaktif checklist üretelim.
        </p>
      </div>

      {/* ── Input card ── */}
      <div className="card">
        <p className="card-label">Mevcut Yapılandırma</p>
        <div className="input-grid">
          <div className="input-group">
            <label className="input-label"><span className="file-tag">vercel.json</span></label>
            <textarea
              className="code-ta"
              placeholder={'{\n  "headers": []\n}'}
              value={vercelJson}
              onChange={(e) => setVercelJson(e.target.value)}
            />
          </div>
          <div className="input-group">
            <label className="input-label"><span className="file-tag">next.config.js</span></label>
            <textarea
              className="code-ta"
              placeholder={"module.exports = {}"}
              value={nextConfig}
              onChange={(e) => setNextConfig(e.target.value)}
            />
          </div>
          <div className="input-group">
            <label className="input-label"><span className="file-tag">.env.local</span></label>
            <textarea
              className="code-ta"
              placeholder={"NEXT_PUBLIC_API_KEY=..."}
              value={envContent}
              onChange={(e) => setEnvContent(e.target.value)}
            />
          </div>
        </div>

        <button className="btn btn-green" onClick={handleFix} disabled={loading}>
          {loading ? (
            <span className="spin" />
          ) : (
            <svg width="13" height="13" viewBox="0 0 16 16" fill="none">
              <path d="M3 8.5L6.5 12L13 4" stroke="currentColor" strokeWidth="1.6" strokeLinecap="round" strokeLinejoin="round" />
            </svg>
          )}
          {loading ? "Üretiliyor…" : "Güvenli Yapılandırma Üret"}
        </button>
      </div>

      {error && <div className="err-bar">{error}</div>}

      {/* ── Report ── */}
      {report && (
        <>
          <div className="div" />

          {/* Hardening score */}
          <div className="card" style={{ marginBottom: 12 }}>
            <HardeningScore score={report.hardening_score} />
          </div>

          {/* Interactive checklist */}
          {report.recommendations.length > 0 && (
            <div className="card" style={{ marginBottom: 12 }}>
              <p className="card-label">Aksiyonlar — İnteraktif Checklist</p>
              <ChecklistProgress done={doneCount} total={total} />
              <div className="checklist">
                {report.recommendations.map((rec, i) => {
                  const isDone = done.has(i);
                  return (
                    <button
                      key={i}
                      className={`check-item ${isDone ? "check-done" : ""}`}
                      onClick={() => toggleDone(i)}
                      type="button"
                    >
                      <span className="check-box">
                        {isDone && (
                          <svg width="10" height="10" viewBox="0 0 12 12" fill="none">
                            <path d="M2 6L5 9L10 3" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round" />
                          </svg>
                        )}
                      </span>
                      <span className="check-text">{rec}</span>
                    </button>
                  );
                })}
              </div>
              {doneCount === total && total > 0 && (
                <div style={{
                  marginTop: 14,
                  padding: "10px 14px",
                  background: "var(--low-dim)",
                  border: "1px solid oklch(70% 0.19 145 / 0.25)",
                  borderRadius: 5,
                  fontSize: "var(--t-sm)",
                  color: "var(--low)",
                  fontWeight: 700,
                }}>
                  ✓ Tüm aksiyonlar tamamlandı. Yapılandırmanız sertleştirildi.
                </div>
              )}
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
