import { createServer } from 'node:http';
import { readFile, stat } from 'node:fs/promises';
import { extname, join, normalize } from 'node:path';

const root = new URL('../dist/site/', import.meta.url).pathname;
const types = { '.html':'text/html; charset=utf-8','.js':'text/javascript; charset=utf-8','.css':'text/css; charset=utf-8','.json':'application/json; charset=utf-8','.svg':'image/svg+xml','.webp':'image/webp','.png':'image/png','.xml':'application/xml; charset=utf-8','.txt':'text/plain; charset=utf-8','.webmanifest':'application/manifest+json' };
createServer(async (request,response)=>{
  const pathname = decodeURIComponent(new URL(request.url ?? '/', 'http://local').pathname);
  const clean = normalize(pathname).replace(/^(\.\.(\/|\\|$))+/, '');
  let file = join(root, clean);
  try {
    const info = await stat(file);
    if (info.isDirectory()) file = join(file,'index.html');
    const body = await readFile(file);
    response.writeHead(200, {'Content-Type':types[extname(file)] ?? 'application/octet-stream','X-Content-Type-Options':'nosniff','Referrer-Policy':'strict-origin-when-cross-origin','Content-Security-Policy':"default-src 'self'; connect-src 'self'; img-src 'self' data:; object-src 'none'; base-uri 'self'; form-action 'self'; frame-ancestors 'none'"});
    response.end(body);
  } catch {
    const body = await readFile(join(root,'404.html'));
    response.writeHead(404, {'Content-Type':'text/html; charset=utf-8','X-Content-Type-Options':'nosniff'});
    response.end(body);
  }
}).listen(4173,'127.0.0.1');

