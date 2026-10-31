//! Background work: the UI asks for things here and hears back through [`Event`]s, so the window
//! never waits on the network or an installer.

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::{Duration, Instant};

use crate::catalog::{App, Platform};
use crate::debug;
use crate::detect::{self, Detected};
use crate::feed::{self, FeedResult};
use crate::github::{Lookup, Release, Releases};
use crate::http::Http;
use crate::index::{self, Index};
use crate::install::{self, Failure, Stage};
use crate::paths::Paths;
use crate::selfupdate;
use crate::state::Installed;

pub enum Event {
    Catalog(FeedResult),
    Release {
        id: String,
        lookup: Lookup,
    },
    ReleasesChecked,
    Progress {
        id: String,
        stage: Stage,
    },
    Installed {
        id: String,
        result: Result<Installed, Failure>,
    },
    Uninstalled {
        id: String,
        result: Result<(), String>,
    },
    Icon {
        id: String,
        image: Option<eframe::egui::ColorImage>,
    },
    Detected(HashMap<String, Detected>),
    /// A newer release index; it always arrives before the catalog it belongs to.
    Index(Index),
    /// A new version of the launcher, downloaded and ready to restart into, or why not.
    LauncherUpdate(Result<selfupdate::Ready, String>),
}

pub struct Worker {
    tx: Sender<Event>,
    rx: Receiver<Event>,
    ctx: eframe::egui::Context,
    http: Http,
    paths: Paths,
    pub releases: Arc<Releases>,
}

impl Worker {
    pub fn new(ctx: eframe::egui::Context, paths: Paths) -> Self {
        let (tx, rx) = channel();
        let http = Http::new();
        let releases = Arc::new(Releases::new(http.clone(), paths.cache().join("releases.json")));
        Self { tx, rx, ctx, http, paths, releases }
    }

