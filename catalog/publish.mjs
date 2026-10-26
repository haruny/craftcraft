#!/usr/bin/env node
// Writes the catalog as it's published: catalog.json with featured.json's list merged in as
// "featured" and any left-out `added` and `color` filled in, plus the schema and the landing page.
//
// featured.json is kept apart so the maintainers' picks never get mixed into contributors'
// pull requests to catalog.json (CODEOWNERS requires a maintainer's review for it).
//
//   node catalog/publish.mjs <dir>

import { execFileSync } from "node:child_process";
import { copyFile, readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const dir = process.argv[2];
if (!dir) {
  console.error("usage: node catalog/publish.mjs <dir>");
  process.exit(2);
}

const here = (f) => new URL(`./${f}`, import.meta.url);
const catalog = JSON.parse(await readFile(here("catalog.json"), "utf8"));
const { featured = [] } = JSON.parse(await readFile(here("featured.json"), "utf8"));
const ids = new Set(catalog.apps.map((a) => a.id));
// check.mjs rejects unknown ids; skipping them here too means a removed app can't break publishing.
catalog.featured = featured.filter((id) => ids.has(id));

// Contributors can leave these out; fill them in so every app has them once published.
const TINTS = ["#e8662e", "#2563eb", "#16a34a", "#9333ea", "#db2777", "#0891b2", "#ca8a04", "#4f46e5", "#dc2626", "#0d9488"];
const today = new Date().toISOString().slice(0, 10);
for (const app of catalog.apps) {
  app.added ??= firstSeen(app.id) ?? today;
  app.color ??= TINTS[[...app.id].reduce((h, c) => (h * 31 + c.charCodeAt(0)) >>> 0, 7) % TINTS.length];
}

/** The day `id` first appeared in catalog.json's history (needs the full history checked out). */
function firstSeen(id) {
  try {
    const out = execFileSync("git", ["log", "--reverse", "--format=%as", "-S", `"id": "${id}"`, "--", "catalog/catalog.json"], {
      cwd: fileURLToPath(new URL("..", import.meta.url)),
      encoding: "utf8",
      stdio: ["ignore", "pipe", "ignore"],
    });
    return out.split("\n")[0] || null;
  } catch {
    return null;
  }
}

await writeFile(join(dir, "catalog.json"), JSON.stringify(catalog, null, 2) + "\n");
for (const f of ["catalog.schema.json", "index.html"]) await copyFile(here(f), join(dir, f));
console.log(`catalog.json: ${catalog.apps.length} apps, ${catalog.featured.length} featured`);
