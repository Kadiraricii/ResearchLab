import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface Vulnerability {
  id: string;
  title: string;
  description: string;
  risk_level: string;
  affected_component: string;
  remediation: string;
}

interface AnalysisReport {
  vulnerabilities: Vulnerability[];
  score: number;
}

interface ReportsProps {
  lastReport: AnalysisReport | null;
}

// ─── HTML export template ─────────────────────────────────────────────────────

function buildHtml(report: AnalysisReport): string {
  const riskColor: Record<string, string> = {
    Critical: "#ef4444", High: "#f97316", Medium: "#eab308", Low: "#22c55e", Info: "#3b82f6",
  };

  const rows = report.vulnerabilities
    .map(
      (v) =>
        `<tr>
          <td style="font-family:monospace;white-space:nowrap">${v.id}</td>
          <td>${v.title}</td>
          <td><span style="color:${riskColor[v.risk_level] ?? "#888"};font-weight:700">${v.risk_level}</span></td>
          <td>${v.affected_component}</td>
          <td>${v.description}</td>
          <td>${v.remediation}</td>
        </tr>`
    )
    .join("\n");

  return `<!DOCTYPE html>
<html lang="tr">
<head>
<meta charset="UTF-8">
<title>Vercel Security Report</title>
<style>
  body { font-family: system-ui, sans-serif; background: #0f0f0f; color: #eee; padding: 40px; }
  h1 { font-size: 1.5rem; font-weight: 800; letter-spacing: -0.03em; margin-bottom: 4px; }
  .score { font-size: 3rem; font-weight: 900; color: ${report.score >= 7 ? "#ef4444" : report.score >= 4 ? "#f97316" : "#22c55e"}; }
  table { width: 100%; border-collapse: collapse; margin-top: 32px; }
  th { text-align: left; padding: 8px 12px; border-bottom: 1px solid #333; font-size: 11px; text-transform: uppercase; letter-spacing: .06em; color: #888; }
  td { padding: 10px 12px; border-bottom: 1px solid #1e1e1e; font-size: 13px; vertical-align: top; }
  tr:hover td { background: #161616; }
</style>
</head>
<body>
<h1>Vercel Security Analyzer — Rapor</h1>
<p style="color:#888;font-size:13px">Oluşturulma: ${new Date().toLocaleString("tr-TR")}</p>
<div class="score">${report.score.toFixed(1)}<span style="font-size:1rem;color:#888;font-weight:500"> / 10</span></div>
<p style="color:#888;font-size:13px">${report.vulnerabilities.length} zafiyet tespit edildi</p>
<table>
  <thead><tr><th>ID</th><th>Başlık</th><th>Risk</th><th>Bileşen</th><th>Açıklama</th><th>Çözüm</th></tr></thead>
  <tbody>${rows}</tbody>
</table>
</body>
</html>`;
}

// ─── Download helper ──────────────────────────────────────────────────────────

function download(content: string, filename: string, mime: string) {
  const blob = new Blob([content], { type: mime });
  const url  = URL.createObjectURL(blob);
  const a    = document.createElement("a");
  a.href     = url;
  a.download = filename;
  a.click();
  URL.revokeObjectURL(url);
}

// ─── Main component ───────────────────────────────────────────────────────────

