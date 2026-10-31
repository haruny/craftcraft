// CraftCraft's usage statistics endpoint, a Cloudflare Worker with a D1 database.
// What it collects, and why, is in Privacy_Statement.md; keep the two in step.
//
//   GET  /catalog.json, /releases.json   passed on from GitHub Pages. Catalog checks are counted by
//                                        day, launcher version and type of computer (User-Agent).
//   POST /event                          usage statistics from launchers that have them turned on.
//   POST /site                           a page view or Download click on the website.
//   GET  /stats.json                     public totals, recomputed daily (see publicStats).
//   GET  /badge/<name>                   one of them as a shields.io endpoint badge:
//                                        https://img.shields.io/endpoint?url=<this>/badge/users
//   daily                                snapshots the launcher's GitHub download counts, deletes
//                                        events older than EVENT_DAYS and recomputes the totals.
//
// IP addresses are never stored: only the country Cloudflare derives from one.

const ORIGIN = "https://haruny.github.io/craftcraft";
const LAUNCHER_REPO = "haruny/craftcraft";
const EVENT_DAYS = 400;

const PROXIED = new Set(["/catalog.json", "/releases.json"]);
const EVENTS = new Set(["first_run", "launch", "app_install", "app_update", "app_install_failed", "app_uninstall", "app_open"]);
const SITE_EVENTS = new Set(["view", "download_click"]);
const MAX_BODY = 2048;
// Public totals leave out any group with fewer installations than this, so none points at a person.
const MIN_GROUP = 5;
const BRAND = "e8662e";

export default {
  async fetch(request, env, ctx) {
    const { pathname } = new URL(request.url);
    if (request.method === "GET" && PROXIED.has(pathname)) return proxy(request, pathname, env, ctx);
    if (request.method === "GET" && pathname === "/stats.json") return json(await stats(env));
    if (request.method === "GET" && pathname.startsWith("/badge/")) return badge(pathname.slice(7), await stats(env));
    if (request.method === "POST" && pathname === "/event") return event(request, env);
    if (request.method === "POST" && pathname === "/site") return site(request, env);
    return new Response("Not found\n", { status: 404 });
  },

  async scheduled(_controller, env, ctx) {
    ctx.waitUntil(daily(env));
  },
};

async function proxy(request, pathname, env, ctx) {
  const headers = new Headers({ "User-Agent": "craftcraft-telemetry" });
  const etag = request.headers.get("If-None-Match");
  if (etag) headers.set("If-None-Match", etag);
  const res = await fetch(ORIGIN + pathname, { headers, cf: { cacheTtl: 300 } });
  if (pathname === "/catalog.json") {
    const ua = parseLauncherAgent(request.headers.get("User-Agent") ?? "");
    if (ua) ctx.waitUntil(count(env, "catalog_check", { ...ua, country: country(request) }).catch(() => {}));
  }
  return new Response(res.body, res);
}

