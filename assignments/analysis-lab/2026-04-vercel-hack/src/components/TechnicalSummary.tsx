import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

// ─── Types ────────────────────────────────────────────────────────────────────

interface PlatformDefaults {
  hsts_enabled_by_default: boolean;
  csp_enabled_by_default: boolean;
  public_env_prefix: string;
}

interface VercelParsed {
  redirects: unknown[];
  rewrites: unknown[];
  headers: unknown[];
}

interface NextParsed {
  powered_by_header: boolean | null;
  has_headers_function: boolean;
  remote_patterns_found: boolean;
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

function StatusVal({
  ok,
  trueLabel = "Etkin ✓",
  falseLabel = "Devre Dışı ✗",
}: {
  ok: boolean;
  trueLabel?: string;
  falseLabel?: string;
}) {
  return (
    <span style={{ color: ok ? "var(--low)" : "var(--critical)", fontWeight: 700 }}>
      {ok ? trueLabel : falseLabel}
    </span>
  );
}

// ─── Main component ───────────────────────────────────────────────────────────

export function TechnicalSummary() {
  const [vercelJson, setVercelJson]     = useState("");
  const [nextConfig, setNextConfig]     = useState("");
  const [vercelParsed, setVercelParsed] = useState<VercelParsed | null>(null);
  const [nextParsed, setNextParsed]     = useState<NextParsed | null>(null);
  const [defaults, setDefaults]         = useState<PlatformDefaults | null>(null);
  const [error, setError]               = useState<string | null>(null);
  const [loading, setLoading]           = useState<"vercel" | "next" | null>(null);

  // Load platform defaults on mount
  useEffect(() => {
    invoke<PlatformDefaults>("get_platform_defaults")
      .then(setDefaults)
      .catch((e: unknown) =>
        console.error("get_platform_defaults failed:", e)
      );
  }, []);

  const handleVercelParse = async () => {
    setLoading("vercel");
    setError(null);
    try {
      const res = await invoke<VercelParsed>("parse_vercel_config", {
        content: vercelJson,
      });
      setVercelParsed(res);
    } catch (e: unknown) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setLoading(null);
    }
  };

  const handleNextParse = async () => {
    setLoading("next");
    setError(null);
    try {
      const res = await invoke<NextParsed>("parse_next_config", {
        content: nextConfig,
      });
      setNextParsed(res);
    } catch (e: unknown) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setLoading(null);
    }
  };

  return (
    <div className="page">
      {/* ── Page header ── */}
      <div className="ph">
        <h1 className="ph-title">Platform Analizi</h1>
        <p className="ph-desc">
          Vercel güvenlik varsayılanları ve yapılandırma ayrıştırma.
        </p>
      </div>

      {/* ── Platform defaults ── */}
      {defaults && (
        <div className="card" style={{ marginBottom: 12 }}>
          <p className="card-label">Vercel Platform Varsayılanları</p>
          <div className="defs-grid">
            <div className="def-item">
              <div className="def-lbl">HSTS Varsayılan</div>
              <div className="def-val">
                <StatusVal ok={defaults.hsts_enabled_by_default} />
              </div>
            </div>
            <div className="def-item">
              <div className="def-lbl">CSP Varsayılan</div>
              <div className="def-val">
                <StatusVal
                  ok={defaults.csp_enabled_by_default}
                  falseLabel="Manuel Gerekli"
                />
              </div>
            </div>
            <div className="def-item">
              <div className="def-lbl">Public Env Prefix</div>
              <div
                className="def-val"
                style={{ fontFamily: "var(--mono)", fontSize: "var(--t-sm)", color: "var(--accent)" }}
              >
                {defaults.public_env_prefix}
              </div>
            </div>
          </div>
        </div>
      )}

      {/* ── Error ── */}
      {error && <div className="err-bar">{error}</div>}

      {/* ── Parser cards ── */}
      <div className="two-col">
        {/* vercel.json */}
        <div className="card">
          <p className="card-label">
            <span className="file-tag">vercel.json</span>&nbsp; Ayrıştırıcı
          </p>
          <div className="input-group" style={{ marginBottom: 12 }}>
            <textarea
              className="code-ta"
              rows={8}
              placeholder={'{\n  "headers": [],\n  "redirects": [],\n  "rewrites": []\n}'}
              value={vercelJson}
              onChange={(e) => setVercelJson(e.target.value)}
            />
          </div>
          <button
            className="btn btn-primary"
            onClick={handleVercelParse}
            disabled={loading === "vercel"}
          >
            {loading === "vercel" ? <span className="spin" /> : null}
            {loading === "vercel" ? "Ayrıştırılıyor…" : "Ayrıştır"}
          </button>

          {vercelParsed && (
            <div className="parsed-grid">
              <div className="parsed-item">
                <div className="parsed-lbl">Redirects</div>
                <div className="parsed-val">{vercelParsed.redirects.length}</div>
              </div>
              <div className="parsed-item">
                <div className="parsed-lbl">Rewrites</div>
                <div className="parsed-val">{vercelParsed.rewrites.length}</div>
              </div>
              <div className="parsed-item">
                <div className="parsed-lbl">Header Kuralı</div>
                <div className="parsed-val">{vercelParsed.headers.length}</div>
              </div>
            </div>
          )}
        </div>

        {/* next.config.js */}
        <div className="card">
          <p className="card-label">
            <span className="file-tag">next.config.js</span>&nbsp; Ayrıştırıcı
          </p>
          <div className="input-group" style={{ marginBottom: 12 }}>
            <textarea
              className="code-ta"
              rows={8}
              placeholder={"module.exports = {\n  poweredByHeader: false\n}"}
              value={nextConfig}
              onChange={(e) => setNextConfig(e.target.value)}
            />
          </div>
          <button
            className="btn btn-primary"
            onClick={handleNextParse}
            disabled={loading === "next"}
          >
            {loading === "next" ? <span className="spin" /> : null}
            {loading === "next" ? "Ayrıştırılıyor…" : "Ayrıştır"}
          </button>

          {nextParsed && (
            <div className="parsed-grid">
              <div className="parsed-item">
                <div className="parsed-lbl">X-Powered-By</div>
                <div
                  className="parsed-val"
                  style={{
                    fontSize: "var(--t-sm)",
                    color:
                      nextParsed.powered_by_header === false
                        ? "var(--low)"
                        : "var(--critical)",
                  }}
                >
                  {nextParsed.powered_by_header === false ? "Kapalı ✓" : "Açık ✗"}
                </div>
              </div>
              <div className="parsed-item">
                <div className="parsed-lbl">Headers Fn.</div>
                <div className="parsed-val" style={{ fontSize: "var(--t-sm)" }}>
                  {nextParsed.has_headers_function ? "Var" : "Yok"}
                </div>
              </div>
              <div className="parsed-item">
                <div className="parsed-lbl">Remote Patterns</div>
                <div className="parsed-val" style={{ fontSize: "var(--t-sm)" }}>
                  {nextParsed.remote_patterns_found ? "Tanımlı" : "Tanımsız"}
                </div>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
