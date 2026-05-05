# Vercel Hardening Guide

Step-by-step remediation for every finding this tool can produce.

## Environment Variables

**Mark all secrets as Sensitive**  
Vercel Dashboard → Project → Settings → Environment Variables → click the lock icon on every variable that contains a key, token, password, or connection string. Sensitive vars are encrypted at rest and masked in logs and the UI.

**Never use NEXT_PUBLIC_ for secrets**  
`NEXT_PUBLIC_` and `VITE_` vars are inlined into the frontend bundle at build time. Only use them for truly public data (feature flags, public API base URLs with no auth).

```bash
# Good
DATABASE_URL=postgres://...          # server-only, mark Sensitive
STRIPE_SECRET_KEY=sk_live_...        # server-only, mark Sensitive
NEXT_PUBLIC_APP_NAME=MyApp           # fine — not a secret

# Bad
NEXT_PUBLIC_API_KEY=sk_live_...      # exposed to everyone
NEXT_PUBLIC_DB_PASSWORD=hunter2      # critical exposure
```

## Security Headers

Add to `vercel.json`:

```json
{
  "headers": [
    {
      "source": "/(.*)",
      "headers": [
        { "key": "Strict-Transport-Security", "value": "max-age=63072000; includeSubDomains; preload" },
        { "key": "Content-Security-Policy",   "value": "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; connect-src 'self' https://api.vercel.com" },
        { "key": "X-Frame-Options",           "value": "DENY" },
        { "key": "X-Content-Type-Options",    "value": "nosniff" },
        { "key": "Referrer-Policy",           "value": "strict-origin-when-cross-origin" },
        { "key": "Permissions-Policy",        "value": "camera=(), microphone=(), geolocation=()" }
      ]
    }
  ]
}
```

## CORS

Only allow your own origin. For API routes in Next.js:

```ts
// pages/api/data.ts
export default function handler(req, res) {
  res.setHeader('Access-Control-Allow-Origin', 'https://yourdomain.com');
  res.setHeader('Access-Control-Allow-Methods', 'GET, POST');
  res.setHeader('Access-Control-Allow-Headers', 'Content-Type, Authorization');
  // Never set Allow-Origin: * on authenticated endpoints
}
```

## Redirects — prevent open redirect

Always anchor the destination:

```json
{
  "redirects": [
    {
      "source": "/old-path",
      "destination": "/new-path",
      "permanent": true
    }
  ]
}
```

Never pass user-controlled path segments to external `http://` destinations.

## Source Maps

In `next.config.js`:

```js
module.exports = {
  productionBrowserSourceMaps: false,  // default — keep this false
}
```

If you need source maps for error monitoring, configure your error tracking tool to upload maps privately instead.

## Preview Deployment Authentication

Enable Vercel Authentication for preview environments:  
Dashboard → Project → Settings → Deployment Protection → enable **Vercel Authentication** for Preview.

Or implement custom middleware:

```ts
// middleware.ts
import { NextRequest, NextResponse } from 'next/server';

export function middleware(req: NextRequest) {
  if (process.env.VERCEL_ENV === 'preview') {
    const token = req.cookies.get('preview-token');
    if (token?.value !== process.env.PREVIEW_SECRET) {
      return NextResponse.redirect(new URL('/login', req.url));
    }
  }
  return NextResponse.next();
}
```

## Build Log Secrets

Never pass secrets as CLI arguments:

```bash
# Bad — visible in Vercel build logs
npm run build -- --api-key=sk_live_abc123

# Good — read from environment at runtime
npm run build
# In code: process.env.API_KEY
```

## Dependency pinning

Pin exact versions to prevent supply chain attacks via auto-update:

```json
{
  "dependencies": {
    "context-ai": "2.4.0"
  }
}
```

Add `bun pm audit` or `npm audit` to CI to catch known vulnerabilities in the dependency tree.

## SSL / HSTS

Include HSTS with `preload` directive and submit your domain to the [HSTS Preload List](https://hstspreload.org/):

```
Strict-Transport-Security: max-age=63072000; includeSubDomains; preload
```

Minimum `max-age` for preload eligibility is 31536000 (1 year).

## Serverless Function Input Validation

Validate all inputs at the API boundary:

```ts
import { z } from 'zod';

const schema = z.object({
  userId: z.string().uuid(),
  action: z.enum(['read', 'write']),
});

export default function handler(req, res) {
  const result = schema.safeParse(req.body);
  if (!result.success) {
    return res.status(400).json({ error: 'Invalid input' });
  }
  // safe to use result.data
}
```
