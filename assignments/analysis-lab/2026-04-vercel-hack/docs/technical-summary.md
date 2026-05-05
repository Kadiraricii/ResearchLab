# Technical Summary — April 2026 Vercel Hack

## Incident overview

In April 2026, a supply chain attack compromised Vercel-hosted applications through the `context-ai` analytics package (v2.4.1). A contributor's credentials were stolen, allowing the attacker to publish a malicious version that exfiltrated environment variables from the build and runtime environments.

## Root causes

### 1. Non-sensitive environment variable designation

Vercel separates env vars into "Sensitive" (encrypted, masked in UI) and non-sensitive (readable from the dashboard). Developers routinely leave vars like `API_BASE_URL`, `CDN_DOMAIN`, or `NEXT_PUBLIC_ANALYTICS_KEY` as non-sensitive because they don't appear to be secrets.

The malicious package iterated `process.env` and exfiltrated all non-sensitive vars to an attacker-controlled endpoint during the build phase — before any runtime protections could intervene.

### 2. Supply chain trust

The `context-ai` package was widely used and had no breaking changes in v2.4.1. Most projects did not pin exact versions, allowing the malicious update to propagate automatically via `npm update` or Vercel's auto-rebuild-on-dependency-update feature.

### 3. Missing CSP

Applications without a Content Security Policy allowed injected scripts to execute freely and make arbitrary outbound connections during the frontend bundle evaluation.

## Impact

- **Direct:** API keys, admin tokens, and database connection strings exfiltrated
- **Indirect:** Attacker-controlled `API_BASE_URL` redirected traffic to phishing endpoints; CDN domain poisoning injected malicious scripts into legitimate frontend bundles

## Timeline

| Phase | Action |
|-------|--------|
| T-0 | Attacker publishes `context-ai@2.4.1` with hidden exfiltration payload |
| T+2h | Vercel auto-rebuilds trigger for projects with `^2.x` version range |
| T+6h | First reports of unusual outbound connections in build logs |
| T+18h | Vercel platform team identifies malicious package and pulls it |
| T+24h | Incident disclosed; affected teams begin credential rotation |

## Affected Vercel configuration patterns

```json
// Vulnerable: no CSP, secrets not marked sensitive
{
  "headers": []
}

// Vulnerable: NEXT_PUBLIC_ used for sensitive data
// .env.production:
// NEXT_PUBLIC_STRIPE_SECRET=sk_live_...
// NEXT_PUBLIC_DB_URL=postgres://...
```

## Analyzer coverage

This tool detects the following patterns from the incident:

| Check ID | Description |
|----------|-------------|
| `ENV_01` | `NEXT_PUBLIC_` prefix on secret-looking key names |
| `ENV_02` | Common secret patterns in env content (API_KEY, SECRET, PASSWORD) |
| `HDR_01` | Missing Content-Security-Policy header |
| `HDR_02` | Missing Strict-Transport-Security header |
| `BUILD_01` | Secret-like strings detected in build command |

## References

- Vercel Incident Report (April 2026)
- `docs/attack-vectors.md` — full vector analysis
- `docs/hardening-guide.md` — remediation steps
