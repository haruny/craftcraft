# Setting up usage statistics

This guide takes you from nothing to a working usage statistics endpoint: a Cloudflare Worker with a
D1 database, a launcher release that uses it, the website counting visits, and the README badges.
Allow about 30 minutes. Everything runs on Cloudflare's free plan.

[README.md](README.md) explains what the Worker does, and [Privacy_Statement.md](../Privacy_Statement.md)
says what is collected. If you change what the Worker stores, change the statement too.

## What you'll end up with

```
launcher ──GET /catalog.json──▶ Worker ──▶ GitHub Pages     (counted, no id)
launcher ──POST /event────────▶ Worker ──▶ D1 "events"     (only if the user left it on)
website  ──POST /site─────────▶ Worker ──▶ D1 "counts"
shields.io ──GET /badge/users─▶ Worker ◀── D1 "published"  (totals, recomputed daily)
daily cron: GitHub download counts ──▶ D1 "downloads"
```

## Before you start

- **A Cloudflare account.** Sign up at [dash.cloudflare.com/sign-up](https://dash.cloudflare.com/sign-up).
  The free plan is enough, and you don't need a domain.
- **Node.js 20 or later.** `node --version` shows yours. Wrangler, Cloudflare's command-line
  tool, runs through `npx`, so there's nothing else to install.
- **Admin access to the `haruny/craftcraft` repository**, to add an Actions variable.

## 1. Sign in to Cloudflare

```sh
cd telemetry
npx wrangler login
```

A browser window opens. Allow access, then come back to the terminal. `npx wrangler whoami`
confirms which account you're signed in to.

## 2. Create the database

```sh
npx wrangler d1 create craftcraft-telemetry
```

It prints a block like this:

```toml
[[d1_databases]]
binding = "DB"
database_name = "craftcraft-telemetry"
database_id = "1f2e3d4c-...."
```

Copy the `database_id` value into [wrangler.toml](wrangler.toml), replacing
`REPLACE_WITH_DATABASE_ID`. The ID isn't a secret, so you can commit it.

Then create the tables:

```sh
npx wrangler d1 execute craftcraft-telemetry --remote --file schema.sql
```

Running it again later is safe: it only adds tables that are missing. Run it again whenever
`schema.sql` changes.

## 3. Deploy the Worker

```sh
npx wrangler deploy
```

The first time, Wrangler asks you to pick a `workers.dev` subdomain for your account (for example
`haruny`). The deploy ends by printing the Worker's address:

```
https://craftcraft-telemetry.haruny.workers.dev
```

This guide calls that **the Worker address**. Run `npx wrangler deploy` again whenever
`src/index.js` or `wrangler.toml` changes.

### Optional: a GitHub token for the daily download snapshot

The daily job makes a few requests to GitHub's API. That works without a token (60 requests an hour
are allowed), but a token avoids hitting the limit if the Worker shares an IP address with busy
neighbours. Create a [fine-grained token](https://github.com/settings/personal-access-tokens/new)
with **Public repositories (read-only)** access and no other permissions, then:

```sh
npx wrangler secret put GITHUB_TOKEN     # paste the token when asked
```

### Optional: your own domain

If you have a domain on Cloudflare, you can serve the Worker from it, for example
`stats.example.com`: in the dashboard, open **Workers & Pages > craftcraft-telemetry > Settings >
Domains & Routes > Add > Custom domain**. Use that address everywhere this guide says "the Worker
address". Once launchers are using an address, keep it working: they have it built in.

## 4. Check that it works

Replace `<worker>` with the Worker address.

```sh
# Passes the catalog on, and counts the check
curl -sA "CraftCraft/0.0.0 (test; test; test; +x)" <worker>/catalog.json | head -c 120; echo

# Accepts an event (204) and refuses a bad one (400)
curl -s -o /dev/null -w "%{http_code}\n" -X POST <worker>/event \
  -d '{"id":"00000000000000000000000000000000","event":"launch","os":"test"}'
curl -s -o /dev/null -w "%{http_code}\n" -X POST <worker>/event -d '{"id":"x"}'

# Totals and a badge
curl -s <worker>/stats.json; echo
curl -s <worker>/badge/users; echo
```

Look at what arrived, then delete the test rows:

```sh
npx wrangler d1 execute craftcraft-telemetry --remote --command "SELECT * FROM counts; SELECT * FROM events"
npx wrangler d1 execute craftcraft-telemetry --remote --command "DELETE FROM counts WHERE os = 'test'; DELETE FROM events WHERE os = 'test'; DELETE FROM published"
```

`npx wrangler tail` streams the Worker's live logs and errors, if something doesn't look right.

## 5. Build it into the launcher

The release workflow builds the Worker address into the launcher from a repository variable. Add it
with the GitHub CLI:

```sh
gh variable set CRAFTCRAFT_TELEMETRY --repo haruny/craftcraft --body "https://craftcraft-telemetry.haruny.workers.dev"
```

Or in the browser: **Settings > Secrets and variables > Actions > Variables > New repository
variable**. Use a variable rather than a secret: the address ends up inside every build anyway.

The next tagged release (or a run of the Release workflow started by hand) includes it. On first
launch, those builds:

- fetch the catalog through the Worker, which counts the check. If the Worker can't be reached,
  they fetch it from GitHub Pages instead.
- show the notice about usage statistics, and send events only once the user has answered it.

Builds without the variable (from source, or before you set it) behave as before: no statistics,
and the catalog comes straight from GitHub Pages.

### Trying it before a release

Serve the Worker on this machine, with a local database, on port 8787:

```sh
cd telemetry
npx wrangler d1 execute craftcraft-telemetry --local --file schema.sql   # once
npx wrangler dev
```

Then, in another terminal at the repository root:

```sh
CRAFTCRAFT_TELEMETRY=http://127.0.0.1:8787 CRAFTCRAFT_HOME=/tmp/craftcraft-test cargo run
```

Debug builds accept an endpoint on this machine. `CRAFTCRAFT_HOME` gives the test run its own data
folder, so it starts from a fresh state and shows the notice. On Windows PowerShell, set the two
variables with `$env:NAME = "..."` first.

## 6. Count website visits

In the craftcraft-site repository, set `TELEMETRY` at the top of `app.js` to the Worker address and
publish the site:

```js
const TELEMETRY = "https://craftcraft-telemetry.haruny.workers.dev";
```

From then on, each page view and each click on Download is counted by day, visitor OS and country.
No cookies are used, and nothing is sent from `localhost` or from browsers with Do Not Track or
Global Privacy Control turned on.

## 7. Add the badges

Once the Worker has run for a day or two, add the badges from
[README.md](README.md#public-stats-and-badges) to the main README, with your Worker address in
place of `<you>`. Until enough installations share usage statistics, the users badge shows "new"
and smaller platforms and apps are left out (any group under 5 installations).

## Day to day

Run each of these from `telemetry/`.

| To | Run |
|---|---|
| Ask a question of the data | `npx wrangler d1 execute craftcraft-telemetry --remote --command "<SQL>"` (examples in [queries.sql](queries.sql)), or the D1 console in the dashboard |
| See live requests and errors | `npx wrangler tail` |
| Recompute the badges now | `npx wrangler d1 execute craftcraft-telemetry --remote --command "DELETE FROM published"` (the next request rebuilds them) |
| Ship a Worker change | `npx wrangler deploy` |
| Apply a schema change | `npx wrangler d1 execute craftcraft-telemetry --remote --file schema.sql` |
| Back up the database | `npx wrangler d1 export craftcraft-telemetry --remote --output backup.sql` |
| Delete someone's usage statistics | see [README.md](README.md#answering-a-deletion-request) |

The daily job runs at 05:30 UTC (`crons` in `wrangler.toml`). The dashboard's **Workers & Pages >
craftcraft-telemetry > Settings > Trigger events** shows its past runs.

## Limits and costs

The free plan allows, per day: 100,000 Worker requests, 100,000 database rows written and
5 million rows read, with 5 GB of storage. Each catalog check, event or site visit is one request and
one row written. A launcher checks at start-up and every 8 hours by default, so the free plan covers
roughly 20,000 to 30,000 daily launchers. Past that, Cloudflare's Workers Paid plan ($5 a month)
raises the limits far beyond what CraftCraft needs.

When a limit is reached, the Worker's requests fail until the next day. Launchers then fall back to
GitHub Pages for the catalog, so nobody loses updates, and only the statistics stop for that day.

## Turning it off

- **Stop collecting from new releases:** delete the `CRAFTCRAFT_TELEMETRY` variable. Releases built
  afterwards send nothing. Launchers already installed keep using the Worker until they update.
- **Stop counting website visits:** set `TELEMETRY` back to `""` in the site's `app.js`.
- **Stop everything now:** `npx wrangler delete` removes the Worker. Launchers that have it built in
  fall back to GitHub Pages for the catalog, and their events fail quietly.
- **Delete the data:** `npx wrangler d1 delete craftcraft-telemetry`. This can't be undone, so
  export a backup first if you might want it.

If you stop collecting, update [Privacy_Statement.md](../Privacy_Statement.md) to match.
