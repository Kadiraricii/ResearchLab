# Attack Vectors — Vercel Security Analysis

Twelve attack vectors identified across Vercel deployment configurations. Each maps to one or more analyzer checks in this tool.

## 1. Environment Variable Exposure — CRITICAL

`NEXT_PUBLIC_` or `VITE_` prefixed variables are embedded in the frontend JS bundle as plain text. Any key that looks like a secret (contains `KEY`, `SECRET`, `TOKEN`, `PASSWORD`, `AUTH`) but carries this prefix is immediately readable by anyone who downloads the page.

**Analyzer:** `ENV_01`, `ENV_02`  
**Trigger pattern:** `NEXT_PUBLIC_API_KEY=sk-...` in env content

## 2. Source Map Exposure — HIGH

When `productionBrowserSourceMaps: true` is set in `next.config.js`, the compiled `.map` files are served alongside the production bundle. This allows anyone to reconstruct the original TypeScript source, exposing business logic, internal API routes, and developer comments.

**Analyzer:** `SRC_01`  
**Trigger pattern:** `productionBrowserSourceMaps` in next config

## 3. Serverless Function Injection — CRITICAL

API routes under `/api` that pass URL parameters or request body fields directly to database queries or shell commands without validation. Leads to SQL injection, NoSQL injection, or RCE.

**Analyzer:** `SRV_01`  
**Trigger pattern:** Absence of input validation patterns in function bodies

## 4. Open Redirect — MEDIUM

`redirects` rules in `vercel.json` that use unanchored regex or pass the full `path` parameter to an external `destination`. Attackers craft URLs like `/redirect?to=https://phishing.com` to abuse trusted domains.

**Analyzer:** `RDR_01`  
**Trigger pattern:** External `http://` destination in redirects

## 5. CORS Misconfiguration — HIGH

`Access-Control-Allow-Origin: *` on authenticated endpoints, or accepting credentials with a wildcard origin. Allows attacker-controlled pages to make authenticated cross-origin requests on behalf of the victim.

**Analyzer:** `CORS_01`, `CORS_02`  
**Trigger pattern:** `"value": "*"` on ACAO header with credentials

## 6. Missing Security Headers — MEDIUM

Absence of HSTS, X-Frame-Options, Content-Security-Policy, or Referrer-Policy. Each missing header enables a specific browser-level attack: clickjacking, mixed content, XSS, referrer leakage.

**Analyzer:** `HDR_01` through `HDR_05`  
**Trigger pattern:** Headers array missing required security keys

## 7. Path Traversal via Rewrites — HIGH

`rewrites` rules that proxy requests to internal services can expose admin APIs if the source pattern is too broad. Example: `source: "/api/(.*)"` rewritten to an internal service that doesn't re-authenticate.

**Analyzer:** `RWT_01`  
**Trigger pattern:** Wildcard rewrite destination pointing to internal hostnames

## 8. DNS Misconfiguration — MEDIUM

Dangling CNAME records, missing CAA records, or expired SSL certificates create domain takeover opportunities or man-in-the-middle windows.

**Analyzer:** `DNS_01`  
**Trigger pattern:** Misconfigured custom domain entries

## 9. SSL/TLS Issues — HIGH

Missing HSTS preload, short `max-age`, or missing `includeSubDomains`. Allows downgrade attacks or cookie theft on subdomains.

**Analyzer:** `SSL_01`  
**Trigger pattern:** HSTS header present but misconfigured

## 10. Unauthenticated Preview Deployments — MEDIUM

Preview deployments are publicly accessible by default. Without Vercel Authentication or custom middleware, staging environments with unfinished features or debug endpoints are exposed to the internet.

**Analyzer:** `PRV_01`  
**Trigger pattern:** No authentication config detected for preview environment

## 11. Build Log Secret Leakage — HIGH

Secrets passed as CLI arguments (`--secret=value`) or echoed in build scripts appear in Vercel build logs, which may be accessible to team members or through log export integrations.

**Analyzer:** `BLD_01`  
**Trigger pattern:** Secret-like strings in `buildCommand` field

## 12. Middleware Auth Bypass — CRITICAL

Edge Middleware that implements authentication but has incomplete path matching (e.g., missing trailing slash normalization or case-insensitive bypass) allows attackers to access protected routes.

**Analyzer:** `MID_01`  
**Trigger pattern:** Middleware matchers with incomplete coverage

---

## Risk matrix

| Vector | Likelihood | Impact | Risk |
|--------|-----------|--------|------|
| Env exposure | High | Critical | **Critical** |
| Source maps | Medium | High | **High** |
| Serverless injection | Medium | Critical | **Critical** |
| Open redirect | High | Medium | **Medium** |
| CORS | Medium | High | **High** |
| Missing headers | High | Medium | **Medium** |
| Path traversal | Low | High | **High** |
| DNS | Low | High | **Medium** |
| SSL | Medium | High | **High** |
| Preview auth | High | Medium | **Medium** |
| Build log leakage | Medium | High | **High** |
| Middleware bypass | Low | Critical | **High** |
