#!/usr/bin/env node
// Writes releases.json: each app's latest release (with the download files the catalog names and
// their SHA-256), its GitHub stars and its license.
//
// The catalog workflow publishes it next to catalog.json every hour, so a launcher learns about
// every app's updates from one cached request to GitHub Pages, instead of asking GitHub's API
// about each app (which allows 60 requests an hour without a token). It costs the workflow two
// API requests per app per run.
//
//   node catalog/releases.mjs <out.json>     set GITHUB_TOKEN to avoid GitHub's 60-an-hour limit

import { readFile, writeFile } from "node:fs/promises";

const out = process.argv[2];
if (!out) {
  console.error("usage: node catalog/releases.mjs <out.json>");
  process.exit(2);
}

const catalog = JSON.parse(await readFile(new URL("./catalog.json", import.meta.url), "utf8"));
const headers = { Accept: "application/vnd.github+json", "User-Agent": "craftcraft-releases" };
if (process.env.GITHUB_TOKEN) headers.Authorization = `Bearer ${process.env.GITHUB_TOKEN}`;

async function get(path) {
  const res = await fetch(`https://api.github.com/${path}`, { headers });
  if (res.status === 404) return null;
  if (!res.ok) throw new Error(`GitHub said ${res.status} for ${path}`);
  return res.json();
}

const glob = (g) => new RegExp("^" + g.split("*").map((s) => s.replace(/[.+?^${}()|[\]\\]/g, "\\$&")).join(".*") + "$");

const apps = {};
for (const app of catalog.apps) {
  const repo = await get(`repos/${app.repo}`);
  const gh = await get(`repos/${app.repo}/releases/latest`);
  // Only the files the catalog points at, to keep the index small.
  const patterns = Object.values(app.assets ?? {}).map((a) => glob(typeof a === "string" ? a : a.file));
  const release = gh && {
    tag: gh.tag_name,
    published_at: gh.published_at ?? "",
    html_url: gh.html_url,
    assets: gh.assets
      .filter((a) => patterns.some((re) => re.test(a.name)))
      .map((a) => ({ name: a.name, size: a.size, url: a.browser_download_url, sha256: a.digest?.startsWith("sha256:") ? a.digest.slice(7) : null })),
  };
  const license = repo?.license?.spdx_id;
  apps[app.id] = { stars: repo?.stargazers_count ?? 0, license: license && license !== "NOASSERTION" ? license : null, release };
}

await writeFile(out, JSON.stringify({ generated: new Date().toISOString(), apps }) + "\n");
console.log(`${Object.keys(apps).length} apps → ${out}`);
