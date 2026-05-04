import { useState } from "react";
import "./App.css";
import { AttackVectors } from "./components/AttackVectors";
import { TechnicalSummary } from "./components/TechnicalSummary";
import { RemediationGuide } from "./components/RemediationGuide";

type Tab = "scanner" | "analysis" | "hardening";

const TABS: { id: Tab; label: string }[] = [
  { id: "scanner",   label: "Zafiyet Tarayıcı" },
  { id: "analysis",  label: "Platform Analizi" },
  { id: "hardening", label: "Sertleştirme" },
];

function ShieldIcon() {
  return (
    <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
      <path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4z" />
    </svg>
  );
}

export default function App() {
  const [tab, setTab] = useState<Tab>("scanner");

  return (
    <div className="app">
      {/* ── Header ── */}
      <header className="hdr">
        <div className="hdr-brand">
          <div className="hdr-icon">
            <ShieldIcon />
          </div>
          <div className="hdr-wordmark">
            <span className="hdr-title">Vercel Security Analyzer</span>
            <span className="hdr-meta">v0.1.0 · Tauri 2 + Rust</span>
          </div>
        </div>

        <div className="hdr-sep" />

        <nav className="nav">
          {TABS.map(({ id, label }) => (
            <button
              key={id}
              className={`nav-item ${tab === id ? "on" : ""}`}
              onClick={() => setTab(id)}
            >
              <span className="nav-pip" />
              {label}
            </button>
          ))}
        </nav>

        <div className="hdr-spacer" />

        <div className="ready-badge">
          <span className="ready-dot" />
          Hazır
        </div>
      </header>

      {/* ── Pages ── */}
      <main className="app-main">
        {tab === "scanner"   && <AttackVectors />}
        {tab === "analysis"  && <TechnicalSummary />}
        {tab === "hardening" && <RemediationGuide />}
      </main>
    </div>
  );
}
