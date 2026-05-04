import { useState, type ReactNode } from "react";
import "./App.css";
import { Dashboard }       from "./components/Dashboard";
import { AttackVectors }   from "./components/AttackVectors";
import { TechnicalSummary } from "./components/TechnicalSummary";
import { RemediationGuide } from "./components/RemediationGuide";
import { Reports }          from "./components/Reports";
import { Settings }         from "./components/Settings";

// ─── Types ────────────────────────────────────────────────────────────────────

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

type Tab = "dashboard" | "scanner" | "analysis" | "hardening" | "reports" | "settings";

// ─── Nav config ───────────────────────────────────────────────────────────────

interface NavItem {
  id: Tab;
  label: string;
  icon: ReactNode;
}

function ShieldIcon() {
  return (
    <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
      <path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4z" />
    </svg>
  );
}

function NavIcon({ tab }: { tab: Tab }) {
  if (tab === "dashboard") return (
    <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
      <path d="M1 1h6v6H1V1zm8 0h6v6H9V1zM1 9h6v6H1V9zm8 0h6v6H9V9z" />
    </svg>
  );
  if (tab === "scanner") return (
    <svg width="12" height="12" viewBox="0 0 16 16" fill="none">
      <circle cx="7" cy="7" r="5" stroke="currentColor" strokeWidth="1.5" />
      <path d="M11 11L14 14" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" />
    </svg>
  );
  if (tab === "analysis") return (
    <svg width="12" height="12" viewBox="0 0 16 16" fill="none">
      <path d="M2 12L5 8L8 10L11 5L14 7" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  );
  if (tab === "hardening") return (
    <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
      <path d="M8 1L2 3.5v4.5c0 3.6 2.5 7 6 8 3.5-1 6-4.4 6-8V3.5L8 1z" />
    </svg>
  );
  if (tab === "reports") return (
    <svg width="12" height="12" viewBox="0 0 16 16" fill="none">
      <rect x="2" y="1" width="12" height="14" rx="1.5" stroke="currentColor" strokeWidth="1.5" />
      <path d="M5 5h6M5 8h6M5 11h4" stroke="currentColor" strokeWidth="1.3" strokeLinecap="round" />
    </svg>
  );
  // settings
  return (
    <svg width="12" height="12" viewBox="0 0 16 16" fill="none">
      <circle cx="8" cy="8" r="2.5" stroke="currentColor" strokeWidth="1.5" />
      <path d="M8 1v1.5M8 13.5V15M1 8h1.5M13.5 8H15M3.2 3.2l1.1 1.1M11.7 11.7l1.1 1.1M3.2 12.8l1.1-1.1M11.7 4.3l1.1-1.1" stroke="currentColor" strokeWidth="1.3" strokeLinecap="round" />
    </svg>
  );
}

const NAV_ITEMS: NavItem[] = [
  { id: "dashboard", label: "Ana Sayfa",    icon: <NavIcon tab="dashboard" /> },
  { id: "scanner",   label: "Tarayıcı",     icon: <NavIcon tab="scanner" /> },
  { id: "analysis",  label: "Platform",     icon: <NavIcon tab="analysis" /> },
  { id: "hardening", label: "Sertleştirme", icon: <NavIcon tab="hardening" /> },
  { id: "reports",   label: "Raporlar",     icon: <NavIcon tab="reports" /> },
  { id: "settings",  label: "Ayarlar",      icon: <NavIcon tab="settings" /> },
];

// ─── App ──────────────────────────────────────────────────────────────────────

export default function App() {
  const [tab, setTab]             = useState<Tab>("dashboard");
  const [lastReport, setLastReport] = useState<AnalysisReport | null>(null);

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
          {NAV_ITEMS.map(({ id, label, icon }) => (
            <button
              key={id}
              className={`nav-item ${tab === id ? "on" : ""}`}
              onClick={() => setTab(id)}
            >
              <span className="nav-icon">{icon}</span>
              <span className="nav-pip" />
              {label}
            </button>
          ))}
        </nav>

        <div className="hdr-spacer" />

        <div className="ready-badge">
          <span className="ready-dot" />
          {lastReport ? `${lastReport.vulnerabilities.length} bulgu` : "Hazır"}
        </div>
      </header>

      {/* ── Pages ── */}
      <main className="app-main">
        {tab === "dashboard" && (
          <Dashboard onNavigate={(t) => setTab(t as Tab)} />
        )}
        {tab === "scanner" && (
          <AttackVectors onReportReady={(r) => { setLastReport(r); setTab("reports"); }} />
        )}
        {tab === "analysis"  && <TechnicalSummary />}
        {tab === "hardening" && <RemediationGuide />}
        {tab === "reports"   && <Reports lastReport={lastReport} />}
        {tab === "settings"  && <Settings />}
      </main>
    </div>
  );
}
