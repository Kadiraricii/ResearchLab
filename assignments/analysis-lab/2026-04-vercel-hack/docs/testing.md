# Testing Strategy

## Test layout

```
src-tauri/
├── src/
│   └── analyzer/
│       └── headers.rs        # #[cfg(test)] inline unit tests
├── tests/
│   ├── common/mod.rs         # shared helpers: test_db(), assert::has_vuln()
│   ├── unit/
│   │   ├── remediation_tests.rs   # remediation engine unit tests
│   │   ├── summary_tests.rs       # config parser unit tests
│   │   └── proptest_analyzer.rs   # property-based tests (#[ignore])
│   ├── integration/
│   │   └── full_scan_pipeline.rs  # end-to-end Rust pipeline tests
│   └── mocks/
│       └── vercel_mock_server.rs  # wiremock HTTP server
└── benches/
    ├── analyzer_bench.rs     # analyzer throughput benchmarks
    └── db_bench.rs           # database CRUD benchmarks
```

## Running tests

```bash
# All tests (parallel, fast)
cargo nextest run

# CI profile — retries + JUnit XML
cargo nextest run --profile ci

# Property-based / slow tests (run manually or weekly in CI)
cargo test -- --ignored

# Single test by name
cargo nextest run --test full_scan_pipeline

# With log output
RUST_LOG=debug cargo nextest run
```

## Test helpers (`tests/common/mod.rs`)

```rust
// In-memory SQLite DB with real schema migrations
let db = test_db();

// OnceLock fixture loading (disk read once, shared across all tests)
let json = vulnerable_vercel_json();

// Assertion helpers
assert::has_vuln(&report, "ENV_01");
assert::no_vuln(&report, "CORS_01");
assert::score_in_range(&report, 5.0, 10.0);
```

## Fixture files (`tests/fixtures/`)

| File | Purpose |
|------|---------|
| `vulnerable_vercel.json` | Config with multiple known vulnerabilities |
| `secure_vercel.json` | Fully hardened config (should produce no findings) |
| `valid_projects.json` | Mock Vercel API projects response |
| `valid_deployments.json` | Mock Vercel API deployments response |
| `mixed_findings.json` | Mix of risk levels for scoring tests |

## Writing new tests

### Unit test (in source file)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_wildcard_cors() {
        let input = r#"{"headers":[{"source":"/api/(.*)","headers":[
            {"key":"Access-Control-Allow-Origin","value":"*"}
        ]}]}"#;
        let vulns = analyze(input, "");
        assert!(vulns.iter().any(|v| v.id == "CORS_01"), "expected CORS_01");
    }
}
```

### Integration test

```rust
// tests/integration/my_test.rs
mod common;

#[test]
fn full_pipeline_finds_env_vulns() {
    use vercel_hack_analysis::analyzer::run_all_analyzers;
    let report = run_all_analyzers("", "", "NEXT_PUBLIC_SECRET=abc123");
    common::assert::has_vuln(&report, "ENV_01");
}
```

### Property-based test

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    #[ignore]   // slow — runs in CI weekly job only
    fn analyzer_never_panics(input in ".*") {
        let _ = analyze(&input, "");
    }
}
```

## Coverage

```bash
# Generate HTML report
cargo llvm-cov --html
# Open target/llvm-cov/html/index.html

# Check threshold
cargo llvm-cov --fail-under-lines 70
```

Coverage targets: Rust ≥ 70%, Frontend ≥ 60%.

## Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench db_bench

# Generate flamegraph (requires cargo-flamegraph)
cargo flamegraph --bench analyzer_bench
```

Benchmark results are stored in `target/criterion/` and uploaded as CI artifacts on the `main` branch.
