-- The usage statistics database (Cloudflare D1). Apply with:
--   npx wrangler d1 execute craft-telemetry --remote --file schema.sql

-- Daily counters with no ids: catalog checks (kind catalog_check), and the website's page views and
-- Download clicks (site_view, site_download_click; launcher, arch and os_version are empty).
CREATE TABLE IF NOT EXISTS counts (
  day        TEXT NOT NULL, -- YYYY-MM-DD, UTC
  kind       TEXT NOT NULL,
  launcher   TEXT NOT NULL,
  os         TEXT NOT NULL,
  arch       TEXT NOT NULL,
  os_version TEXT NOT NULL,
  country    TEXT NOT NULL, -- two letters, from Cloudflare; the IP address isn't kept
  n          INTEGER NOT NULL DEFAULT 0,
  PRIMARY KEY (day, kind, launcher, os, arch, os_version, country)
);

-- Usage statistics from launchers that have them on. Deleted after 400 days.
CREATE TABLE IF NOT EXISTS events (
  day         TEXT NOT NULL,
  install_id  TEXT NOT NULL, -- random, made by the launcher; forgotten there when turned off
  event       TEXT NOT NULL, -- first_run, launch, app_install, app_update, app_install_failed, app_uninstall, app_open
  app         TEXT NOT NULL, -- catalog id, or empty
  app_version TEXT NOT NULL,
  detail      TEXT NOT NULL, -- app_install_failed: the step (download, checksum, installer…)
  launcher    TEXT NOT NULL,
  os          TEXT NOT NULL,
  arch        TEXT NOT NULL,
  os_version  TEXT NOT NULL,
  country     TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS events_day ON events (day);
CREATE INDEX IF NOT EXISTS events_install ON events (install_id);

-- The launcher's GitHub release download counts (running totals), snapshotted daily.
CREATE TABLE IF NOT EXISTS downloads (
  day       TEXT NOT NULL,
  tag       TEXT NOT NULL,
  asset     TEXT NOT NULL,
  downloads INTEGER NOT NULL,
  PRIMARY KEY (day, asset)
);

-- Public totals (/stats.json and the badges), recomputed daily.
CREATE TABLE IF NOT EXISTS published (
  name TEXT PRIMARY KEY,
  json TEXT NOT NULL
);