    pub fn events(&self) -> impl Iterator<Item = Event> + '_ {
        self.rx.try_iter()
    }

    fn spawn(&self, name: &str, job: impl FnOnce(&Sender<Event>, &Http, &Paths, &Releases) + Send + 'static) {
        let (tx, ctx, http, paths, releases) = (self.tx.clone(), self.ctx.clone(), self.http.clone(), self.paths.clone(), self.releases.clone());
        let spawned = std::thread::Builder::new().name(name.to_string()).spawn(move || {
            job(&tx, &http, &paths, &releases);
            ctx.request_repaint();
        });
        if let Err(e) = spawned {
            eprintln!("couldn't start {name}: {e}");
        }
    }

    /// Fetches the release index, then the catalog: by the time the catalog arrives, the launcher
    /// knows which apps' releases it still has to ask GitHub about.
    /// When `source` is the usage statistics endpoint and it fails, GitHub Pages is asked instead.
    pub fn refresh_catalog(&self, source: String) {
        self.spawn("catalog", move |tx, http, paths, _| {
            let fetch = |source: &str| {
                if let Some(i) = index::fetch(http, paths, &index::source(source)) {
                    _ = tx.send(Event::Index(i));
                }
                feed::fetch(http, paths, source)
            };
            let mut result = fetch(&source);
            if let FeedResult::Failed(e) = &result
                && crate::telemetry::endpoint().is_some_and(|endpoint| source.starts_with(endpoint))
            {
                debug::log(format_args!("catalog via {source} failed ({e}); trying {}", feed::FEED));
                result = fetch(feed::FEED);
            }
            _ = tx.send(Event::Catalog(result));
        });
    }

    /// Looks up the latest release of each `(app id, repo)`, one at a time to go easy on GitHub.
    pub fn check_releases(&self, apps: Vec<(String, String)>) {
        let ctx = self.ctx.clone();
        self.spawn("releases", move |tx, _, _, releases| {
            for (id, repo) in apps {
                let lookup = releases.latest(&repo);
                _ = tx.send(Event::Release { id, lookup });
                ctx.request_repaint();
            }
            releases.save();
            _ = tx.send(Event::ReleasesChecked);
        });
    }

    pub fn install(&self, app: App, release: Release, platform: Platform, for_everyone: bool) {
        let ctx = self.ctx.clone();
        self.spawn("install", move |tx, http, paths, _| {
            let id = app.id.clone();
            debug::log(format_args!("{id}: installing {} for {}", release.tag, if for_everyone { "everyone" } else { "this user" }));
            let mut last = Instant::now() - Duration::from_secs(1);
            let result = install::install(paths, http, &app, &release, platform, for_everyone, |stage| {
                // Downloads report every chunk; the UI only needs a few updates a second.
                if matches!(stage, Stage::Downloading { .. }) && last.elapsed() < Duration::from_millis(100) {
                    return;
                }
                last = Instant::now();
                _ = tx.send(Event::Progress { id: id.clone(), stage });
                ctx.request_repaint();
            });
            match &result {
                Ok(installed) => debug::log(format_args!("{id}: installed {}", installed.tag)),
                Err(failure) => {
                    debug::log(format_args!("{id}: {failure}{}", failure.log.as_ref().map(|l| format!(" (log: {})", l.display())).unwrap_or_default()))
                }
            }
            _ = tx.send(Event::Installed { id, result });
        });
    }

    /// Downloads the launcher's latest release from `repo`, ready to restart into.
    pub fn prepare_launcher_update(&self, repo: String, offered: String, how: selfupdate::How) {
        self.spawn("selfupdate", move |tx, http, paths, releases| {
            let result = selfupdate::prepare(http, paths, releases, &repo, &offered, &how);
            releases.save();
            if let Err(e) = &result {
                debug::log(format_args!("updating CraftCraft: {e}"));
            }
            _ = tx.send(Event::LauncherUpdate(result));
        });
    }

    /// Fetches the icons for `(app id, url)`, a few at a time.
    pub fn fetch_icons(&self, icons: Vec<(String, String)>) {
        for (n, chunk) in icons.chunks(6).enumerate() {
            let chunk = chunk.to_vec();
            self.spawn(&format!("icons-{n}"), move |tx, http, paths, _| {
                for (id, url) in chunk {
                    let image = crate::icons::load(http, paths, &url).inspect_err(|e| eprintln!("icon for {id}: {e}")).ok();
                    _ = tx.send(Event::Icon { id, image });
                }
            });
        }
    }

    /// Looks for the catalog's apps already on this computer.
    pub fn detect(&self, apps: Vec<App>, platform: Platform) {
        self.spawn("detect", move |tx, _, paths, _| {
            let found = detect::detect_all(&apps, platform, paths);
            if debug::enabled() {
                let mut ids: Vec<_> = found.keys().collect();
                ids.sort();
                for id in ids {
                    let d = &found[id];
                    let exe = d.exe.as_ref().map_or_else(|| "?".into(), |e| e.display().to_string());
                    let scope = if d.for_everyone { "everyone" } else { "this user" };
                    debug::log(format_args!(
                        "found {id} {} for {scope} at {exe}, uninstall: {}",
                        d.version.as_deref().unwrap_or("?"),
                        d.uninstall.as_deref().unwrap_or("-")
                    ));
                }
            }
            _ = tx.send(Event::Detected(found));
        });
    }

    /// Runs an installed program's own uninstaller, then looks again.
    pub fn uninstall_external(&self, id: String, command: String) {
        self.spawn("uninstall", move |tx, _, _, _| {
            let result = crate::system::run_uninstaller(&command);
            _ = tx.send(Event::Uninstalled { id, result });
        });
    }

    pub fn uninstall(&self, id: String, installed: Installed) {
        self.spawn("uninstall", move |tx, _, paths, _| {
            let result = install::uninstall(paths, &id, &installed);
            _ = tx.send(Event::Uninstalled { id, result });
        });
    }
}
