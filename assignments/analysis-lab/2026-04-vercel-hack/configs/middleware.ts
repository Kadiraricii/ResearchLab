import { NextResponse } from 'next/server';
import type { NextRequest } from 'next/server';

export function middleware(request: NextRequest) {
  // Basit Authentication Kontrolu
  const authCookie = request.cookies.get('session-token');
  
  if (request.nextUrl.pathname.startsWith('/api/admin')) {
    if (!authCookie || authCookie.value !== 'valid-token') {
      return NextResponse.json(
        { success: false, message: 'Unauthorized Access' },
        { status: 401 }
      );
    }
  }

  const response = NextResponse.next();
  // Edge seviyesinde CSP basligi
  response.headers.set('Content-Security-Policy', "default-src 'self'");
  return response;
}

export const config = {
  matcher: ['/api/:path*'],
};
