import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export function AttackVectors() {
  const [vercelJsonContent, setVercelJsonContent] = useState("");
  const [nextConfigContent, setNextConfigContent] = useState("");
  const [envContent, setEnvContent] = useState("");
  const [report, setReport] = useState<any>(null);
  const [error, setError] = useState<string | null>(null);

  const handleScan = async () => {
    try {
      setError(null);
      const res = await invoke("run_analysis", { 
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

  const getRiskColor = (level: string) => {
    switch (level) {
      case "Critical": return "#ffebee"; // Kırmızı
      case "High": return "#fff3e0"; // Turuncu
      case "Medium": return "#fffde7"; // Sarı
      case "Low": return "#e8f5e9"; // Yeşil
      default: return "#f5f5f5";
    }
  };

  return (
    <div style={{ padding: "20px", fontFamily: "system-ui" }}>
      <h2>Zafiyet Analiz Motoru (Vercel Scanner)</h2>
      <p>Proje dosyalarını girerek bilinen 12 saldırı vektörüne karşı tarama yapın.</p>
      
      {error && <div style={{ color: "red", marginBottom: "10px" }}>Hata: {error}</div>}

      <div style={{ display: "flex", gap: "20px", marginBottom: "20px" }}>
        <div style={{ flex: 1 }}>
          <label><strong>vercel.json</strong></label>
          <textarea 
            rows={5} 
            style={{ width: "100%", padding: "5px", fontFamily: "monospace" }}
            value={vercelJsonContent}
            onChange={(e) => setVercelJsonContent(e.target.value)}
          />
        </div>
        <div style={{ flex: 1 }}>
          <label><strong>next.config.js</strong></label>
          <textarea 
            rows={5} 
            style={{ width: "100%", padding: "5px", fontFamily: "monospace" }}
            value={nextConfigContent}
            onChange={(e) => setNextConfigContent(e.target.value)}
          />
        </div>
        <div style={{ flex: 1 }}>
          <label><strong>.env (.env.local)</strong></label>
          <textarea 
            rows={5} 
            style={{ width: "100%", padding: "5px", fontFamily: "monospace" }}
            placeholder="NEXT_PUBLIC_API_KEY=12345"
            value={envContent}
            onChange={(e) => setEnvContent(e.target.value)}
          />
        </div>
      </div>

      <button onClick={handleScan} style={{ padding: "10px 20px", fontSize: "16px", cursor: "pointer", backgroundColor: "#0070f3", color: "white", border: "none", borderRadius: "5px" }}>
        Sistemi Tara
      </button>

      {report && (
        <div style={{ marginTop: "30px" }}>
          <h3>Tarama Raporu</h3>
          <div style={{ fontSize: "24px", fontWeight: "bold", color: report.score > 7 ? "red" : (report.score > 4 ? "orange" : "green") }}>
            Güvenlik Riski Skoru: {report.score.toFixed(1)} / 10.0
          </div>
          
          {report.vulnerabilities.length === 0 ? (
            <div style={{ marginTop: "20px", padding: "15px", background: "#e8f5e9", borderLeft: "5px solid #4caf50" }}>
              Tebrikler! Belirtilen konfigürasyonlarda bilinen bir zafiyet bulunamadı.
            </div>
          ) : (
            <div style={{ marginTop: "20px", display: "grid", gap: "15px" }}>
              {report.vulnerabilities.map((v: any, index: number) => (
                <div key={index} style={{ padding: "15px", background: getRiskColor(v.risk_level), border: "1px solid #ddd", borderRadius: "5px" }}>
                  <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
                    <h4 style={{ margin: "0 0 10px 0" }}>[{v.id}] {v.title}</h4>
                    <span style={{ fontWeight: "bold", padding: "3px 8px", background: "white", borderRadius: "3px", fontSize: "12px" }}>
                      Risk: {v.risk_level}
                    </span>
                  </div>
                  <p style={{ margin: "0 0 10px 0" }}>{v.description}</p>
                  <div style={{ fontSize: "14px" }}>
                    <strong>Etkilenen Bileşen:</strong> {v.affected_component}<br/>
                    <strong>Çözüm:</strong> {v.remediation}
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      )}
    </div>
  );
}
