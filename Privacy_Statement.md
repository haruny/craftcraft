# CraftCraft Privacy Statement

_Effective 31 October 2026_

CraftCraft is a launcher that installs, updates and opens open-source apps. This statement explains
what information CraftCraft and its website collect, why, and how to turn it off. It covers the
CraftCraft launcher as released on [GitHub](https://github.com/haruny/craftcraft/releases) and
the CraftCraft website. It doesn't cover the apps you install with CraftCraft, which have their own
policies.

**In short:** we count which kinds of computers CraftCraft runs on, and, unless you turn it off,
which apps people install and open with it. We never collect your name, email address, files,
or what you do inside the apps. We don't store IP addresses, and we never sell or share any of it.

## What the launcher sends

### 1. Counts with every catalog check (no ID)

CraftCraft checks its app catalog when it starts and every few hours after that (8 by default; you
can change this in Settings). Release builds fetch the catalog through our own server, which adds 1
to a daily count for the combination of:

- the CraftCraft version
- the operating system, its version and the processor type (for example: Windows, 10.0.26300, x86_64)
- the country, as worked out by our hosting provider from your IP address

These details come from the `User-Agent` that CraftCraft sends with every web request, including
requests to GitHub. Nothing ties one count to another, so these counts can't identify you or your
computer, or tell how many checks came from the same computer.

### 2. Usage statistics (on by default, and you can turn them off)

These help us see which apps people actually use, which installs fail, and how many people use
CraftCraft. Nothing is sent until you have seen the notice about them in the launcher. Each event
includes:

- a random installation ID, created when you agree. It's unrelated to your computer, your
  accounts or you, and it's deleted from your computer when you turn usage statistics off.
- what happened: CraftCraft's first run, a launch, or an app installed, updated, opened or
  uninstalled. For a failed install, the step that failed (for example "download" or "installer"),
  never the error message, which could contain file paths.
- for app events, the app's catalog name (such as `photocraft`) and its version
- the CraftCraft version, operating system and version, processor type, and country (as above)
- the date (not the time)

### What we never collect

Your name, email, user name, computer name, IP address, files, documents or file paths. We don't
collect what you do inside the apps, your other installed software (beyond the catalog apps
CraftCraft manages), hardware serial numbers or other device identifiers. We don't use cookies or
advertising IDs.

## The website

The CraftCraft website counts page views and clicks on its Download button by day, by the
visitor's operating system (from the browser's `User-Agent`) and by country. It doesn't use cookies
or any visitor ID, and it stores no IP addresses. It sends nothing if your browser has Do Not Track
or Global Privacy Control turned on.

The website loads its fonts from Google Fonts, so Google receives your IP address as part of that
request ([Google's privacy policy](https://policies.google.com/privacy)).

## Turning it off

- **Usage statistics:** choose **Turn off** on the notice, or clear **Settings > Privacy > Share
  anonymous usage statistics**. CraftCraft then deletes its installation ID; if you turn usage
  statistics back on later, it creates a new one.
- **Everything, including the counts:** set the environment variable `DO_NOT_TRACK=1` or
  `CRAFTCRAFT_NO_TELEMETRY=1`. CraftCraft then sends no statistics and fetches the catalog
  directly from GitHub Pages instead of through our server.
- **Fewer connections:** turn off automatic update checks in Settings. CraftCraft then connects
  only when you click Check for updates, install something, or open its links.
- **Builds from source** never send statistics.

## Why we collect it

We use this information to decide which platforms and app versions to support, to find and fix
failing installs, and to see whether CraftCraft is useful to people. In legal terms, we rely on our
legitimate interest in maintaining and improving CraftCraft. We keep that interest small by
collecting only what's described here, without IP addresses or personal identifiers, and by letting
you turn it off at any time.

## Where it's kept, and for how long

The statistics are stored in a database run by our hosting provider,
[Cloudflare](https://www.cloudflare.com/privacypolicy/), which may process them in any country where
it operates. Cloudflare sees your IP address while it handles each request, as any web host does,
but we don't keep it.

- Usage statistics events are deleted after 400 days.
- The daily counts, which can't identify anyone, are kept indefinitely.

Only the CraftCraft maintainers can access the data. We publish totals that can't identify anyone,
such as monthly users, the share of each operating system and the most-used apps, on badges in our
README. These leave out any group of fewer than 5 installations. We don't sell the data or share it
with anyone else, and we don't use it for advertising.

## Other services CraftCraft connects to

To work at all, CraftCraft connects to GitHub (`github.com`, `api.github.com`, and
`haruny.github.io`, which serves the catalog and install scripts), and to the web addresses where the
catalog's app icons are kept. These services receive your IP address and CraftCraft's `User-Agent`,
like any other web request, and their own privacy policies apply. Downloads from GitHub also add to
the public download counts on GitHub releases.

## Your rights

Depending on where you live, you may have the right to see, correct or delete data about you, or to
object to how it's processed. The counts can't be linked to you. For your usage statistics, find
your installation ID in CraftCraft's `state.json` (under `"usage"` → `"id"`; Settings > Data folder
> Show opens the folder). Contact us with that ID and we'll delete everything recorded under it.
Turning usage statistics off stops anything new from being sent.

## Changes

If we change what CraftCraft collects, we'll update this statement and its date, and record the
change in the repository's history. Before collecting any new kind of information, we'll show you a
new notice in the launcher.

## Contact

Open an issue at [github.com/haruny/craftcraft/issues](https://github.com/haruny/craftcraft/issues).
For deletion requests, don't post your installation ID publicly. Ask in an issue and we'll arrange a
private way for you to send it.
