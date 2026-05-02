import { useState } from "react";
import "./App.css";
import { TechnicalSummary } from "./components/TechnicalSummary";
import { AttackVectors } from "./components/AttackVectors";
import { RemediationGuide } from "./components/RemediationGuide";

function App() {
  const [activeTab, setActiveTab] = useState("summary");

  return (
    <div style={{ maxWidth: "1200px", margin: "0 auto", padding: "20px" }}>
      <header style={{ display: "flex", gap: "10px", marginBottom: "20px", borderBottom: "1px solid #ccc", paddingBottom: "10px" }}>
        <button 
          onClick={() => setActiveTab("summary")}
          style={{ padding: "10px 20px", border: "none", background: activeTab === "summary" ? "#e0e0e0" : "transparent", cursor: "pointer", fontWeight: activeTab === "summary" ? "bold" : "normal" }}
        >
          Phase 1: Teknik Özet
        </button>
        <button 
          onClick={() => setActiveTab("vectors")}
          style={{ padding: "10px 20px", border: "none", background: activeTab === "vectors" ? "#e0e0e0" : "transparent", cursor: "pointer", fontWeight: activeTab === "vectors" ? "bold" : "normal" }}
        >
          Phase 2: Zafiyet Analizi
        </button>
        <button 
          onClick={() => setActiveTab("remediation")}
          style={{ padding: "10px 20px", border: "none", background: activeTab === "remediation" ? "#e0e0e0" : "transparent", cursor: "pointer", fontWeight: activeTab === "remediation" ? "bold" : "normal" }}
        >
          Phase 3: Sertleştirme
        </button>
      </header>
      
      <main>
        {activeTab === "summary" && <TechnicalSummary />}
        {activeTab === "vectors" && <AttackVectors />}
        {activeTab === "remediation" && <RemediationGuide />}
      </main>
    </div>
  );
}

export default App;
