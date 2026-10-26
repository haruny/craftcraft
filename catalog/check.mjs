#!/usr/bin/env node
// Checks catalog.json before it is published.
//
//   node catalog/check.mjs                    structure and cross-references
//   node catalog/check.mjs --online           also checks each app is open source on GitHub, and every asset pattern against its
//                                             latest release (set GITHUB_TOKEN in CI)

import { readFile } from "node:fs/promises";

const PLATFORMS = ["windows-x86_64", "windows-aarch64", "macos-x86_64", "macos-aarch64", "linux-x86_64", "linux-aarch64"];
const KINDS = ["gui", "tui", "cli"];
const METHODS = ["archive", "installer", "portable"];
const ID = /^[a-z0-9][a-z0-9-]{0,39}$/;
const REPO = /^[A-Za-z0-9-]+\/[A-Za-z0-9._-]+$/;
const DATE = /^\d{4}-\d{2}-\d{2}$/;
const COLOR = /^#[0-9a-fA-F]{6}$/;
const BIN = /^[A-Za-z0-9._-]+$/;

const online = process.argv.includes("--online");
const path = new URL("./catalog.json", import.meta.url);
const errors = [];
const warnings = [];
const err = (where, msg) => errors.push(`${where}: ${msg}`);

let catalog;
try {
  catalog = JSON.parse(await readFile(path, "utf8"));
} catch (e) {
  console.error(`catalog.json: ${e.message}`);
  process.exit(1);
}

if (catalog.schema !== 1) err("schema", "must be 1");
for (const key of ["name", "updated"]) if (typeof catalog[key] !== "string") err(key, "missing");
if (catalog.updated && !DATE.test(catalog.updated)) err("updated", "must be YYYY-MM-DD");
if (catalog.refresh_hours !== undefined && !(Number.isInteger(catalog.refresh_hours) && catalog.refresh_hours >= 1 && catalog.refresh_hours <= 168))
  err("refresh_hours", "must be a whole number from 1 to 168");
if (!catalog.launcher?.version) err("launcher.version", "missing");
if (!REPO.test(catalog.launcher?.repo ?? "")) err("launcher.repo", "must be owner/name");

const categories = new Set();
for (const [i, c] of (catalog.categories ?? []).entries()) {
  if (!ID.test(c.id ?? "")) err(`categories[${i}].id`, "invalid id");
  if (categories.has(c.id)) err(`categories[${i}].id`, `duplicate "${c.id}"`);
  categories.add(c.id);
}
if (categories.size === 0) err("categories", "needs at least one");

const notices = new Set();
for (const [i, n] of (catalog.notices ?? []).entries()) {
  if (!ID.test(n.id ?? "")) err(`notices[${i}].id`, "invalid id");
  if (notices.has(n.id)) err(`notices[${i}].id`, `duplicate "${n.id}"`);
  notices.add(n.id);
  if (!n.title) err(`notices[${i}].title`, "missing");
  if (n.url && !n.url.startsWith("https://")) err(`notices[${i}].url`, "must be https");
}

