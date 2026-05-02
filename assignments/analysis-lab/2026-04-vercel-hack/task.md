# Analysis Lab Case Study: VERCEL HACK (April 2026)
**UUID**: `434fce1d-2fa3-450c-9847-c27724a49552`
**Type**: `ANALYSIS-LAB`

## Mission Objective
Conduct in-depth research and analysis of the Vercel infrastructure breach involving the Context.ai supply chain incident.

## 1. Technical Summary
In April 2026, a sophisticated supply chain attack targeted Vercel users through the `context-ai` analytics package. The breach exploited a combination of "Non-Sensitive" environment variable designations and a malicious dependency update.

- **Incident Mechanism:** The `context-ai` package was hijacked via a contributor's compromised credentials. The malicious version (v2.4.1) scanned the process environment for keys that were not flagged as "Sensitive" in the Vercel dashboard.
- **Data Leakage:** Because many developers left "API_BASE_URL" or "CDN_DOMAIN" as non-sensitive, the attacker used these to redirect traffic or inject malicious scripts into the frontend build process.

## 2. Attack Vectors & Risks
- **Environment Variable Exposure:** Variables not marked as sensitive are readable by any process in the build/runtime environment. If a malicious dependency is included, it can exfiltrate these strings.
- **Supply Chain Poisoning:** Injecting code into the frontend bundle via hijacked analytics or monitoring scripts.
- **Risk Level:** **CRITICAL**. Total compromise of application logic and user data if database strings or admin tokens are exposed.

## 3. Hardening & Remediation Guide
1. **Audit All Env Vars:** Go to Vercel Dashboard > Project Settings > Environment Variables. Ensure all keys are marked as **Sensitive**.
2. **Pin Dependencies:** Use exact versions in `package.json` and verify `bun.lock` or `package-lock.json` integrity.
3. **Content Security Policy (CSP):** Implement a strict CSP to prevent unauthorized scripts from executing, even if injected.
4. **Environment Variable Prefixing:** Strictly use `NEXT_PUBLIC_` only for truly public data.

## 4. Verification Commands
- Check for non-sensitive vars: `vercel env pull .env.local`
- Audit dependencies: `bun pm audit` or `cargo audit`
- Verify CSP headers: `curl -I https://your-project.vercel.app`
