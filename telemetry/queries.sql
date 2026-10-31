-- Useful questions. Run one with:
--   npx wrangler d1 execute craftcraft-telemetry --remote --command "<query>"

-- Launcher downloads by version and file (latest snapshot).
SELECT tag, asset, downloads FROM downloads WHERE day = (SELECT max(day) FROM downloads) ORDER BY tag DESC, downloads DESC;

-- New downloads per day, by file.
SELECT d.day, d.asset, d.downloads - coalesce(p.downloads, 0) AS new
FROM downloads d LEFT JOIN downloads p ON p.asset = d.asset AND p.day = date(d.day, '-1 day')
ORDER BY d.day DESC, new DESC;

-- Catalog checks per day by OS and processor (every running launcher, opted in or not).
SELECT day, os, arch, sum(n) AS checks FROM counts WHERE kind = 'catalog_check' GROUP BY day, os, arch ORDER BY day DESC, checks DESC;

-- Launcher versions in use over the last 7 days.
SELECT launcher, sum(n) AS checks FROM counts WHERE kind = 'catalog_check' AND day >= date('now', '-7 days') GROUP BY launcher ORDER BY checks DESC;

-- Windows builds and macOS versions in use over the last 30 days.
SELECT os, os_version, sum(n) AS checks FROM counts WHERE kind = 'catalog_check' AND day >= date('now', '-30 days') GROUP BY os, os_version ORDER BY checks DESC;

-- Website views and Download clicks per day, by the visitor's OS.
SELECT day, kind, os, sum(n) AS n FROM counts WHERE kind LIKE 'site_%' GROUP BY day, kind, os ORDER BY day DESC;

-- Opted-in installations active per day and per month (unique ids).
SELECT day, count(DISTINCT install_id) AS active FROM events GROUP BY day ORDER BY day DESC LIMIT 30;
SELECT substr(day, 1, 7) AS month, count(DISTINCT install_id) AS active FROM events GROUP BY month ORDER BY month DESC;

-- New installations per day (opted in).
SELECT day, count(*) AS new FROM events WHERE event = 'first_run' GROUP BY day ORDER BY day DESC;

-- Most installed and most opened apps over the last 30 days.
SELECT app, sum(event = 'app_install') AS installs, sum(event = 'app_update') AS updates, sum(event = 'app_open') AS opens,
       sum(event = 'app_uninstall') AS removals, count(DISTINCT install_id) AS installations
FROM events WHERE app != '' AND day >= date('now', '-30 days') GROUP BY app ORDER BY installations DESC;

-- Where installs fail, by app, step and platform.
SELECT app, detail AS step, os, arch, count(*) AS failures FROM events WHERE event = 'app_install_failed' GROUP BY app, step, os, arch ORDER BY failures DESC;

-- Opted-in installations per OS over the last 7 days (compare with catalog checks above).
SELECT os, count(DISTINCT install_id) AS opted_in FROM events WHERE day >= date('now', '-7 days') GROUP BY os;
