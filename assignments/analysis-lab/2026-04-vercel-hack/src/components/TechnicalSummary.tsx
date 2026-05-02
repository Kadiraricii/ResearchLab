import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

export function TechnicalSummary() {
  const [vercelJsonContent, setVercelJsonContent] = useState("");
  const [nextConfigContent, setNextConfigContent] = useState("");
  const [vercelParsed, setVercelParsed] = useState<any>(null);
  const [nextParsed, setNextParsed] = useState<any>(null);
  const [defaults, setDefaults] = useState<any>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    invoke("get_platform_defaults").then((res) => setDefaults(res)).catch(console.error);
  }, []);

  const handleVercelParse = async () => {
    try {
      setError(null);
      const parsed = await invoke("parse_vercel_config", { content: vercelJsonContent });
      setVercelParsed(parsed);
    } catch (e: any) {
      setError(e.toString());
      setVercelParsed(null);
    }
  };

  const handleNextParse = async () => {
    try {
      setError(null);
      const parsed = await invoke("parse_next_config", { content: nextConfigContent });
      setNextParsed(parsed);
    } catch (e: any) {
      setError(e.toString());
      setNextParsed(null);
    }
  };

  return (
    <div style={{ padding: "20px", fontFamily: "system-ui" }}>
      <h2>Teknik Özet & Platform Analizi</h2>
      
      {defaults && (
        <div style={{ background: "#f0f0f0", padding: "10px", borderRadius: "5px", marginBottom: "20px" }}>
          <h4>Vercel Varsayılan Güvenlik Ayarları</h4>
          <ul>
            <li>HSTS Varsayılan Etkin mi? {defaults.hsts_enabled_by_default ? "Evet" : "Hayır"}</li>
            <li>CSP Varsayılan Etkin mi? {defaults.csp_enabled_by_default ? "Evet" : "Hayır"}</li>
            <li>Ortam Değişkeni İfşa Öneki: {defaults.public_env_prefix}</li>
          </ul>
        </div>
      )}

      {error && <div style={{ color: "red", marginBottom: "10px" }}>Hata: {error}</div>}

      <div style={{ display: "flex", gap: "20px" }}>
        <div style={{ flex: 1 }}>
          <h3>vercel.json Analizi</h3>
          <textarea 
            rows={10} 
            style={{ width: "100%", padding: "10px", fontFamily: "monospace" }}
            placeholder='{"headers": [], "redirects": []}'
            value={vercelJsonContent}
            onChange={(e) => setVercelJsonContent(e.target.value)}
          />
          <button onClick={handleVercelParse} style={{ marginTop: "10px", padding: "8px 16px" }}>Analiz Et</button>
          
          {vercelParsed && (
            <div style={{ marginTop: "10px", background: "#e8f5e9", padding: "10px", borderRadius: "5px" }}>
              <h4>Bulgular:</h4>
              <p>Yönlendirme (Redirects) Sayısı: {vercelParsed.redirects.length}</p>
              <p>Yeniden Yazma (Rewrites) Sayısı: {vercelParsed.rewrites.length}</p>
              <p>Özel Başlık (Headers) Kural Sayısı: {vercelParsed.headers.length}</p>
            </div>
          )}
        </div>

        <div style={{ flex: 1 }}>
          <h3>next.config.js Analizi</h3>
          <textarea 
            rows={10} 
            style={{ width: "100%", padding: "10px", fontFamily: "monospace" }}
            placeholder='module.exports = { poweredByHeader: false }'
            value={nextConfigContent}
            onChange={(e) => setNextConfigContent(e.target.value)}
          />
          <button onClick={handleNextParse} style={{ marginTop: "10px", padding: "8px 16px" }}>Analiz Et</button>
          
          {nextParsed && (
            <div style={{ marginTop: "10px", background: "#e3f2fd", padding: "10px", borderRadius: "5px" }}>
              <h4>Bulgular:</h4>
              <p>Powered-By Başlığı Kapatılmış mı? {nextParsed.powered_by_header === false ? "Evet ✅" : "Hayır/Belirsiz ⚠️"}</p>
              <p>Headers Fonksiyonu Var mı? {nextParsed.has_headers_function ? "Evet" : "Hayır"}</p>
              <p>Remote Patterns Tanımlı mı? {nextParsed.remote_patterns_found ? "Evet" : "Hayır"}</p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