export function Reports({ lastReport }: ReportsProps) {
  const [jsonInput, setJsonInput] = useState("");
  const [parsed, setParsed]       = useState<AnalysisReport | null>(null);
  const [parseError, setParseError] = useState<string | null>(null);
  const [exporting, setExporting]   = useState<"json" | "html" | null>(null);

  // Use lastReport prop if available, otherwise use manually pasted JSON
  const activeReport = lastReport ?? parsed;

  const handleParse = () => {
    setParseError(null);
    try {
      const r = JSON.parse(jsonInput) as AnalysisReport;
      if (!Array.isArray(r.vulnerabilities) || typeof r.score !== "number") {
        throw new Error("Geçersiz rapor formatı. 'vulnerabilities' dizisi ve 'score' alanı gerekli.");
      }
      setParsed(r);
    } catch (e: unknown) {
      setParseError(e instanceof Error ? e.message : "JSON ayrıştırma hatası");
    }
  };

  const handleExportJson = async () => {
    if (!activeReport) return;
    setExporting("json");
    try {
      const json = await invoke<string>("export_report", { report: activeReport });
      download(json, `vercel-security-report-${Date.now()}.json`, "application/json");
    } catch (e: unknown) {
      console.error(e);
    } finally {
      setExporting(null);
    }
  };

  const handleExportHtml = () => {
    if (!activeReport) return;
    setExporting("html");
    try {
      const html = buildHtml(activeReport);
      download(html, `vercel-security-report-${Date.now()}.html`, "text/html");
    } finally {
      setExporting(null);
    }
  };

  return (
    <div className="page">
      <div className="ph">
        <h1 className="ph-title">Raporlar</h1>
        <p className="ph-desc">Tarama sonuçlarını görüntüleyin ve JSON / HTML olarak dışa aktarın.</p>
      </div>

      {/* ── Source ── */}
      {lastReport ? (
        <div className="info-box">
          <span className="info-box-icon">
            <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor">
              <path d="M8 0a8 8 0 1 0 0 16A8 8 0 0 0 8 0zm.75 12h-1.5V7h1.5v5zm0-6.5h-1.5v-1.5h1.5v1.5z"/>
            </svg>
          </span>
          <span>
            Bu oturumda gerçekleştirilen son tarama gösteriliyor.{" "}
            <strong style={{ color: "var(--text)" }}>{lastReport.vulnerabilities.length} zafiyet</strong>,
            skor <strong style={{ color: "var(--text)" }}>{lastReport.score.toFixed(1)}</strong>.
          </span>
        </div>
      ) : (
        <div className="card" style={{ marginBottom: 12 }}>
          <p className="card-label">Rapor JSON Yapıştır</p>
          <p style={{ fontSize: "var(--t-xs)", color: "var(--text-subtle)", marginBottom: 10 }}>
            Tarayıcı sayfasında tarama yapın; rapor otomatik burada görünür. Ya da aşağıya JSON yapıştırın.
          </p>
          <textarea
            className="code-ta"
            style={{ minHeight: 120 }}
            placeholder={'{"vulnerabilities": [], "score": 0.0}'}
            value={jsonInput}
            onChange={(e) => setJsonInput(e.target.value)}
          />
          {parseError && <div className="err-bar" style={{ marginTop: 8 }}>{parseError}</div>}
          <button className="btn btn-primary" style={{ marginTop: 10 }} onClick={handleParse} disabled={!jsonInput.trim()}>
            JSON Yükle
          </button>
        </div>
      )}

      {/* ── Report view ── */}
      {activeReport && (
        <>
          {/* Export buttons */}
          <div className="export-btns">
            <button className="btn btn-primary" onClick={handleExportJson} disabled={exporting === "json"}>
              {exporting === "json" ? <span className="spin" /> : (
                <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
                  <path d="M8 12L3 7h3V1h4v6h3L8 12zM1 14h14v1.5H1V14z"/>
                </svg>
              )}
              JSON İndir
            </button>
            <button className="btn btn-outline" onClick={handleExportHtml} disabled={exporting === "html"}>
              {exporting === "html" ? <span className="spin" /> : null}
              HTML İndir
            </button>
          </div>

          <div className="div" />

          {/* Score summary */}
          <div style={{ display: "flex", alignItems: "baseline", gap: 12, marginBottom: 20 }}>
            <span style={{
              fontSize: "3rem", fontWeight: 900, letterSpacing: "-0.05em", lineHeight: 1,
              color: activeReport.score >= 7 ? "var(--critical)" : activeReport.score >= 4 ? "var(--high)" : "var(--low)",
            }}>
              {activeReport.score.toFixed(1)}
            </span>
            <span style={{ fontSize: "var(--t-sm)", color: "var(--text-muted)" }}>
              / 10 — {activeReport.vulnerabilities.length} zafiyet
            </span>
          </div>

          {/* Findings table */}
          {activeReport.vulnerabilities.length === 0 ? (
            <div className="empty">
              <div className="empty-icon">✓</div>
              <h3>Zafiyet Bulunamadı</h3>
              <p>Bu raporda bilinen güvenlik açığı tespit edilmedi.</p>
            </div>
          ) : (
            <div className="card" style={{ padding: 0, overflow: "hidden" }}>
              <table style={{ width: "100%", borderCollapse: "collapse" }}>
                <thead>
                  <tr style={{ borderBottom: "1px solid var(--border)" }}>
                    {["ID", "Başlık", "Risk", "Bileşen", "Açıklama"].map((h) => (
                      <th key={h} style={{
                        textAlign: "left", padding: "10px 14px",
                        fontSize: "var(--t-xs)", fontWeight: 700,
                        letterSpacing: "0.06em", textTransform: "uppercase",
                        color: "var(--text-subtle)",
                        background: "var(--surface)",
                      }}>
                        {h}
                      </th>
                    ))}
                  </tr>
                </thead>
                <tbody>
                  {[...activeReport.vulnerabilities]
                    .sort((a, b) => {
                      const ord: Record<string, number> = { Critical: 0, High: 1, Medium: 2, Low: 3, Info: 4 };
                      return (ord[a.risk_level] ?? 9) - (ord[b.risk_level] ?? 9);
                    })
                    .map((v, i) => (
                      <tr
                        key={`${v.id}-${i}`}
                        style={{ borderBottom: "1px solid var(--border-sub)" }}
                      >
                        <td style={{ padding: "10px 14px", fontFamily: "var(--mono)", fontSize: "var(--t-xs)", color: "var(--text-subtle)", whiteSpace: "nowrap" }}>
                          {v.id}
                        </td>
                        <td style={{ padding: "10px 14px", fontSize: "var(--t-sm)", fontWeight: 600, color: "var(--text)" }}>
                          {v.title}
                        </td>
                        <td style={{ padding: "10px 14px", whiteSpace: "nowrap" }}>
                          <span className={`badge badge-${v.risk_level.toLowerCase()}`}>{v.risk_level}</span>
                        </td>
                        <td style={{ padding: "10px 14px", fontSize: "var(--t-xs)", color: "var(--text-muted)", whiteSpace: "nowrap" }}>
                          {v.affected_component}
                        </td>
                        <td style={{ padding: "10px 14px", fontSize: "var(--t-xs)", color: "var(--text-subtle)", lineHeight: 1.5, maxWidth: 300 }}>
                          {v.description}
                        </td>
                      </tr>
                    ))}
                </tbody>
              </table>
            </div>
          )}
        </>
      )}
    </div>
  );
}