/** `CraftCraft/0.2.0 (windows; x86_64; 10.0.26300; +https://…)` */
function parseLauncherAgent(ua) {
  const m = /^CraftCraft\/([^ ]+) \(([^;]+); ([^;]+); ([^;]+);/.exec(ua);
  return m && { launcher: clip(m[1], 24), os: clip(m[2], 16), arch: clip(m[3], 16), os_version: clip(m[4], 32) };
}

async function event(request, env) {
  const body = await readJson(request);
  if (!body) return new Response("Bad request\n", { status: 400 });
  const { id, event, app, app_version, detail, launcher, os, arch, os_version } = body;
  if (typeof id !== "string" || !/^[0-9a-f]{32}$/.test(id) || !EVENTS.has(event)) return new Response("Bad request\n", { status: 400 });
  if (app != null && (typeof app !== "string" || !/^[a-z0-9][a-z0-9-]{0,63}$/.test(app))) return new Response("Bad request\n", { status: 400 });
  await env.DB.prepare(
    `INSERT INTO events (day, install_id, event, app, app_version, detail, launcher, os, arch, os_version, country)
     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
  )
    .bind(today(), id, event, app ?? "", clip(app_version, 32), clip(detail, 16), clip(launcher, 24), clip(os, 16), clip(arch, 16), clip(os_version, 32), country(request))
    .run();
  return new Response(null, { status: 204 });
}

async function site(request, env) {
  // Sent with navigator.sendBeacon, so there's no preflight and nobody reads the answer.
  const body = await readJson(request);
  if (!body || !SITE_EVENTS.has(body.event)) return new Response("Bad request\n", { status: 400 });
  const os = browserOs(request.headers.get("User-Agent") ?? "");
  await count(env, `site_${body.event}`, { launcher: "", os, arch: "", os_version: "", country: country(request) });
  return new Response(null, { status: 204 });
}

function browserOs(ua) {
  if (/Windows/.test(ua)) return "windows";
  if (/iPhone|iPad|iPod/.test(ua)) return "ios";
  if (/Mac OS X|Macintosh/.test(ua)) return "macos";
  if (/Android/.test(ua)) return "android";
  if (/Linux|X11/.test(ua)) return "linux";
  return "other";
}

function count(env, kind, { launcher, os, arch, os_version, country }) {
  return env.DB.prepare(
    `INSERT INTO counts (day, kind, launcher, os, arch, os_version, country, n) VALUES (?, ?, ?, ?, ?, ?, ?, 1)
     ON CONFLICT (day, kind, launcher, os, arch, os_version, country) DO UPDATE SET n = n + 1`,
  )
    .bind(today(), kind, launcher, os, arch, os_version, country)
    .run();
}

async function daily(env) {
  const headers = { Accept: "application/vnd.github+json", "User-Agent": "craftcraft-telemetry" };
  if (env.GITHUB_TOKEN) headers.Authorization = `Bearer ${env.GITHUB_TOKEN}`;
  const day = today();
  const rows = [];
  for (let page = 1; page <= 10; page++) {
    const res = await fetch(`https://api.github.com/repos/${LAUNCHER_REPO}/releases?per_page=100&page=${page}`, { headers });
    if (!res.ok) throw new Error(`GitHub said ${res.status}`);
    const releases = await res.json();
    for (const r of releases) for (const a of r.assets) rows.push([day, r.tag_name, a.name, a.download_count]);
    if (releases.length < 100) break;
  }
  const insert = env.DB.prepare(`INSERT OR REPLACE INTO downloads (day, tag, asset, downloads) VALUES (?, ?, ?, ?)`);
  const cutoff = new Date(Date.now() - EVENT_DAYS * 86400e3).toISOString().slice(0, 10);
  await env.DB.batch([...rows.map((r) => insert.bind(...r)), env.DB.prepare(`DELETE FROM events WHERE day < ?`).bind(cutoff)]);
  await publish(env);
}

/** The published totals, computing them the first time. */
async function stats(env) {
  const row = await env.DB.prepare(`SELECT json FROM published WHERE name = 'stats'`).first();
  return row ? JSON.parse(row.json) : publish(env);
}

/**
 * Totals safe to show anyone: no ids, no single events, and no group (an OS, an app) smaller than
 * MIN_GROUP. Users and apps count only installations that share usage statistics.
 */
async function publish(env) {
  const all = async (sql) => (await env.DB.prepare(sql).all()).results;
  const one = async (sql) => (await env.DB.prepare(sql).first("n")) ?? 0;
  const since = `date('now', '-30 days')`;
  const users = await one(`SELECT count(DISTINCT install_id) AS n FROM events WHERE day >= ${since}`);
  const appInstalls = await one(`SELECT count(*) AS n FROM events WHERE event = 'app_install' AND day >= ${since}`);
  const downloads = await one(`SELECT sum(downloads) AS n FROM downloads WHERE day = (SELECT max(day) FROM downloads)`);
  const platforms = await all(
    `SELECT os, count(DISTINCT install_id) AS users FROM events WHERE day >= ${since} GROUP BY os HAVING users >= ${MIN_GROUP} ORDER BY users DESC`,
  );
  const apps = await all(
    `SELECT app, count(DISTINCT install_id) AS users FROM events WHERE app != '' AND day >= ${since}
     GROUP BY app HAVING users >= ${MIN_GROUP} ORDER BY users DESC LIMIT 10`,
  );
  // Shares of everyone, so the platforms left out still count against the ones shown.
  const shown = Math.max(users, 1);
  const result = {
    generated: new Date().toISOString(),
    users_30d: users >= MIN_GROUP ? users : 0,
    app_installs_30d: appInstalls,
    launcher_downloads: downloads,
    platforms: platforms.map((p) => ({ os: p.os, share: Math.round((100 * p.users) / shown) })),
    top_apps: apps,
  };
  await env.DB.prepare(`INSERT OR REPLACE INTO published (name, json) VALUES ('stats', ?)`).bind(JSON.stringify(result)).run();
  return result;
}

const OS_NAMES = { windows: "Windows", macos: "macOS", linux: "Linux" };

/** https://shields.io/badges/endpoint-badge */
function badge(name, s) {
  const badges = {
    users: () => ["monthly users", s.users_30d ? compact(s.users_30d) : "new"],
    downloads: () => ["downloads", compact(s.launcher_downloads)],
    "app-installs": () => ["app installs / month", compact(s.app_installs_30d)],
    platforms: () => ["runs on", s.platforms.map((p) => `${OS_NAMES[p.os] ?? p.os} ${p.share}%`).join(" · ") || "Windows · macOS · Linux"],
    "top-app": () => ["most used", s.top_apps[0]?.app ?? "–"],
  };
  if (!badges[name]) return new Response("Not found\n", { status: 404 });
  const [label, message] = badges[name]();
  return json({ schemaVersion: 1, label, message, color: BRAND, cacheSeconds: 3600 });
}

/** 1234 → 1.2k */
function compact(n) {
  return new Intl.NumberFormat("en", { notation: "compact", maximumFractionDigits: 1 }).format(n).toLowerCase();
}

function json(value) {
  return new Response(JSON.stringify(value), {
    headers: { "Content-Type": "application/json", "Cache-Control": "public, max-age=3600", "Access-Control-Allow-Origin": "*" },
  });
}

async function readJson(request) {
  const text = await request.text();
  if (text.length > MAX_BODY) return null;
  try {
    const value = JSON.parse(text);
    return value && typeof value === "object" ? value : null;
  } catch {
    return null;
  }
}

/** A short plain string: letters, digits and `. _ -` and spaces only. */
function clip(value, max) {
  return typeof value === "string" ? value.replace(/[^\w.\- ]/g, "").slice(0, max) : "";
}

const country = (request) => clip(request.cf?.country ?? "", 2);
const today = () => new Date().toISOString().slice(0, 10);
