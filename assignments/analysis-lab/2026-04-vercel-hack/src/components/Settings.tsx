import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

interface ConfiguredStatus {
  configured: boolean;
}

export function Settings() {
  const [token, setToken]           = useState("");
  const [masked, setMasked]         = useState(true);
  const [configured, setConfigured] = useState<boolean | null>(null);
  const [loading, setLoading]       = useState(false);
  const [saving, setSaving]         = useState(false);
  const [success, setSuccess]       = useState(false);
  const [error, setError]           = useState<string | null>(null);

  // Check token status on mount
  useEffect(() => {
    setLoading(true);
    invoke<ConfiguredStatus>("is_configured")
      .then((s) => setConfigured(s.configured))
      .catch(() => setConfigured(false))
      .finally(() => setLoading(false));
  }, []);

  const handleSave = async () => {
    if (!token.trim()) return;
    setSaving(true);
    setError(null);
    setSuccess(false);
    try {
      await invoke("set_vercel_token", { token: token.trim() });
      setConfigured(true);
      setSuccess(true);
      setToken("");
      // Clear success after 3s
      setTimeout(() => setSuccess(false), 3000);
    } catch (e: unknown) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="page">
      <div className="ph">
        <h1 className="ph-title">Ayarlar</h1>
        <p className="ph-desc">API token yapılandırması ve uygulama ayarları.</p>
      </div>

      {/* ── Token status ── */}
      <div className="card" style={{ marginBottom: 12 }}>
        <p className="card-label">Token Durumu</p>
        {loading ? (
          <span className="spin" style={{ borderColor: "var(--border)", borderTopColor: "var(--accent)" }} />
        ) : (
          <div style={{ display: "flex", alignItems: "center", gap: 12 }}>
            <span className={`token-status ${configured ? "configured" : "not-configured"}`}>
              {configured ? "✓ Token Yapılandırılmış" : "✗ Token Yapılandırılmamış"}
            </span>
            {configured && (
              <span style={{ fontSize: "var(--t-xs)", color: "var(--text-subtle)" }}>
                Vercel API'ye erişim hazır. Tarayıcı ve tam tarama komutları kullanılabilir.
              </span>
            )}
          </div>
        )}
      </div>

      {/* ── Token input ── */}
      <div className="card">
        <p className="card-label">Vercel API Token</p>

        <div className="info-box">
          <span className="info-box-icon">
            <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor">
              <path d="M8 0a8 8 0 1 0 0 16A8 8 0 0 0 8 0zm.75 12h-1.5V7h1.5v5zm0-6.5h-1.5v-1.5h1.5v1.5z" />
            </svg>
          </span>
          <span>
            Token yalnızca bu oturum süresince bellekte tutulur, diske yazılmaz.
            Vercel Dashboard → Account Settings → Tokens bölümünden oluşturabilirsiniz.
          </span>
        </div>

        <div className="settings-field">
          <label className="settings-label">API Token</label>
          <p className="settings-desc">
            <span style={{ fontFamily: "var(--mono)", color: "var(--accent)" }}>vercel_</span> ile
            başlayan token değerini girin.
          </p>
          <div style={{ position: "relative" }}>
            <input
              type={masked ? "password" : "text"}
              className="token-input"
              placeholder="vercel_xxxxxxxxxxxxxxxxxxxx"
              value={token}
              onChange={(e) => setToken(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && handleSave()}
              spellCheck={false}
              autoComplete="off"
            />
            <button
              onClick={() => setMasked((m) => !m)}
              style={{
                position: "absolute",
                right: 10,
                top: "50%",
                transform: "translateY(-50%)",
                background: "none",
                border: "none",
                cursor: "pointer",
                color: "var(--text-subtle)",
                padding: 4,
                fontSize: "var(--t-xs)",
                fontFamily: "var(--sans)",
              }}
              type="button"
            >
              {masked ? "Göster" : "Gizle"}
            </button>
          </div>
        </div>

        {error && <div className="err-bar" style={{ marginBottom: 12 }}>{error}</div>}

        {success && (
          <div style={{
            display: "flex", alignItems: "center", gap: 8,
            padding: "10px 14px", background: "var(--low-dim)",
            border: "1px solid oklch(70% 0.19 145 / 0.3)",
            borderRadius: 5, marginBottom: 12,
            fontSize: "var(--t-sm)", color: "var(--low)", fontWeight: 600,
          }}>
            ✓ Token başarıyla kaydedildi.
          </div>
        )}

        <button
          className="btn btn-primary"
          onClick={handleSave}
          disabled={saving || !token.trim()}
        >
          {saving ? <span className="spin" /> : (
            <svg width="13" height="13" viewBox="0 0 16 16" fill="none">
              <path d="M2 8.5L5.5 12L14 4" stroke="currentColor" strokeWidth="1.6" strokeLinecap="round" strokeLinejoin="round" />
            </svg>
          )}
          {saving ? "Kaydediliyor…" : "Token Kaydet"}
        </button>
      </div>

      {/* ── About ── */}
      <div className="card" style={{ marginTop: 12 }}>
        <p className="card-label">Uygulama Hakkında</p>
        <div style={{ display: "grid", gridTemplateColumns: "repeat(3, 1fr)", gap: 8 }}>
          {[
            { k: "Sürüm",   v: "0.1.0" },
            { k: "Çerçeve", v: "Tauri 2 + React 19" },
            { k: "Backend", v: "Rust + rayon + tokio" },
            { k: "UUID",    v: "434fce1d" },
            { k: "Dal",     v: "2026-04-vercel-hack" },
            { k: "Analiz",  v: "12 modül / paralel" },
          ].map(({ k, v }) => (
            <div key={k} className="parsed-item">
              <div className="parsed-lbl">{k}</div>
              <div style={{ fontSize: "var(--t-xs)", fontFamily: "var(--mono)", color: "var(--text-muted)", fontWeight: 600 }}>{v}</div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
