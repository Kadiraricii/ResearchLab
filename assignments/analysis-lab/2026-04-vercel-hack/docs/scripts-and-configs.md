# Scripts & Configurations

All CLI commands, config files, and automation scripts used in this project.

## Development commands

```bash
# Start dev server with hot reload
bun run tauri dev

# Frontend only
bun run dev

# Production build
bun run tauri build

# Type check only
bun run tsc --noEmit
```

## Rust commands

```bash
cd src-tauri

# Check compilation (fast)
cargo check

# Run all tests
cargo nextest run

# Run CI profile (retries + junit)
cargo nextest run --profile ci

# Run property-based / slow tests
cargo test -- --ignored

# Run benchmarks
cargo bench

# Code coverage
cargo llvm-cov --html

# Lint
cargo clippy -- -D warnings

# Format
cargo fmt

# Security audit
cargo audit

# License + advisory check
cargo deny check

# Build timing report
cargo build --timings
```

## Key configuration files

### `.cargo/config.toml`
Build profiles, linker overrides, sccache setup, parallelism settings.

```toml
[build]
jobs = 10                           # parallel compilation units

[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=mold"]   # fast linker on Linux

[profile.release]
lto = "thin"
codegen-units = 1
strip = true
panic = "abort"
```

### `.config/nextest.toml`
Nextest test runner configuration — parallel groups, CI profile, E2E profile.

```toml
[profile.ci]
retries = 2
junit = { path = "junit.xml" }
slow-timeout = { period = "30s", terminate-after = 3 }
```

### `vite.config.ts`
Frontend build config — chunk splitting, tree shaking, esbuild minification.

```ts
manualChunks: { "vendor-react": ["react", "react-dom"], "vendor-tauri": ["@tauri-apps/api"] }
treeshake: { moduleSideEffects: false }
esbuild: { drop: ["console", "debugger"] }
```

### `src-tauri/tauri.conf.json`
Tauri app config — windows, CSP, bundle targets, auto-updater endpoint.

### `deny.toml`
`cargo deny` configuration — blocked advisories, allowed licenses.

### `clippy.toml`
Clippy lint configuration.

### `rustfmt.toml`
Rustfmt formatting rules.

### `rust-toolchain.toml`
Pinned Rust toolchain version.

## Rust CLI scripts (`scripts/`)

Pre-built Rust scanning utilities located in `scripts/`:

```bash
# Scan a local vercel.json file
scripts/scan_local --file vercel.json

# Batch scan via Vercel API
scripts/batch_scan --token $VERCEL_TOKEN --team $TEAM_ID

# Generate a JSON report
scripts/generate_report --scan-id <id> --format json --output report.json
```

## Docker

```bash
# Build container
docker build -t vercel-analyzer .

# Run tests in container
docker-compose -f docker-compose.yml run --rm test

# Scan with Docker (CI-friendly, no desktop)
docker run --rm -e VERCEL_TOKEN=$VERCEL_TOKEN vercel-analyzer scan
```

## Secure configuration templates

`configs/` directory contains ready-to-use templates:

| File | Purpose |
|------|---------|
| `configs/vercel-secure.json` | `vercel.json` with all recommended security headers |
| `configs/next-secure.config.js` | `next.config.js` with source maps disabled, strict mode |
| `configs/csp-strict.txt` | Strict CSP policy template |

## Environment variables reference

| Variable | Required | Description |
|----------|----------|-------------|
| `VERCEL_TOKEN` | Yes (for API scan) | Vercel personal API token |
| `VERCEL_TEAM_ID` | No | Restrict scan to a specific team |
| `RUST_LOG` | No | Log level (`debug`, `info`, `warn`, `error`) |
| `RUST_BACKTRACE` | No | Enable backtraces (`1` or `full`) |
