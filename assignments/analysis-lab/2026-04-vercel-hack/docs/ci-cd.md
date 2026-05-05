# CI/CD Pipeline

## Workflows

| File | Trigger | Purpose |
|------|---------|---------|
| `.github/workflows/ci.yml` | push/PR to main/develop | Lint, test, coverage, security audit, build check |
| `.github/workflows/release.yml` | push tag `v*` | Cross-platform release build + GitHub Release |
| `.github/workflows/deps.yml` | scheduled | Dependency update checks |
| `.github/workflows/docker.yml` | push to main | Docker image build + push |

## CI pipeline (ci.yml)

```
push / PR
    │
    ├── JOB 1: Lint
    │   ├── cargo fmt --check
    │   ├── cargo clippy -- -D warnings
    │   ├── cargo deny (licenses + advisories)
    │   └── bun install (frontend deps)
    │
    ├── JOB 2: Unit Tests  (needs: lint)
    │   ├── cargo nextest run --profile ci
    │   └── upload junit.xml artifact
    │
    ├── JOB 3: Integration Tests  (needs: lint)
    │   └── cargo nextest run --profile ci --test '*'
    │
    ├── JOB 4: Coverage  (needs: unit-tests)
    │   ├── cargo llvm-cov --lcov
    │   ├── cargo llvm-cov --html
    │   └── upload coverage-html artifact
    │
    ├── JOB 5: Security Audit
    │   └── cargo audit
    │
    ├── JOB 6: Build Check  (needs: unit + integration)
    │   ├── cargo build --release
    │   ├── cargo build --timings → upload cargo-timing.html
    │   └── (Linux GTK/WebKit deps + mold)
    │
    ├── JOB 7: Benchmarks  (main branch only, needs: build)
    │   ├── cargo bench
    │   └── upload criterion results
    │
    └── JOB 8: Slow Tests  (workflow_dispatch or weekly schedule)
        └── cargo test -- --ignored  (proptest suite)
```

## Performance optimizations in CI

- **sccache** (`mozilla-actions/sccache-action@v0.0.5`) — shared compilation cache across jobs
- **mold linker** — installed on all Linux jobs, configured via `.cargo/config.toml`
- **Cargo registry cache** — `actions/cache@v4` on `~/.cargo/registry` + target dir
- **nextest** — parallel test execution with `num-cpus` threads

## Release pipeline (release.yml)

Triggered by `git push origin v0.1.0` (any `v*` tag).

Builds on 4 platforms in parallel:

| Platform | Target | Output |
|----------|--------|--------|
| macOS (ARM) | `aarch64-apple-darwin` | `.dmg` |
| macOS (Intel) | `x86_64-apple-darwin` | `.dmg` |
| Ubuntu 22.04 | `x86_64-unknown-linux-gnu` | `.AppImage`, `.deb` |
| Windows | `x86_64-pc-windows-msvc` | `.msi`, `.exe` |

Uses `tauri-apps/tauri-action@v0` which:
1. Builds the frontend (`bun run build`)
2. Compiles the Rust binary
3. Packages the installer
4. Creates a GitHub Release with all artifacts
5. Generates `latest.json` for the auto-updater (requires `TAURI_SIGNING_PRIVATE_KEY` secret)

## Required GitHub Secrets

| Secret | Used by | Description |
|--------|---------|-------------|
| `TAURI_SIGNING_PRIVATE_KEY` | release.yml | Updater package signing key |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | release.yml | Key password (empty if generated with `--ci`) |

## Releasing a new version

```bash
# 1. Update version in src-tauri/Cargo.toml and tauri.conf.json
# 2. Commit
git add -p && git commit -m "chore: bump version to 0.2.0"

# 3. Tag
git tag v0.2.0

# 4. Push tag — triggers release workflow
git push origin v0.2.0
```

## Local CI simulation

```bash
# Simulate lint job
cargo fmt --check && cargo clippy -- -D warnings && cargo deny check

# Simulate test job
cargo nextest run --profile ci

# Simulate build job
cargo build --release

# Simulate all
bun run tsc --noEmit && cargo nextest run && cargo build --release
```
