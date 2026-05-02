import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

export function RemediationGuide() {
  const [recommendations, setRecommendations] = useState<string[]>([]);
  const [vercelJsonTemplate, setVercelJsonTemplate] = useState("");
  const [nextConfigTemplate, setNextConfigTemplate] = useState("");
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function fetchData() {
      try {
        const recs: string[] = await invoke("generate_recommendations");
        const [vJson, nConfig]: [string, string] = await invoke("get_remediation_templates");
        setRecommendations(recs);
        setVercelJsonTemplate(vJson);
        setNextConfigTemplate(nConfig);
      } catch (e: any) {
        setError(e.toString());
      }
    }
    fetchData();
  }, []);

  return (
    <div style={{ padding: "20px", fontFamily: "system-ui" }}>
      <h2>Sertleştirme ve Düzeltme Rehberi</h2>
      <p>Aşağıdaki adımları ve güvenli yapılandırma şablonlarını uygulayarak Vercel projenizdeki zafiyetleri kapatın.</p>
      
      {error && <div style={{ color: "red", marginBottom: "10px" }}>Hata: {error}</div>}

      <div style={{ marginBottom: "30px", background: "#f8f9fa", padding: "20px", borderRadius: "5px", borderLeft: "5px solid #0070f3" }}>
        <h3>Önerilen Aksiyonlar</h3>
        <ul style={{ lineHeight: "1.6" }}>
          {recommendations.map((rec, idx) => (
            <li key={idx}><strong>{rec.split(':')[0]}:</strong> {rec.split(':')[1]}</li>
          ))}
        </ul>
      </div>

      <div style={{ display: "flex", gap: "20px" }}>
        <div style={{ flex: 1, background: "#1e1e1e", color: "#d4d4d4", padding: "15px", borderRadius: "5px" }}>
          <h3 style={{ color: "#fff", marginTop: 0 }}>Güvenli vercel.json Şablonu</h3>
          <pre style={{ whiteSpace: "pre-wrap", wordWrap: "break-word", fontSize: "13px" }}>
            {vercelJsonTemplate}
          </pre>
        </div>

        <div style={{ flex: 1, background: "#1e1e1e", color: "#d4d4d4", padding: "15px", borderRadius: "5px" }}>
          <h3 style={{ color: "#fff", marginTop: 0 }}>Güvenli next.config.js Şablonu</h3>
          <pre style={{ whiteSpace: "pre-wrap", wordWrap: "break-word", fontSize: "13px" }}>
            {nextConfigTemplate}
          </pre>
        </div>
      </div>
    </div>
  );
}