const ids = new Set();
for (const [i, app] of (catalog.apps ?? []).entries()) {
  const at = `apps[${i}] (${app.id ?? "?"})`;
  if (!ID.test(app.id ?? "")) err(at, "invalid id");
  if (ids.has(app.id)) err(at, "duplicate id");
  ids.add(app.id);
  for (const key of ["name", "tagline"]) if (!app[key]) err(at, `missing ${key}`);
  if (!Array.isArray(app.replaces) || app.replaces.length === 0) err(at, "replaces must list at least one program");
  if (!categories.has(app.category)) err(at, `unknown category "${app.category}"`);
  if (app.kind !== undefined && !KINDS.includes(app.kind)) err(at, `kind must be one of ${KINDS.join(", ")}`);
  if (!REPO.test(app.repo ?? "")) err(at, "repo must be owner/name");
  if (app.color !== undefined && !COLOR.test(app.color)) err(at, "color must be #rrggbb");
  if (app.added !== undefined && !DATE.test(app.added)) err(at, "added must be YYYY-MM-DD");
  if (app.homepage && !app.homepage.startsWith("https://")) err(at, "homepage must be https");
  if (app.icon && !app.icon.startsWith("https://")) err(at, "icon must be https");
  if (app.icon && !/\.(png|jpe?g)$|^https:\/\/avatars\.githubusercontent\.com\//i.test(app.icon)) err(at, "icon must be a PNG or JPEG");
  for (const b of app.bin ?? []) if (!BIN.test(b)) err(at, `bad bin name "${b}"`);

  const entries = Object.entries(app.assets ?? {});
  if (entries.length === 0) err(at, "needs at least one asset");
  for (const [platform, asset] of entries) {
    if (!PLATFORMS.includes(platform)) err(at, `unknown platform "${platform}"`);
    const { file, method } = typeof asset === "string" ? { file: asset } : asset;
    if (!file) err(`${at}.${platform}`, "empty pattern");
    if (method && !METHODS.includes(method)) err(`${at}.${platform}`, `method must be one of ${METHODS.join(", ")}`);
    const m = method ?? guessMethod(file ?? "");
    if (m === null) err(`${at}.${platform}`, `can't tell how to install "${file}"; add a method`);
    if (m === "archive" && !(app.bin?.length)) err(`${at}.${platform}`, "archives need a bin entry");
    const os = platform.split("-")[0];
    if (m === "installer" && !app.launch?.[os]) warnings.push(`${at}.${platform}: installer without launch.${os}, so the launcher can't open or detect it`);
  }
}

// Featuring is the maintainers' call: it lives in featured.json and is merged in when publishing.
if ("featured" in catalog) err("featured", "goes in featured.json, not catalog.json");
try {
  const { featured } = JSON.parse(await readFile(new URL("./featured.json", import.meta.url), "utf8"));
  if (!Array.isArray(featured)) err("featured.json", "needs a \"featured\" list of app ids");
  else for (const id of featured) if (!catalog.apps?.some((a) => a.id === id)) err("featured.json", `no app "${id}" in catalog.json`);
} catch (e) {
  err("featured.json", e.message);
}

if (online) await checkReleases();

for (const w of warnings) console.warn(`warning  ${w}`);
for (const e of errors) console.error(`error    ${e}`);
console.log(`${catalog.apps?.length ?? 0} apps, ${errors.length} errors, ${warnings.length} warnings`);
process.exit(errors.length ? 1 : 0);

function guessMethod(file) {
  const f = file.toLowerCase();
  if ([".zip", ".tar.gz", ".tgz", ".tar.xz", ".txz"].some((e) => f.endsWith(e))) return "archive";
  if ([".msi", ".exe", ".dmg", ".pkg", ".deb", ".rpm"].some((e) => f.endsWith(e))) return "installer";
  if (f.endsWith(".appimage")) return "portable";
  return null;
}

function globToRegExp(glob) {
  return new RegExp("^" + glob.split("*").map((s) => s.replace(/[.+?^${}()|[\]\\]/g, "\\$&")).join(".*") + "$");
}

async function checkReleases() {
  const headers = { Accept: "application/vnd.github+json", "User-Agent": "craftcraft-catalog-check" };
  if (process.env.GITHUB_TOKEN) headers.Authorization = `Bearer ${process.env.GITHUB_TOKEN}`;
  for (const app of catalog.apps ?? []) {
    if (app.icon) {
      const icon = await fetch(app.icon, { method: "HEAD" });
      if (!icon.ok) err(`${app.id}.icon`, `HTTP ${icon.status}`);
      else if (!/^image\/(png|jpeg)/.test(icon.headers.get("content-type") ?? "")) err(`${app.id}.icon`, `served as ${icon.headers.get("content-type")}`);
    }
    // Rule 2: open source, with its source on github.com. A private or missing repo is a 404.
    const repo = await fetch(`https://api.github.com/repos/${app.repo}`, { headers });
    if (!repo.ok) {
      err(app.id, `GitHub said ${repo.status} for ${app.repo}: the source must be in a public GitHub repository`);
      continue;
    }
    const info = await repo.json();
    if (info.full_name.toLowerCase() !== app.repo.toLowerCase()) err(`${app.id}.repo`, `${app.repo} has moved to ${info.full_name}`);
    if (!info.license) err(`${app.id}.repo`, `${app.repo} has no license file, so it isn't open source`);
    else if (info.license.spdx_id === "NOASSERTION") warnings.push(`${app.id}.repo: GitHub doesn't recognise ${app.repo}'s license; check it's an open-source one`);
    if (info.archived) warnings.push(`${app.id}.repo: ${app.repo} is archived`);

    const res = await fetch(`https://api.github.com/repos/${app.repo}/releases/latest`, { headers });
    if (!res.ok) {
      err(app.id, `GitHub said ${res.status} for ${app.repo}`);
      continue;
    }
    const release = await res.json();
    const names = release.assets.map((a) => a.name);
    for (const [platform, asset] of Object.entries(app.assets ?? {})) {
      const file = typeof asset === "string" ? asset : asset.file;
      const re = globToRegExp(file);
      const hits = names.filter((n) => re.test(n));
      if (hits.length === 0) err(`${app.id}.${platform}`, `"${file}" matches nothing in ${release.tag_name}`);
      else if (hits.length > 1) err(`${app.id}.${platform}`, `"${file}" matches ${hits.length} files in ${release.tag_name}: ${hits.join(", ")}`);
      else if (!release.assets.find((a) => a.name === hits[0]).digest)
        warnings.push(`${app.id}.${platform}: ${hits[0]} has no SHA-256 from GitHub, so it can't be verified`);
    }
  }
}
