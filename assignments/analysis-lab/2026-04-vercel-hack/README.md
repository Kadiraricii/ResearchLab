# Vercel Security Analyzer

A desktop security scanner for Vercel deployments — audits projects for common misconfigurations, exposed secrets, and missing hardening controls.

[![CI](https://github.com/kadirarici/vercel-security-analyzer/actions/workflows/ci.yml/badge.svg)](https://github.com/kadirarici/vercel-security-analyzer/actions/workflows/ci.yml)
[![Release](https://github.com/kadirarici/vercel-security-analyzer/actions/workflows/release.yml/badge.svg)](https://github.com/kadirarici/vercel-security-analyzer/actions/workflows/release.yml)

## What it detects

| Category | Checks |
|----------|--------|
| Env vars | `NEXT_PUBLIC_` secret exposure, non-sensitive flag misuse |
| Headers | Missing HSTS, CSP, X-Frame-Options, Referrer-Policy |
| CORS | Wildcard origin, credentials + wildcard combo |
| Redirects | Open redirect patterns in `vercel.json` |
| Rewrites | Internal API path traversal via rewrites |
| Source maps | Production `.map` file exposure |
| Serverless | Missing input validation indicators |
| Middleware | Auth bypass patterns |
| SSL/DNS | Misconfigured domains, missing HTTPS |
| Preview auth | Unauthenticated preview deployments |
| Build logs | Secret leakage in build output |

## Installation

### Pre-built binaries

Download the latest installer for your platform from [Releases](https://github.com/kadirarici/vercel-security-analyzer/releases).

| Platform | File |
|----------|------|
| macOS (Apple Silicon) | `Vercel.Security.Analyzer_aarch64.dmg` |
| macOS (Intel) | `Vercel.Security.Analyzer_x86_64.dmg` |
| Windows | `Vercel.Security.Analyzer_x64-setup.exe` |
| Linux | `vercel-security-analyzer_amd64.AppImage` |

### Build from source

**Prerequisites:** Rust stable, Bun, system WebKit (Linux only)

```bash
# Linux system deps
sudo apt install libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf

# Clone and build
git clone https://github.com/kadirarici/vercel-security-analyzer
cd vercel-security-analyzer/assignments/analysis-lab/2026-04-vercel-hack
bun install
bun run tauri build
```

## Usage

1. Launch the app
2. Go to **Settings** → paste your Vercel API token
3. Go to **Scanner** → click **Run Full Scan**
4. Review findings in **Reports** and follow the **Hardening** checklist

### Vercel API token

Generate a token at `vercel.com/account/tokens` with **Read** scope. The token is stored locally and never transmitted outside the app.

## Development

```bash
# Install deps
bun install

# Start dev server (hot reload)
bun run tauri dev

# Run Rust tests
cd src-tauri && cargo nextest run

# Run all tests (unit + integration)
cargo nextest run --profile ci

# Run benchmarks
cargo bench

# Type-check frontend
bun run tsc --noEmit
```

## Project structure

```
├── src/                    # React/TypeScript frontend
│   ├── App.tsx             # 6-tab shell (dashboard, scanner, analysis, hardening, reports, settings)
│   └── components/
│       ├── AttackVectors.tsx   # Scanner UI
│       ├── Dashboard.tsx       # Overview
│       ├── RemediationGuide.tsx # Interactive hardening checklist
│       ├── Reports.tsx         # Scan history
│       └── Settings.tsx        # Token + project config
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── analyzer/       # 12 security analyzers (rayon-parallel)
│   │   ├── vercel/         # Vercel REST API client (HTTP/2, TTL cache)
│   │   ├── db/             # SQLite persistence (rusqlite bundled)
│   │   ├── remediation/    # Report + recommendation engine
│   │   └── commands.rs     # Tauri IPC commands
│   └── tauri.conf.json
├── docs/                   # Technical documentation
├── tests/                  # Integration + property-based tests
└── benches/                # Criterion benchmarks
```

## Architecture

The Rust backend runs all 12 analyzers in parallel via `rayon`. The Vercel API client uses HTTP/2 with a TTL-based DashMap cache and semaphore-based rate limiting. Results are persisted in a local SQLite database via `rusqlite` (bundled — no system SQLite required).

## Documentation

- [Attack Vectors](docs/attack-vectors.md)
- [Hardening Guide](docs/hardening-guide.md)
- [Technical Summary](docs/technical-summary.md)
- [Scripts & Configs](docs/scripts-and-configs.md)
- [Testing](docs/testing.md)
- [CI/CD](docs/ci-cd.md)

## License

MIT
