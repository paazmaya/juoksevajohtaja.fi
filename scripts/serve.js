// Static server for the built site, behaving like the production host:
// directories serve index.html, unknown paths get dist/404.html (the pre-rendered SPA fallback).
const http = require("node:http");
const fs = require("node:fs");
const path = require("node:path");

const PORT = process.env.PORT ? Number.parseInt(process.env.PORT, 10) : 8080;
const ROOT_DIR = path.resolve(__dirname, "..", "dist");

const MIME_TYPES = {
  ".html": "text/html; charset=utf-8",
  ".js": "application/javascript; charset=utf-8",
  ".wasm": "application/wasm",
  ".json": "application/json; charset=utf-8",
  ".webmanifest": "application/manifest+json; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".xml": "application/xml; charset=utf-8",
  ".txt": "text/plain; charset=utf-8",
  ".png": "image/png",
  ".jpg": "image/jpeg",
  ".jpeg": "image/jpeg",
  ".webp": "image/webp",
  ".svg": "image/svg+xml",
  ".ico": "image/x-icon",
  ".mp3": "audio/mpeg",
};

function resolveFile(urlPath) {
  const filePath = path.join(ROOT_DIR, urlPath);
  if (!filePath.startsWith(ROOT_DIR)) {
    return null;
  }
  if (fs.existsSync(filePath) && fs.statSync(filePath).isDirectory()) {
    const index = path.join(filePath, "index.html");
    return fs.existsSync(index) ? index : null;
  }
  return fs.existsSync(filePath) ? filePath : null;
}

const server = http.createServer((req, res) => {
  const urlPath = decodeURIComponent(new URL(req.url, `http://localhost:${PORT}`).pathname);
  let filePath = resolveFile(urlPath);
  let status = 200;

  if (!filePath) {
    status = 404;
    filePath = path.join(ROOT_DIR, "404.html");
    if (!fs.existsSync(filePath)) {
      res.writeHead(404, { "Content-Type": "text/plain" });
      res.end("404 Not Found");
      return;
    }
  }

  const ext = path.extname(filePath).toLowerCase();
  res.writeHead(status, {
    "Content-Type": MIME_TYPES[ext] || "application/octet-stream",
    "Cache-Control": "no-cache",
  });
  fs.createReadStream(filePath).pipe(res);
});

server.listen(PORT, () => {
  console.log(`Static server running at http://localhost:${PORT}/ serving ${ROOT_DIR}`);
});
