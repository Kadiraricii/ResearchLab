# Contributing

## Dev environment setup

**Required tools:**
- Rust stable (`rustup update stable`)
- Bun (`curl -fsSL https://bun.sh/install | bash`)
- cargo-nextest (`cargo install cargo-nextest --locked`)

**Linux only:**
```bash
sudo apt install libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf mold
```

**First-time setup:**
```bash
bun install
cd src-tauri && cargo check
```

## Running the app

```bash
bun run tauri dev
```

## Running tests

```bash
# All Rust tests
cd src-tauri && cargo nextest run

# CI profile (retries, junit output)
cargo nextest run --profile ci

# Slow/property-based tests (runs separately)
cargo test -- --ignored

# Frontend type check
bun run tsc --noEmit
```

## Commit format

```
<type>: <description>
```

Types: `feat`, `fix`, `refactor`, `docs`, `test`, `chore`, `perf`, `ci`

Examples:
```
feat(analyzer): add middleware bypass detection
fix(db): handle mutex poison on scan delete
test(cors): add property-based wildcard origin tests
```

## Writing tests

- Unit tests go in `#[cfg(test)]` blocks inside the source file
- Integration tests go in `tests/`
- Use `tests/common/mod.rs` helpers (`test_db()`, `assert::has_vuln()`)
- Property-based tests use `proptest!` macro and must be marked `#[ignore]`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::common::test_db;

    #[test]
    fn detects_wildcard_cors() {
        let vulns = analyze(r#"{"headers":[{"source":"/api/(.*)","headers":[{"key":"Access-Control-Allow-Origin","value":"*"}]}]}"#, "");
        assert!(vulns.iter().any(|v| v.id == "CORS_01"));
    }
}
```

## PR process

1. Branch from `master` or the active dev branch
2. Keep PRs focused — one feature or fix per PR
3. All CI checks must pass before review
4. Add or update tests for any logic change

## Code style

- `cargo fmt` before committing
- `cargo clippy -- -D warnings` must pass
- Functions < 50 lines; files < 800 lines
- No `unwrap()` in production paths — use `?` and typed errors
