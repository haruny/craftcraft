# Usage statistics endpoint

A Cloudflare Worker and D1 database that count how CraftCraft is used. What it collects is set out
in [Privacy_Statement.md](../Privacy_Statement.md): change the two together.

```
SETUP.md         setting it up, step by step
src/index.js     the Worker
schema.sql       the database's tables
queries.sql      questions worth asking it
wrangler.toml.example   the Worker's configuration, without IDs: copy it to wrangler.toml (ignored by git)
```

It does four things:

- **Passes on the catalog.** Release builds fetch `catalog.json` and `releases.json` through it
  instead of straight from GitHub Pages. Each catalog check adds 1 to a daily counter keyed by
  launcher version, OS, processor, OS version and country, read from the `User-Agent` and from
  Cloudflare. If the Worker is down, launchers fall back to GitHub Pages.
- **Records usage statistics** (`POST /event`) from launchers whose users left them on: first run,
  launches, and apps installed, updated, opened, removed and failing to install, each under the
  installation's random id. Rows older than 400 days are deleted.
- **Counts the website's page views and Download clicks** (`POST /site`) by day, visitor OS and
  country. No cookies, and nothing is sent when the browser asks not to be tracked.
- **Snapshots the launcher's GitHub download counts** daily, per release and file.

No table holds an IP address.

## Set it up

**[SETUP.md](SETUP.md) walks through it step by step**: the Cloudflare account, the database,
deploying, checking it works, building it into the launcher and the site, the badges, day-to-day
commands, limits and turning it off. The short version:

```sh
cd telemetry
npx wrangler login
cp wrangler.toml.example wrangler.toml
npx wrangler d1 create craft-telemetry      # paste the database_id into wrangler.toml
npx wrangler d1 execute craft-telemetry --remote --file schema.sql
npx wrangler deploy                              # prints https://craft-telemetry.<you>.workers.dev
# Optional: a token, for GitHub's higher API limit in the daily snapshot
npx wrangler secret put GITHUB_TOKEN
```

Check that it passes the catalog on, then switch the launcher and the site over:

```sh
curl -sA "CraftCraft/0.0.0 (test; test; test; +x)" https://craft-telemetry.<you>.workers.dev/catalog.json | head -c 200
```

1. **Launcher:** in the GitHub repository's Settings > Secrets and variables > Actions > Variables,
   add `CRAFTCRAFT_TELEMETRY` = `https://craft-telemetry.<you>.workers.dev`. The release
   workflow builds it in. Builds without it (such as from source) send nothing and fetch the catalog
   from GitHub Pages as before.
2. **Website:** set `TELEMETRY` at the top of the site's `app.js` to the same address.

To try it locally, run `npx wrangler dev` here and, in another terminal,
`CRAFTCRAFT_TELEMETRY=http://127.0.0.1:8787 CRAFTCRAFT_HOME=/tmp/cc cargo run` (debug builds
accept an endpoint on this machine; `CRAFTCRAFT_HOME` keeps your own launcher's state out of it).

## Read it

```sh
npx wrangler d1 execute craft-telemetry --remote --command "SELECT os, arch, sum(n) FROM counts WHERE kind = 'catalog_check' GROUP BY os, arch"
```

[queries.sql](queries.sql) has more: downloads per platform, versions in use, active installations,
most-used apps and where installs fail. The Cloudflare dashboard's D1 console runs them too.

Reading the numbers:

- A launcher checks the catalog at start-up and every few hours (8 by default), so catalog checks
  measure running launchers rather than people. Opted-in `launch` events and unique `install_id`s
  per day give the closer figure.
- Download counts include the Windows `.zip`, the `.msi`, and the files the install scripts fetch.
- Installs of catalog apps count on their own projects' GitHub releases too. Only the usage
  statistics say which came through CraftCraft.

## Public stats and badges

`/stats.json` holds totals that are safe to show anyone. The daily run recomputes them: monthly
users, app installs in the last 30 days, launcher downloads, the share of each OS and the most-used
apps. Users, OS shares and apps count only installations that share usage statistics. Any OS or app
with fewer than 5 installations is left out, and so is a user total under 5.

`/badge/<name>` serves each total in [shields.io's endpoint format](https://shields.io/badges/endpoint-badge).
Replace `<you>` with your workers.dev subdomain and paste these into the README:

```md
[![Monthly users](https://img.shields.io/endpoint?url=https%3A%2F%2Fcraft-telemetry.<you>.workers.dev%2Fbadge%2Fusers)](Privacy_Statement.md)
[![Runs on](https://img.shields.io/endpoint?url=https%3A%2F%2Fcraft-telemetry.<you>.workers.dev%2Fbadge%2Fplatforms)](Privacy_Statement.md)
[![App installs](https://img.shields.io/endpoint?url=https%3A%2F%2Fcraft-telemetry.<you>.workers.dev%2Fbadge%2Fapp-installs)](Privacy_Statement.md)
[![Most used](https://img.shields.io/endpoint?url=https%3A%2F%2Fcraft-telemetry.<you>.workers.dev%2Fbadge%2Ftop-app)](Privacy_Statement.md)
```

There's also `/badge/downloads`, though shields.io's own
`https://img.shields.io/github/downloads/haruny/craftcraft/total` shows the same number without the
Worker. Badges are cached for an hour, by shields.io and by GitHub. Until the first daily run, the
totals are computed on the first request.

## Answering a deletion request

Someone who wants their usage statistics deleted can send the id from `usage.id` in CraftCraft's
`state.json` (in the data folder, Settings > Data folder > Show):

```sh
npx wrangler d1 execute craft-telemetry --remote --command "DELETE FROM events WHERE install_id = '<id>'"
```
