<!--
Keep the section that matches your pull request and delete the other one.
- Adding an app to the catalog: ADD_APP.md explains each step. Title it "Add <name>".
- Changing the launcher, the website, the scripts or the docs: see "Changing the launcher" in CONTRIBUTING.md.
-->

## Adding an app

- **App:**
- **Replaces:** <!-- the paid products, best known first -->
- **Repository:** https://github.com/

- [ ] It's written in Rust and replaces **paid** software (named above)
- [ ] It's open source: a public github.com repository with an open-source license
- [ ] Its builds are files on its latest GitHub release, and each `assets` pattern uses `*` for the version
- [ ] Anything guessed (often the `launch` path): <!-- say what, or "nothing" -->
- [ ] `node catalog/check.mjs --online` passes
- [ ] Tried in the launcher (`CRAFTCRAFT_FEED=catalog/catalog.json cargo run`) on: <!-- Windows / macOS / Linux, or "not tried" -->
- [ ] I'm a maintainer of the app, or its maintainers are fine with it being listed
- [ ] This pull request changes only my app's entry in `catalog/catalog.json`
- [ ] It contains no unverified AI output: I've checked everything an AI tool wrote, this description included

## Changing CraftCraft

**What changes, and why:**
<!-- Link the issue it fixes, if there is one: "Fixes #123" -->

**How I tested it:** <!-- what you did, and on which of Windows / macOS / Linux -->

- [ ] `cargo fmt`, `cargo clippy --all-targets -- -D warnings` and `cargo test` pass
- [ ] New or changed UI text is in `locales/en.json` <!-- other languages too, if you can -->
- [ ] Screenshots below, if what you see in the launcher or on the website changes
- [ ] The docs (README, CONTRIBUTING, ADD_APP) still match, or I've updated them
- [ ] No changes to `catalog/featured.json` or other apps' catalog entries
- [ ] It contains no unverified AI output: I've checked everything an AI tool wrote, this description included
