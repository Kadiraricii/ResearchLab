import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export function RemediationGuide() {
  const [vercelJsonContent, setVercelJsonContent] = useState("");
  const [nextConfigContent, setNextConfigContent] = useState("");
  const [envContent, setEnvContent] = useState("");
  const [report, setReport] = useState<any>(null);
  const [error, setError] = useState<string | null>(null);

  const handleFix = async () => {
    try {
      setError(null);
      const res = await invoke("generate_remediation_report", { 
        vercelJson: vercelJsonContent, 
        nextConfig: nextConfigContent,
        envContent: envContent
      });
      setReport(res);
    } catch (e: any) {
      setError(e.toString());
      setReport(null);
    }
  };

  return (
    <div style={{ padding: "20px", fontFamily: "system-ui" }}>
      <h2>Sertleştirme ve Düzeltme Rehberi</h2>
      <p>Mevcut zafiyetli yapılandırmanızı girin, size güvenli şablonları ve skorunuzu üretelim.</p>
      
      {error && <div style={{ color: "red", marginBottom: "10px" }}>Hata: {error}</div>}

      <div style={{ display: "flex", gap: "20px", marginBottom: "20px" }}>
        <div style={{ flex: 1 }}>
          <label><strong>Zafiyetli vercel.json</strong></label>
          <textarea 
            rows={5} 
            style={{ width: "100%", padding: "5px", fontFamily: "monospace" }}
            value={vercelJsonContent}
            onChange={(e) => setVercelJsonContent(e.target.value)}
          />
        </div>
        <div style={{ flex: 1 }}>
          <label><strong>Zafiyetli next.config.js</strong></label>
          <textarea 
            rows={5} 
            style={{ width: "100%", padding: "5px", fontFamily: "monospace" }}
            value={nextConfigContent}
            onChange={(e) => setNextConfigContent(e.target.value)}
          />
        </div>
        <div style={{ flex: 1 }}>
          <label><strong>Zafiyetli .env</strong></label>
          <textarea 
            rows={5} 
            style={{ width: "100%", padding: "5px", fontFamily: "monospace" }}
            placeholder="NEXT_PUBLIC_API_KEY=12345"
            value={envContent}
            onChange={(e) => setEnvContent(e.target.value)}
          />
        </div>
      </div>

      <button onClick={handleFix} style={{ padding: "10px 20px", fontSize: "16px", cursor: "pointer", backgroundColor: "#28a745", color: "white", border: "none", borderRadius: "5px" }}>
        Otomatik Düzeltme Önerisi Üret
      </button>

      {report && (
        <div style={{ marginTop: "30px" }}>
          <div style={{ fontSize: "24px", fontWeight: "bold", color: report.hardening_score > 7 ? "green" : "orange", marginBottom: "20px" }}>
            Sertleştirme Skoru: {report.hardening_score.toFixed(1)} / 10.0
          </div>

          <div style={{ marginBottom: "30px", background: "#f8f9fa", padding: "20px", borderRadius: "5px", borderLeft: "5px solid #0070f3" }}>
            <h3>Önerilen Aksiyonlar (Dinamik)</h3>
            <ul style={{ lineHeight: "1.6" }}>
              {report.recommendations.map((rec: string, idx: number) => (
                <li key={idx}><strong>{rec}</strong></li>
              ))}
            </ul>
          </div>

          <div style={{ display: "flex", gap: "20px" }}>
            <div style={{ flex: 1, background: "#1e1e1e", color: "#d4d4d4", padding: "15px", borderRadius: "5px" }}>
              <h3 style={{ color: "#fff", marginTop: 0 }}>[AFTER] Güvenli vercel.json</h3>
              <pre style={{ whiteSpace: "pre-wrap", wordWrap: "break-word", fontSize: "13px" }}>
                {report.secure_vercel_json}
              </pre>
            </div>

            <div style={{ flex: 1, background: "#1e1e1e", color: "#d4d4d4", padding: "15px", borderRadius: "5px" }}>
              <h3 style={{ color: "#fff", marginTop: 0 }}>[AFTER] Güvenli next.config.js</h3>
              <pre style={{ whiteSpace: "pre-wrap", wordWrap: "break-word", fontSize: "13px" }}>
                {report.secure_next_config}
              </pre>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
