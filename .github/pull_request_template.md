<!-- Adding an app? Fill in the checklist (ADD_APP.md explains each step). Changing the launcher? Delete it and describe the change. -->

## Adding an app

- **App:**
- **Replaces:**
- **Repository:** https://github.com/

- [ ] It's written in Rust and replaces **paid** software (named above)
- [ ] It's open source: a public github.com repository with an open-source license
- [ ] Its builds are files on its latest GitHub release, and each `assets` pattern uses `*` for the version
- [ ] Anything guessed (often the `launch` path): <!-- say what, or "nothing" -->
- [ ] `node catalog/check.mjs --online` passes
- [ ] Tried in the launcher (`CRAFTCRAFT_FEED=catalog/catalog.json cargo run`) on: <!-- Windows / macOS / Linux, or "not tried" -->
- [ ] I'm a maintainer of the app, or its maintainers are fine with it being listed
- [ ] This pull request changes only my app's entry
