//! The window: a sidebar of sections, and a grid of app cards.

use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant};

use eframe::egui::text::LayoutJob;
use eframe::egui::{self, Align, Align2, Color32, CornerRadius, FontId, Frame, Layout, Margin, RichText, Sense, Stroke, TextFormat, TextureHandle, Vec2};

use crate::catalog::{App, Catalog, Kind, Method, Platform};
use crate::detect::Detected;
use crate::feed::{self, FeedResult};
use crate::github::{Lookup, Release};
use crate::index::{self, Index};
use crate::install::{self, Failure, Stage, Step};
use crate::paths::Paths;
use crate::state::{Installed, MAX_CHECK_HOURS, Sort, State};
use crate::theme::{self, Palette};
use crate::worker::{Event, Worker};
use crate::{debug, now, system, telemetry, version};

const SIDEBAR_WIDTH: f32 = 236.0;
const CARD_MIN_WIDTH: f32 = 320.0;
const CARD_HEIGHT: f32 = 208.0;
const GAP: f32 = 14.0;
const PRIVACY_STATEMENT: &str = "https://github.com/haruny/craftcraft/blob/main/Privacy_Statement.md";
/// After a failed catalog check, wait this long before trying again.
const RETRY_AFTER: Duration = Duration::from_secs(15 * 60);

#[derive(Clone, PartialEq, Eq)]
enum Filter {
    All,
    Installed,
    Updates,
    Category(String),
}

enum Busy {
    Working(Stage),
    Removing,
}

enum Icon {
    Loading,
    Ready(TextureHandle),
    Missing,
}

/// Where an app stands on this computer. `external` means it was found already installed (by its
/// own installer, winget, a package manager…) rather than installed into the launcher's folder.
enum Status<'a> {
    NoBuild,
    /// The latest release isn't known yet (or couldn't be found).
    Unknown,
    Available(&'a Release),
    /// Up to date, or there's no way to tell.
    Installed {
        version: Option<String>,
        latest: Option<&'a Release>,
        external: bool,
    },
    Update {
        from: Option<String>,
        to: &'a Release,
        external: bool,
    },
}

pub struct Launcher {
    paths: Paths,
    worker: Worker,
    state: State,
    platform: Option<Platform>,
    catalog: Option<Catalog>,
    catalog_error: Option<String>,
    releases: HashMap<String, Lookup>,
    /// Catalog apps found already installed on this computer.
    detected: HashMap<String, Detected>,
    /// Every app's latest release and stars, from releases.json.
    index: Index,
    busy: HashMap<String, Busy>,
    /// What went wrong last for an app, shown on its card until it's tried again.
    app_errors: HashMap<String, Failure>,
    /// Apps waiting their turn after Update all. They go one at a time: Windows can't run two
    /// installers at once.
    update_queue: VecDeque<String>,
    icons: HashMap<String, Icon>,
    checking_catalog: bool,
    checking_releases: bool,
    last_attempt: Option<Instant>,
    search: String,
    filter: Filter,
    show_settings: bool,
    /// How tall the sidebar's bottom section was last frame, so the sections above can stop short of it.
    sidebar_bottom: f32,
    message: Option<(String, Instant)>,
    #[cfg(tray)]
    tray: Option<crate::tray::Tray>,
    /// Started at logon: hide the window as soon as it's up.
    #[cfg(tray)]
    hide_at_start: bool,
    /// Quit was chosen in the tray, so closing the window really closes it.
    #[cfg(tray)]
    quitting: bool,
    /// Whether the system starts the launcher at logon, as last read from it.
    #[cfg(tray)]
    start_on_logon: bool,
    /// The window is hidden in the menu bar.
    #[cfg(target_os = "macos")]
    hidden: bool,
    /// Whether the app was active last time we looked, while hidden.
    #[cfg(target_os = "macos")]
    was_active: bool,
}

impl Launcher {
    pub fn new(cc: &eframe::CreationContext<'_>, paths: Paths) -> Self {
        theme::install(&cc.egui_ctx);
        let state = State::load(&paths);
        debug::init(&paths.logs(), state.settings.debug);
        debug::log(format_args!("started, data folder {}", paths.root.display()));
        install::tidy(&paths);
        let worker = Worker::new(cc.egui_ctx.clone(), paths.clone());
        let release_index = index::cached(&paths);
        let catalog = feed::cached(&paths);
        let mut releases = HashMap::new();
        if let Some(c) = &catalog {
            for app in &c.apps {
                if let Some(r) = worker.releases.cached(&app.repo) {
                    releases.insert(app.id.clone(), Lookup::Found(r));
                }
            }
        }
        let mut launcher = Self {
            paths,
            worker,
            state,
            platform: Platform::current(),
            catalog,
            catalog_error: None,
            releases,
            detected: HashMap::new(),
            index: release_index,
            busy: HashMap::new(),
            app_errors: HashMap::new(),
            update_queue: VecDeque::new(),
            icons: HashMap::new(),
            checking_catalog: false,
            checking_releases: false,
            last_attempt: None,
            search: String::new(),
            filter: Filter::All,
            show_settings: false,
            sidebar_bottom: 120.0,
            message: None,
            #[cfg(tray)]
            tray: None,
            #[cfg(tray)]
            hide_at_start: false,
            #[cfg(tray)]
            quitting: false,
            #[cfg(tray)]
            start_on_logon: system::starts_on_logon(),
            #[cfg(target_os = "macos")]
            hidden: false,
            #[cfg(target_os = "macos")]
            was_active: false,
        };
        #[cfg(windows)]
        system::add_to_start_menu();
        launcher.apply_index();
        launcher.fetch_icons();
        launcher.detect();
        telemetry::record(&launcher.state, "launch", None, None);
        // Always look at start-up: with an ETag an unchanged catalog costs next to nothing.
        if launcher.state.settings.auto_check {
            launcher.check_now();
        }
        launcher
    }

    /// Adds the tray icon; `background` starts the launcher hidden in it. Without the icon there'd
    /// be no way back to a hidden window, so then the window stays up and closes as usual.
    #[cfg(tray)]
    pub fn with_tray(mut self, ctx: &egui::Context, background: bool, #[cfg(windows)] instance: crate::tray::Instance) -> Self {
        match crate::tray::Tray::new(ctx) {
            Ok(tray) => {
                #[cfg(windows)]
                tray.listen(instance, ctx);
                self.tray = Some(tray);
                self.hide_at_start = background;
            }
            Err(e) => eprintln!("tray icon: {e}"),
        }
        self
    }

    /// Shows the window, or hides it in the tray.
    #[cfg(tray)]
    fn show_window(&mut self, ctx: &egui::Context, show: bool) {
        use egui::ViewportCommand;
        #[cfg(target_os = "macos")]
        {
            crate::tray::show_in_dock(show);
            self.hidden = !show;
            self.was_active = crate::tray::is_active();
        }
        ctx.send_viewport_cmd(ViewportCommand::Visible(show));
        if show {
            ctx.send_viewport_cmd(ViewportCommand::Minimized(false));
            ctx.send_viewport_cmd(ViewportCommand::Focus);
        }
    }

    /// Answers the tray icon, and closes the window to it when that's the setting.
    #[cfg(tray)]
    fn tray(&mut self, ctx: &egui::Context) {
        use egui::ViewportCommand;
        let Some(tray) = &self.tray else { return };
        let actions: Vec<_> = tray.actions().collect();
        for action in actions {
            match action {
                crate::tray::Action::Show => self.show_window(ctx, true),
                crate::tray::Action::Check => self.check_now(),
                crate::tray::Action::Quit => {
                    self.quitting = true;
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }
            }
        }
        if std::mem::take(&mut self.hide_at_start) {
            self.show_window(ctx, false);
        }
        if ctx.input(|i| i.viewport().close_requested()) && self.state.settings.close_to_tray && !self.quitting {
            ctx.send_viewport_cmd(ViewportCommand::CancelClose);
            self.show_window(ctx, false);
        }
        // Opening the app again from Finder or Launchpad only makes it active: then show the window.
        #[cfg(target_os = "macos")]
        if self.hidden {
            let active = crate::tray::is_active();
            if active && !self.was_active {
                self.show_window(ctx, true);
            } else {
                self.was_active = active;
                ctx.request_repaint_after(Duration::from_secs(1));
            }
        }
        let updates = self.update_count();
        let tooltip = match updates {
            0 => "CraftCraft".to_string(),
            1 => "CraftCraft: 1 update".to_string(),
            n => format!("CraftCraft: {n} updates"),
        };
        if let Some(tray) = &mut self.tray {
            tray.set_tooltip(tooltip);
        }
    }

    fn check_now(&mut self) {
        if self.checking_catalog {
            return;
        }
        self.checking_catalog = true;
        self.last_attempt = Some(Instant::now());
        self.worker.refresh_catalog(feed::source());
    }

    /// Looks for the catalog's apps already on this computer.
    fn detect(&self) {
        if let (Some(catalog), Some(platform)) = (&self.catalog, self.platform) {
            self.worker.detect(catalog.apps.clone(), platform);
        }
    }

    /// A copy the launcher unpacked into its own folder. (What a system installer installed is
    /// found by detection instead, so it reflects what's really there.)
    fn managed(&self, id: &str) -> Option<&Installed> {
        self.state.installed.get(id).filter(|i| i.method != Method::Installer)
    }

    fn check_releases(&mut self) {
        let (Some(catalog), Some(platform)) = (&self.catalog, self.platform) else { return };
        if self.checking_releases {
            return;
        }
        // The index covers the catalog's apps; GitHub is asked only about any it doesn't (yet).
        let apps: Vec<_> = catalog
            .apps
            .iter()
            .filter(|a| a.asset_for(platform).is_some() && !self.index.contains_key(&a.id))
            .map(|a| (a.id.clone(), a.repo.clone()))
            .collect();
        if apps.is_empty() {
            return;
        }
        self.checking_releases = true;
        self.worker.check_releases(apps);
    }

    /// Takes each app's latest release from the index.
    fn apply_index(&mut self) {
        let Some(catalog) = &self.catalog else { return };
        for app in &catalog.apps {
            let lookup = match self.index.get(&app.id) {
                Some(index::Entry { release: Some(r), .. }) => Lookup::Found(r.clone().for_repo(&app.repo)),
                Some(_) => Lookup::NoRelease,
                None => continue,
            };
            self.releases.insert(app.id.clone(), lookup);
        }
    }

    /// Starts fetching icons the catalog names that aren't loaded or loading yet.
    fn fetch_icons(&mut self) {
        let Some(catalog) = &self.catalog else { return };
        let wanted: Vec<(String, String)> =
            catalog.apps.iter().filter(|a| !self.icons.contains_key(&a.id)).filter_map(|a| a.icon.clone().map(|url| (a.id.clone(), url))).collect();
        for (id, _) in &wanted {
            self.icons.insert(id.clone(), Icon::Loading);
        }
        if !wanted.is_empty() {
            self.worker.fetch_icons(wanted);
        }
    }

    /// Checks again when the chosen number of hours have passed, or a while after a failure.
    fn schedule(&mut self) {
        if !self.state.settings.auto_check || self.checking_catalog {
            return;
        }
        let hours = self.state.settings.check_hours.clamp(1, MAX_CHECK_HOURS);
        let due = now() - self.state.last_check >= i64::from(hours) * 3600;
        let waited = self.last_attempt.is_none_or(|t| t.elapsed() >= RETRY_AFTER);
        if due && waited {
            self.check_now();
        }
    }

    fn handle(&mut self, ctx: &egui::Context, event: Event) {
        match event {
            Event::Catalog(result) => {
                self.checking_catalog = false;
                match result {
                    FeedResult::Updated(parsed) => {
                        let parsed = *parsed;
                        for p in &parsed.problems {
                            eprintln!("catalog: {p}");
                        }
                        if !self.state.seen_first_catalog {
                            self.state.mark_all_seen(&parsed.catalog);
                        }
                        // An app whose icon changed gets it fetched again.
                        if let Some(old) = &self.catalog {
                            for app in &parsed.catalog.apps {
                                if old.app(&app.id).is_none_or(|o| o.icon != app.icon) {
                                    self.icons.remove(&app.id);
                                }
                            }
                        }
                        self.catalog = Some(parsed.catalog);
                        self.catalog_error = None;
                        self.state.last_check = now();
                        self.state.save(&self.paths);
                        self.apply_index();
                        self.fetch_icons();
                        self.detect();
                        self.check_releases();
                    }
                    FeedResult::Unchanged => {
                        self.catalog_error = None;
                        self.state.last_check = now();
                        self.state.save(&self.paths);
                        self.detect();
                        self.check_releases();
                    }
                    FeedResult::Failed(e) => {
                        debug::log(format_args!("catalog check failed: {e}"));
                        self.catalog_error = Some(e);
                    }
                }
            }
            Event::Release { id, lookup } => {
                // Keep a known release rather than replace it with a failure.
                if matches!(lookup, Lookup::Found(_)) || !matches!(self.releases.get(&id), Some(Lookup::Found(_))) {
                    self.releases.insert(id, lookup);
                }
            }
            Event::ReleasesChecked => self.checking_releases = false,
            Event::Progress { id, stage } => {
                self.busy.insert(id, Busy::Working(stage));
            }
            Event::Installed { id, result } => {
                self.busy.remove(&id);
                let update = self.state.installed.contains_key(&id) || self.detected.contains_key(&id);
                match result {
                    Ok(installed) => {
                        telemetry::record(&self.state, if update { "app_update" } else { "app_install" }, Some((&id, &installed.tag)), None);
                        let name = self.app_name(&id);
                        let note = if installed.verified { "" } else { " (unverified: no published checksum)" };
                        self.flash(format!("{name} {} is installed{note}", version::display(&installed.tag)));
                        self.app_errors.remove(&id);
                        // A system install is tracked by detection, which sees what's really there.
                        if installed.method == Method::Installer {
                            self.state.installed.remove(&id);
                        } else {
                            self.state.installed.insert(id, installed);
                        }
                        self.state.save(&self.paths);
                    }
                    Err(e) => {
                        // Only the step: the message can hold paths with the user's name in them.
                        telemetry::record(&self.state, "app_install_failed", Some((&id, "")), Some(&format!("{:?}", e.step).to_lowercase()));
                        self.app_errors.insert(id, e);
                    }
                }
                self.detect();
                self.next_update();
            }
            Event::Uninstalled { id, result } => {
                self.busy.remove(&id);
                match result {
                    Ok(()) => {
                        telemetry::record(&self.state, "app_uninstall", Some((&id, &self.version_of(&id))), None);
                        self.flash(format!("{} was removed", self.app_name(&id)));
                        self.state.installed.remove(&id);
                        self.state.save(&self.paths);
                    }
                    Err(e) => {
                        debug::log(format_args!("{id}: uninstall failed: {e}"));
                        self.app_errors.insert(id, Failure::new(Step::Uninstall, e));
                    }
                }
                self.detect();
            }
            Event::Detected(found) => self.detected = found,
            Event::Index(index) => {
                self.index = index;
                self.apply_index();
            }
            Event::Icon { id, image } => {
                let icon = match image {
                    Some(image) => Icon::Ready(ctx.load_texture(format!("icon-{id}"), image, egui::TextureOptions::LINEAR)),
                    None => Icon::Missing,
                };
                self.icons.insert(id, icon);
            }
        }
    }

    /// The version of an app on this computer, as far as the launcher knows (empty if it doesn't).
    fn version_of(&self, id: &str) -> String {
        match (self.state.installed.get(id), self.detected.get(id)) {
            (Some(i), _) => i.tag.clone(),
            (None, Some(d)) => d.version.clone().unwrap_or_default(),
            (None, None) => String::new(),
        }
    }

    /// Turns usage statistics on or off. Off forgets this installation's id.
    fn set_share_usage(&mut self, on: bool) {
        self.state.usage.asked = true;
        self.state.settings.share_usage = on;
        if on && self.state.usage.id.is_none() {
            self.state.usage.id = Some(telemetry::new_id());
            telemetry::record(&self.state, "first_run", None, None);
        } else if !on {
            self.state.usage.id = None;
        }
        self.state.save(&self.paths);
    }

    fn app_name(&self, id: &str) -> String {
        self.catalog.as_ref().and_then(|c| c.app(id)).map_or_else(|| id.to_string(), |a| a.name.clone())
    }

    fn flash(&mut self, text: String) {
        self.message = Some((text, Instant::now()));
    }

    fn status<'a>(&'a self, app: &App) -> Status<'a> {
        let Some(platform) = self.platform else { return Status::NoBuild };
        let latest = match self.releases.get(&app.id) {
            Some(Lookup::Found(r)) => Some(r),
            _ => None,
        };
        let (version, external) = if let Some(installed) = self.managed(&app.id) {
            (Some(installed.tag.clone()), false)
        } else if let Some(found) = self.detected.get(&app.id) {
            (found.version.clone(), true)
        } else if app.asset_for(platform).is_none() {
            return Status::NoBuild;
        } else {
            return latest.map_or(Status::Unknown, Status::Available);
        };
        match (latest, &version) {
            (Some(to), Some(from)) if version::is_newer(&to.tag, from) && app.asset_for(platform).is_some() => Status::Update { from: version, to, external },
            _ => Status::Installed { version, latest, external },
        }
    }

    fn is_installed(&self, app: &App) -> bool {
        matches!(self.status(app), Status::Installed { .. } | Status::Update { .. })
    }

    fn update_count(&self) -> usize {
        self.catalog.as_ref().map_or(0, |c| c.apps.iter().filter(|a| matches!(self.status(a), Status::Update { .. })).count())
    }

    /// The apps Update all would update: those with an Update button, not already under way.
    fn updatable(&self) -> Vec<String> {
        let (Some(catalog), Some(platform)) = (&self.catalog, self.platform) else { return Vec::new() };
        catalog
            .apps
            .iter()
            .filter(|a| !self.busy.contains_key(&a.id) && !self.update_queue.contains(&a.id))
            .filter(|a| matches!(self.status(a), Status::Update { external, .. } if !separate_copy(a, platform, external)))
            .map(|a| a.id.clone())
            .collect()
    }

    fn update_all(&mut self) {
        let ready = self.updatable();
        self.update_queue.extend(ready);
        self.next_update();
    }

    /// Starts the next queued update, once nothing else is installing.
    fn next_update(&mut self) {
        if self.busy.values().any(|b| matches!(b, Busy::Working(_))) {
            return;
        }
        let Some(platform) = self.platform else { return };
        while let Some(id) = self.update_queue.pop_front() {
            let Some(app) = self.catalog.as_ref().and_then(|c| c.apps.iter().find(|a| a.id == id)).cloned() else { continue };
            // Skip one that's been updated or removed since it was queued.
            let to = match self.status(&app) {
                Status::Update { to, .. } => to.clone(),
                _ => continue,
            };
            self.start_install(&app, to, platform);
            return;
        }
    }

    /// The latest release's date, `YYYY-MM-DD`.
    fn released(&self, app: &App) -> Option<&str> {
        match self.releases.get(&app.id) {
            Some(Lookup::Found(r)) if !r.published_at.is_empty() => Some(r.date()),
            _ => None,
        }
    }

    fn stars(&self, app: &App) -> u64 {
        self.index.get(&app.id).map_or(0, |e| e.stars)
    }

    fn is_featured(&self, app: &App) -> bool {
        self.catalog.as_ref().is_some_and(|c| c.featured.contains(&app.id))
    }

    /// Puts `apps` in the chosen order. Sorts are stable, so ties keep the catalog's order.
    fn sort(&self, apps: &mut [&App]) {
        match self.state.settings.sort {
            // The maintainers' picks first, in their order; then the catalog's.
            Sort::Featured => {
                let featured = self.catalog.as_ref().map(|c| c.featured.as_slice()).unwrap_or_default();
                apps.sort_by_key(|a| featured.iter().position(|id| *id == a.id).unwrap_or(usize::MAX));
            }
            Sort::Popular => apps.sort_by_key(|a| std::cmp::Reverse(self.stars(a))),
            Sort::Name => apps.sort_by_cached_key(|a| a.name.to_lowercase()),
            Sort::Updated => apps.sort_by(|a, b| self.released(b).cmp(&self.released(a))),
        }
    }

    fn matches(&self, app: &App) -> bool {
        let filter_ok = match &self.filter {
            Filter::All => true,
            Filter::Installed => self.is_installed(app),
            Filter::Updates => matches!(self.status(app), Status::Update { .. }),
            Filter::Category(c) => app.category == *c,
        };
        if !filter_ok {
            return false;
        }
        let q = self.search.trim().to_lowercase();
        if q.is_empty() {
            return true;
        }
        let hay = format!("{} {} {} {}", app.name, app.tagline, app.description, app.replaces.join(" ")).to_lowercase();
        q.split_whitespace().all(|w| hay.contains(w))
    }
}

impl eframe::App for Launcher {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let events: Vec<Event> = self.worker.events().collect();
        for event in events {
            self.handle(ctx, event);
        }
        #[cfg(tray)]
        self.tray(ctx);
        self.schedule();
        // Wake up now and then to run the schedule even when nothing else happens.
        ctx.request_repaint_after(Duration::from_secs(60));
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let p = theme::palette(ui.ctx());
        egui::Panel::left("sidebar")
            .exact_size(SIDEBAR_WIDTH)
            .resizable(false)
            .frame(Frame::new().fill(p.surface).inner_margin(Margin::symmetric(14, 16)).stroke(Stroke::new(1.0, p.line)))
            .show(ui, |ui| self.sidebar(ui, p));
        egui::CentralPanel::default().frame(Frame::new().fill(p.canvas).inner_margin(Margin { left: 28, right: 20, top: 22, bottom: 0 })).show(ui, |ui| {
            self.page_header(ui, p);
            egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                ui.set_max_width(ui.available_width() - 8.0);
                ui.add_space(16.0);
                self.banners(ui, p);
                self.grid(ui, p);
                ui.add_space(24.0);
            });
        });
        if self.show_settings {
            self.settings(ui.ctx(), p);
        }
        self.toast(ui.ctx(), p);
    }
}

// Drawing.
impl Launcher {
    fn sidebar(&mut self, ui: &mut egui::Ui, p: Palette) {
        ui.horizontal(|ui| {
            ui.add_space(4.0);
            logo(ui, 28.0);
            ui.label(RichText::new("CraftCraft").family(theme::strong()).size(17.0).color(p.ink));
        });
        ui.add_space(18.0);

        // The sections scroll when the window is too short for them and the bottom of the sidebar.
        let nav_height = (ui.available_height() - self.sidebar_bottom - 12.0).max(60.0);
        egui::ScrollArea::vertical().id_salt("nav").max_height(nav_height).auto_shrink([false, true]).show(ui, |ui| {
            let installed = self.catalog.as_ref().map_or(0, |c| c.apps.iter().filter(|a| self.is_installed(a)).count());
            let updates = self.update_count();
            let total = self.catalog.as_ref().map_or(0, |c| c.apps.len());
            let mut chosen = None;
            section_label(ui, p, "Library");
            if nav_item(ui, p, "All apps", total, self.filter == Filter::All, false) {
                chosen = Some(Filter::All);
            }
            if nav_item(ui, p, "Installed", installed, self.filter == Filter::Installed, false) {
                chosen = Some(Filter::Installed);
            }
            if nav_item(ui, p, "Updates", updates, self.filter == Filter::Updates, updates > 0) {
                chosen = Some(Filter::Updates);
            }
            if let Some(catalog) = &self.catalog {
                ui.add_space(14.0);
                section_label(ui, p, "Categories");
                for c in &catalog.categories {
                    let n = catalog.apps.iter().filter(|a| a.category == c.id).count();
                    if n > 0 {
                        let filter = Filter::Category(c.id.clone());
                        if nav_item(ui, p, &c.name, n, self.filter == filter, false) {
                            chosen = Some(filter);
                        }
                    }
                }
            }
            if let Some(f) = chosen {
                self.filter = f;
            }
        });

        ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
            ui.add_space(2.0);
            ui.label(RichText::new(format!("CraftCraft {}", env!("CARGO_PKG_VERSION"))).small().color(p.subtle));
            let (text, color) = if self.checking_catalog {
                ("Checking the catalog…".to_string(), p.subtle)
            } else if self.checking_releases {
                ("Checking for updates…".to_string(), p.subtle)
            } else if self.catalog_error.is_some() {
                ("Catalog check failed".to_string(), ui.visuals().warn_fg_color)
            } else if self.state.last_check > 0 {
                (format!("Checked {}", ago(now() - self.state.last_check)), p.subtle)
            } else {
                (String::new(), p.subtle)
            };
            let status = ui.label(RichText::new(text).small().color(color));
            if let Some(e) = &self.catalog_error {
                status.on_hover_text(e);
            }
            if let Some(until) = self.worker.releases.rate_limited_until() {
                ui.label(RichText::new(format!("GitHub limit reached; resumes in {}", minutes(until - now()))).small().color(p.subtle))
                    .on_hover_text("Set GITHUB_TOKEN to a GitHub token to raise the limit from 60 to 5,000 requests an hour.");
            }
            ui.add_space(6.0);
            if nav_item(ui, p, "⚙  Settings", 0, self.show_settings, false) {
                self.show_settings = !self.show_settings;
            }
            let busy = self.checking_catalog || self.checking_releases;
            if nav_item(ui, p, if busy { "⟳  Checking…" } else { "⟳  Check for updates" }, 0, false, false) && !busy {
                self.check_now();
            }
            self.sidebar_bottom = ui.min_rect().height();
        });
    }

    fn page_header(&mut self, ui: &mut egui::Ui, p: Palette) {
        let (title, subtitle) = match &self.filter {
            Filter::All => ("All apps".to_string(), self.catalog.as_ref().map(|c| c.tagline.clone()).unwrap_or_default()),
            Filter::Installed => ("Installed".to_string(), "Apps on this computer, whether CraftCraft installed them or not.".to_string()),
            Filter::Updates => ("Updates".to_string(), "Newer versions of apps you have.".to_string()),
            Filter::Category(id) => {
                let name = self.catalog.as_ref().and_then(|c| c.categories.iter().find(|c| &c.id == id)).map_or(id.clone(), |c| c.name.clone());
                (name, String::new())
            }
        };
        ui.horizontal(|ui| {
            ui.label(RichText::new(title).family(theme::strong()).size(26.0).color(p.ink));
            if self.filter == Filter::Updates {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let waiting = self.update_queue.len();
                    if waiting > 0 {
                        spinner_label(ui, p, &format!("Updating… {waiting} more to go"));
                    } else {
                        let ready = self.updatable().len();
                        if ready > 0 && primary(ui, p, "Update all").clicked() {
                            self.update_all();
                        }
                    }
                });
            }
        });
        if !subtitle.is_empty() {
            ui.label(RichText::new(subtitle).color(p.muted));
        }
        ui.add_space(16.0);

        // The toolbar: search on the left, how many apps and their order on the right.
        let shown = self.catalog.as_ref().map_or(0, |c| c.apps.iter().filter(|a| self.matches(a)).count());
        // In a narrow window the count and the "Sort" label give way to the search box.
        let roomy = ui.available_width() >= 620.0;
        ui.horizontal(|ui| {
            self.search_box(ui, p, if roomy { 300.0 } else { 175.0 });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let mut sort = self.state.settings.sort;
                egui::ComboBox::from_id_salt("sort").selected_text(sort.label()).width(150.0).show_ui(ui, |ui| {
                    for s in Sort::ALL {
                        ui.selectable_value(&mut sort, s, s.label());
                    }
                });
                if sort != self.state.settings.sort {
                    self.state.settings.sort = sort;
                    self.state.save(&self.paths);
                }
                if roomy {
                    ui.label(RichText::new("Sort").color(p.subtle));
                    ui.add_space(12.0);
                    ui.label(RichText::new(if shown == 1 { "1 app".to_string() } else { format!("{shown} apps") }).color(p.subtle));
                }
            });
        });
        ui.add_space(14.0);
        let y = ui.cursor().top();
        ui.painter().hline(ui.max_rect().x_range(), y, Stroke::new(1.0, p.line));
        ui.add_space(1.0);
    }

    /// A search field of a sensible width, with a drawn magnifier (the emoji one clashes).
    fn search_box(&mut self, ui: &mut egui::Ui, p: Palette, beside: f32) {
        let width = (ui.available_width() - beside).clamp(140.0, 340.0);
        let response = ui.add(
            egui::TextEdit::singleline(&mut self.search)
                .hint_text("Search apps, or what they replace")
                .margin(Margin { left: 34, right: 10, top: 8, bottom: 8 })
                .desired_width(width),
        );
        let c = response.rect.left_center() + Vec2::new(17.0, -1.0);
        let stroke = Stroke::new(1.6, if response.has_focus() { p.accent } else { p.subtle });
        ui.painter().circle_stroke(c, 5.0, stroke);
        ui.painter().line_segment([c + Vec2::splat(3.6), c + Vec2::splat(7.4)], stroke);
        if !self.search.is_empty() {
            let x = egui::Rect::from_center_size(response.rect.right_center() - Vec2::new(16.0, 0.0), Vec2::splat(20.0));
            let clear = ui.interact(x, ui.scope_id().with("clear-search"), Sense::click()).on_hover_text("Clear");
            let color = if clear.hovered() { p.ink } else { p.subtle };
            ui.painter().text(x.center(), Align2::CENTER_CENTER, "✕", FontId::proportional(12.0), color);
            if clear.clicked() {
                self.search.clear();
            }
        }
    }

    fn banners(&mut self, ui: &mut egui::Ui, p: Palette) {
        let Some(catalog) = self.catalog.clone() else {
            ui.vertical_centered(|ui| {
                ui.add_space(120.0);
                match &self.catalog_error {
                    Some(e) => {
                        ui.label(RichText::new("Couldn't load the app catalog").family(theme::strong()).size(18.0));
                        ui.label(RichText::new(e).color(p.muted));
                        ui.add_space(8.0);
                        if primary(ui, p, "Try again").clicked() {
                            self.check_now();
                        }
                    }
                    None => {
                        ui.spinner();
                        ui.label(RichText::new("Loading the app catalog…").color(p.muted));
                    }
                }
            });
            return;
        };

        if version::is_newer(&catalog.launcher.version, env!("CARGO_PKG_VERSION")) && crate::catalog::valid_repo(&catalog.launcher.repo) {
            let text = format!("CraftCraft {} is available. You have {}.", catalog.launcher.version, env!("CARGO_PKG_VERSION"));
            if banner(ui, p, p.action, "⬆", &text, None, &["Download"]).is_some() {
                system::open_url(&format!("https://github.com/{}/releases/latest", catalog.launcher.repo));
            }
        }

        if telemetry::available() && !self.state.usage.asked {
            let body = "Which apps you install and open, with your CraftCraft version and type of computer, under a random id. Never your name, files or IP address. You can turn this off now or in Settings.";
            match banner(ui, p, p.subtle, "ℹ", "CraftCraft shares anonymous usage statistics", Some(body), &["OK", "Turn off", "Privacy statement"]) {
                Some(0) => self.set_share_usage(true),
                Some(1) => self.set_share_usage(false),
                Some(_) => system::open_url(PRIVACY_STATEMENT),
                None => {}
            }
        }

        let new_apps = self.state.new_apps(&catalog);
        if !new_apps.is_empty() {
            let names: Vec<_> = new_apps.iter().filter_map(|id| catalog.app(id)).map(|a| a.name.as_str()).collect();
            let text = if names.len() == 1 { format!("New in the catalog: {}", names[0]) } else { format!("{} new apps: {}", names.len(), names.join(", ")) };
            if banner(ui, p, Color32::from_rgb(0x3b, 0x82, 0xf6), "✦", &text, None, &["Got it"]).is_some() {
                self.state.mark_all_seen(&catalog);
                self.state.save(&self.paths);
            }
        }

        let notices: Vec<_> = catalog.notices.iter().filter(|n| !self.state.dismissed_notices.contains(&n.id)).collect();
        for notice in notices {
            let body = (!notice.body.is_empty()).then_some(notice.body.as_str());
            let buttons: &[&str] = if notice.url.is_some() { &["Dismiss", "Learn more"] } else { &["Dismiss"] };
            match banner(ui, p, p.subtle, "ℹ", &notice.title, body, buttons) {
                Some(0) => {
                    self.state.dismissed_notices.insert(notice.id.clone());
                    self.state.save(&self.paths);
                }
                Some(_) => system::open_url(notice.url.as_deref().unwrap_or_default()),
                None => {}
            }
        }
    }

    fn grid(&mut self, ui: &mut egui::Ui, p: Palette) {
        let Some(catalog) = self.catalog.clone() else { return };
        let new_apps: HashSet<String> = self.state.new_apps(&catalog).into_iter().collect();
        let mut apps: Vec<&App> = catalog.apps.iter().filter(|a| self.matches(a)).collect();
        self.sort(&mut apps);
        if apps.is_empty() {
            ui.add_space(60.0);
            ui.vertical_centered(|ui| {
                let text = match self.filter {
                    Filter::Installed => "Nothing installed yet. Pick something from All apps.",
                    Filter::Updates => "Everything is up to date.",
                    _ => "No apps match.",
                };
                ui.label(RichText::new(text).color(p.muted));
            });
            return;
        }
        let width = ui.available_width();
        let columns = ((width + GAP) / (CARD_MIN_WIDTH + GAP)).floor().max(1.0) as usize;
        let card_width = (width - GAP * (columns as f32 - 1.0)) / columns as f32;
        for row in apps.chunks(columns) {
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = GAP;
                for app in row {
                    self.card(ui, p, app, card_width, new_apps.contains(&app.id));
                }
            });
            ui.add_space(GAP - ui.spacing().item_spacing.y);
        }
    }

    fn card(&mut self, ui: &mut egui::Ui, p: Palette, app: &App, width: f32, is_new: bool) {
        let size = Vec2::new(width, CARD_HEIGHT);
        let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
        let hovered = response.hovered();
        ui.painter().rect(
            rect,
            CornerRadius::same(14),
            if hovered { p.hover } else { p.surface },
            Stroke::new(1.0, if hovered { p.line_strong } else { p.line }),
            egui::StrokeKind::Inside,
        );
        let inner = rect.shrink(18.0);
        let mut ui = ui.new_child(egui::UiBuilder::new().max_rect(inner).layout(Layout::top_down(Align::Min)));
        let ui = &mut ui;
        ui.spacing_mut().item_spacing = Vec2::new(8.0, 4.0);

        ui.horizontal(|ui| {
            self.icon(ui, p, app, 46.0);
            ui.add_space(4.0);
            ui.vertical(|ui| {
                ui.add_space(2.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new(&app.name).family(theme::strong()).size(16.5).color(p.ink));
                    if is_new {
                        pill(ui, "NEW", p.action, Color32::WHITE);
                    }
                    if self.is_featured(app) {
                        pill(ui, "FEATURED", p.accent.gamma_multiply(0.16), p.accent);
                    }
                });
                let mut job = LayoutJob::default();
                job.append("Replaces ", 0.0, TextFormat::simple(FontId::proportional(12.5), p.subtle));
                job.append(&app.replaces_text(), 0.0, TextFormat::simple(FontId::new(12.5, theme::strong()), p.muted));
                ui.label(job);
            });
        });
        ui.add_space(8.0);
        ui.label(RichText::new(&app.tagline).family(theme::strong()).size(14.0).color(p.ink));
        if !app.description.is_empty() {
            let mut job = LayoutJob::single_section(app.description.clone(), TextFormat::simple(FontId::proportional(12.5), p.muted));
            job.wrap.max_rows = 2;
            job.wrap.max_width = inner.width();
            ui.label(job);
        }

        let stars = self.stars(app);
        if stars > 0 {
            stars_badge(ui, p, inner.right_top(), stars);
        }

        // The footer sits on the card's bottom edge whatever the text above.
        let foot = egui::Rect::from_min_max(egui::pos2(inner.left(), inner.bottom() - 32.0), inner.max);
        ui.painter().hline(rect.x_range().shrink(1.0), foot.top() - 12.0, Stroke::new(1.0, p.line));
        let mut foot_ui = ui.new_child(egui::UiBuilder::new().max_rect(foot).layout(Layout::left_to_right(Align::Center)));
        self.card_footer(&mut foot_ui, p, app);
    }

    fn card_footer(&mut self, ui: &mut egui::Ui, p: Palette, app: &App) {
        if let Some(busy) = self.busy.get(&app.id) {
            match busy {
                Busy::Working(Stage::Downloading { done, total }) => {
                    let mb = |b: u64| b as f64 / 1_048_576.0;
                    let (fraction, text) = match total {
                        Some(t) if *t > 0 => (*done as f32 / *t as f32, format!("{:.0} of {:.0} MB", mb(*done), mb(*t))),
                        _ => (0.0, format!("{:.1} MB", mb(*done))),
                    };
                    ui.add(
                        egui::ProgressBar::new(fraction)
                            .text(RichText::new(text).small())
                            .fill(p.action)
                            .desired_width(ui.available_width())
                            .desired_height(22.0)
                            .corner_radius(CornerRadius::same(6)),
                    );
                }
                Busy::Working(Stage::Installing) => spinner_label(ui, p, "Installing…"),
                Busy::Working(Stage::Installer) => spinner_label(ui, p, "Finish in the installer window…"),
                Busy::Removing => spinner_label(ui, p, "Removing…"),
            }
            return;
        }
        let Some(platform) = self.platform else { return };
        let installed = self.managed(&app.id).cloned();
        let shown = |v: &Option<String>| v.as_deref().map(version::display).map(|v| format!(" {v}")).unwrap_or_default();

        // Left: where it stands.
        let (status_text, status_color, hint) = match self.status(app) {
            Status::NoBuild => ("Not available for this system".to_string(), p.subtle, None),
            Status::Unknown => match self.releases.get(&app.id) {
                Some(Lookup::Failed(e)) => ("Couldn't check releases".to_string(), p.subtle, Some(e.clone())),
                Some(Lookup::NoRelease) => ("No release yet".to_string(), p.subtle, None),
                _ => ("Checking…".to_string(), p.subtle, None),
            },
            Status::Available(r) => (format!("v{} · {}", version::display(&r.tag), r.date()), p.subtle, None),
            Status::Installed { version, latest, external } => {
                let current = if latest.is_some() && version.is_some() { " · up to date" } else { "" };
                let hint = if external {
                    Some(format!("Found on this computer: installed outside CraftCraft{}.", self.scope_note(app, platform)))
                } else if installed.as_ref().is_some_and(|i| !i.verified) {
                    Some("This download had no published SHA-256, so it couldn't be verified.".to_string())
                } else {
                    None
                };
                (format!("✔ Installed{}{current}", shown(&version)), p.good, hint)
            }
            Status::Update { from, to, .. } => (format!("Update:{} → {}", shown(&from), version::display(&to.tag)), p.accent, None),
        };
        // Or where it went wrong last time: "Installer failed: …".
        let (status_text, status_color, hint) = match self.app_errors.get(&app.id) {
            Some(failure) => (format!("⚠ {failure}"), ui.visuals().error_fg_color, Some(failure_details(failure))),
            None => (status_text, status_color, hint),
        };

        // Right: what you can do. It goes first, so the status on the left gets the room that's
        // left, cut short if need be (the whole of it is on hover).
        let open_label = if app.kind == Kind::Gui { "Open" } else { "Open in terminal" };
        let (mut install, mut open) = (None, false);
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.menu_button(RichText::new("⋯").size(16.0), |ui| self.more_menu(ui, app, installed.as_ref(), platform));
            match self.status(app) {
                Status::Available(r) => {
                    if primary(ui, p, "Install").clicked() {
                        install = Some(r.clone());
                    }
                }
                Status::Update { to, external, .. } => {
                    let separate = separate_copy(app, platform, external);
                    let button = primary(ui, p, if separate { "Install latest" } else { "Update" });
                    let button = if separate {
                        button.on_hover_text("This copy was installed outside CraftCraft (winget, cargo…). CraftCraft will install its own, up-to-date copy.")
                    } else if self.detected.get(&app.id).is_some_and(|d| d.for_everyone) {
                        button.on_hover_text(
                            "This copy is installed for everyone on this computer, so Windows asks for an administrator's approval to update it.",
                        )
                    } else {
                        button
                    };
                    if button.clicked() {
                        install = Some(to.clone());
                    }
                    open = ui.button(open_label).clicked();
                }
                Status::Installed { .. } => open = ui.button(open_label).clicked(),
                Status::Unknown | Status::NoBuild => {}
            }
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                let label = egui::Label::new(RichText::new(status_text).small().color(status_color)).truncate();
                let label = ui.add(label.show_tooltip_when_elided(hint.is_none()));
                if let Some(hint) = hint {
                    label.on_hover_text(hint);
                }
            });
        });
        if let Some(release) = install {
            self.start_install(app, release, platform);
        }
        if open {
            self.open(app, installed.as_ref(), platform);
        }
    }

    /// Who a copy found in Windows' list of installed programs is for, to add to its description.
    fn scope_note(&self, app: &App, platform: Platform) -> &'static str {
        match self.detected.get(&app.id) {
            Some(d) if platform.os == crate::catalog::Os::Windows && d.uninstall.is_some() => {
                if d.for_everyone {
                    ", for everyone"
                } else {
                    ", just for you"
                }
            }
            _ => "",
        }
    }

    fn more_menu(&mut self, ui: &mut egui::Ui, app: &App, installed: Option<&Installed>, platform: Platform) {
        ui.set_min_width(180.0);
        if let Some(failure) = self.app_errors.get(&app.id) {
            if let Some(log) = &failure.log
                && ui.button("Installer log").on_hover_text("Windows Installer's own record of what happened.").clicked()
            {
                system::open_file(log);
            }
            if ui.button("Copy error").clicked() {
                ui.ctx().copy_text(failure_details(failure));
            }
            ui.separator();
        }
        if let Some(home) = &app.homepage
            && ui.button("Website").clicked()
        {
            system::open_url(home);
        }
        if ui.button("Source code").clicked() {
            system::open_url(&app.repo_url());
        }
        if let Some(Lookup::Found(r)) = self.releases.get(&app.id)
            && ui.button(format!("Release notes ({})", version::display(&r.tag))).clicked()
        {
            let repo = app.repo_url();
            system::open_url(if r.html_url.starts_with(&repo) { &r.html_url } else { &repo });
        }
        if let Status::Installed { latest: Some(r), external: true, .. } = self.status(app)
            && app.asset_for(platform).is_some()
            && ui.button("Reinstall latest").clicked()
        {
            let r = r.clone();
            self.start_install(app, r, platform);
        }
        if let Some(i) = installed {
            ui.separator();
            if ui.button("Show files").clicked() {
                system::reveal(&i.path);
            }
            if ui.button("Uninstall").clicked() {
                self.busy.insert(app.id.clone(), Busy::Removing);
                self.worker.uninstall(app.id.clone(), i.clone());
            }
        } else if let Some(found) = self.detected.get(&app.id).cloned() {
            ui.separator();
            if let Some(exe) = &found.exe
                && ui.button("Show files").clicked()
            {
                system::reveal(exe);
            }
            if let Some(command) = found.uninstall {
                if ui.button("Uninstall…").on_hover_text("Runs the app's own uninstaller.").clicked() {
                    self.busy.insert(app.id.clone(), Busy::Removing);
                    self.worker.uninstall_external(app.id.clone(), command);
                }
            } else if platform.os == crate::catalog::Os::Macos
                && ui.button("Uninstall…").on_hover_text("Shows it in Applications; move it to the Bin.").clicked()
            {
                system::open_system_uninstall(platform.os);
            }
        }
        let license = Some(app.license.as_str()).filter(|l| !l.is_empty()).or_else(|| self.index.get(&app.id).and_then(|e| e.license.as_deref()));
        if let Some(license) = license {
            ui.separator();
            ui.label(RichText::new(format!("License: {license}")).small().weak());
        }
    }

    fn icon(&self, ui: &mut egui::Ui, p: Palette, app: &App, size: f32) {
        if let Some(Icon::Ready(texture)) = self.icons.get(&app.id) {
            ui.add(
                egui::Image::from_texture(egui::load::SizedTexture::from_handle(texture))
                    .fit_to_exact_size(Vec2::splat(size))
                    .corner_radius(CornerRadius::same((size * 0.22) as u8)),
            );
            return;
        }
        let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
        let [r, g, b] = app.rgb();
        let color = Color32::from_rgb(r, g, b);
        let painter = ui.painter();
        painter.rect_filled(rect, CornerRadius::same((size * 0.24) as u8), color);
        if !matches!(self.icons.get(&app.id), Some(Icon::Loading)) {
            painter.text(rect.center(), Align2::CENTER_CENTER, initials(&app.name), FontId::monospace(size * 0.34), Color32::WHITE);
        }
        let _ = p;
    }

    fn start_install(&mut self, app: &App, release: Release, platform: Platform) {
        self.app_errors.remove(&app.id);
        self.busy.insert(app.id.clone(), Busy::Working(Stage::Downloading { done: 0, total: None }));
        // Update a copy found installed for everyone in place, rather than beside it.
        let for_everyone = self.detected.get(&app.id).is_some_and(|d| d.for_everyone);
        self.worker.install(app.clone(), release, platform, for_everyone);
    }

    fn open(&mut self, app: &App, installed: Option<&Installed>, platform: Platform) {
        match system::open_app(app, installed, self.detected.get(&app.id), &self.paths, platform) {
            Ok(()) => {
                telemetry::record(&self.state, "app_open", Some((&app.id, &self.version_of(&app.id))), None);
                self.app_errors.remove(&app.id);
            }
            Err(e) => {
                debug::log(format_args!("{}: couldn't open: {e}", app.id));
                self.app_errors.insert(app.id.clone(), Failure::new(Step::Open, e));
            }
        }
    }

    fn toast(&mut self, ctx: &egui::Context, p: Palette) {
        let Some((text, at)) = &self.message else { return };
        if at.elapsed() > Duration::from_secs(6) {
            self.message = None;
            return;
        }
        ctx.request_repaint_after(Duration::from_millis(500));
        egui::Area::new(egui::Id::unique("craftcraft-toast")).anchor(Align2::RIGHT_BOTTOM, [-20.0, -20.0]).show(ctx, |ui| {
            Frame::new()
                .fill(p.surface2)
                .stroke(Stroke::new(1.0, p.line_strong))
                .corner_radius(CornerRadius::same(10))
                .inner_margin(Margin::symmetric(14, 10))
                .shadow(egui::Shadow { offset: [0, 8], blur: 24, spread: 0, color: Color32::from_black_alpha(60) })
                .show(ui, |ui| {
                    ui.label(RichText::new(text).color(p.ink));
                });
        });
    }

    fn settings(&mut self, ctx: &egui::Context, p: Palette) {
        let mut open = self.show_settings;
        egui::Window::new(RichText::new("Settings").family(theme::strong()).size(16.0))
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .default_width(480.0)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .frame(Frame::window(&ctx.global_style()).inner_margin(Margin::same(20)))
            .show(ctx, |ui| {
                let settings = &mut self.state.settings;
                ui.horizontal(|ui| {
                    ui.checkbox(&mut settings.auto_check, "Check for new apps and updates every");
                    ui.add_enabled(settings.auto_check, egui::DragValue::new(&mut settings.check_hours).range(1..=MAX_CHECK_HOURS).speed(0.2));
                    ui.label(if settings.check_hours == 1 { "hour" } else { "hours" });
                });
                ui.label(RichText::new("And at start-up. Check for updates in the sidebar checks right away.").small().color(p.subtle));
                ui.add_space(14.0);

                #[cfg(tray)]
                {
                    // What the system calls the tray, and signing in.
                    let (tray, sign_in) = if cfg!(target_os = "macos") { ("menu bar", "log in") } else { ("tray", "sign in to Windows") };
                    ui.label(RichText::new("In the background").family(theme::strong()));
                    let mut on = self.start_on_logon;
                    if ui.checkbox(&mut on, "Start on logon").changed() {
                        match system::set_start_on_logon(on) {
                            Ok(()) => self.start_on_logon = on,
                            Err(e) => self.flash(format!("Couldn't change that: {e}")),
                        }
                    }
                    ui.label(RichText::new(format!("Opens in the {tray} when you {sign_in}, and checks for updates from there.")).small().color(p.subtle));
                    ui.add_enabled(self.tray.is_some(), egui::Checkbox::new(&mut self.state.settings.close_to_tray, format!("Minimize to {tray} on close")));
                    ui.label(RichText::new(format!("Closing the window keeps CraftCraft in the {tray}. Quit from its icon's menu.")).small().color(p.subtle));
                    ui.add_space(14.0);
                }

                ui.label(RichText::new("Command-line apps").family(theme::strong()));
                ui.label(
                    RichText::new(
                        "When you install a terminal or command-line app (ripgrep, uv…), CraftCraft puts a launcher for each of its commands in this folder:",
                    )
                    .small()
                    .color(p.muted),
                );
                ui.horizontal(|ui| {
                    ui.monospace(self.paths.bin().display().to_string());
                    if ui.small_button("Show").clicked() {
                        system::reveal(&self.paths.bin());
                    }
                });
                let commands = system::commands(&self.paths);
                let has = if commands.is_empty() {
                    "Empty for now: no command-line apps installed yet.".to_string()
                } else {
                    format!("Commands: {}", commands.join(", "))
                };
                ui.label(RichText::new(has).small().color(p.subtle));
                if system::bin_on_path(&self.paths) {
                    ui.label(RichText::new("✔ It's on your PATH, so the commands work in any terminal.").small().color(p.good));
                } else {
                    #[cfg(windows)]
                    if ui.button("Add it to my PATH").clicked() {
                        match system::add_bin_to_path(&self.paths) {
                            Ok(()) => self.flash("Added to PATH. New terminals will find the commands.".into()),
                            Err(e) => self.flash(format!("Couldn't change PATH: {e}")),
                        }
                    }
                    #[cfg(not(windows))]
                    {
                        ui.label(RichText::new("To use the commands in any terminal, add this to your shell profile:").small().color(p.muted));
                        let mut line = system::path_line(&self.paths);
                        ui.add(egui::TextEdit::singleline(&mut line).font(egui::TextStyle::Monospace).desired_width(f32::INFINITY));
                    }
                }
                ui.add_space(14.0);

                ui.label(RichText::new("Data folder").family(theme::strong()));
                ui.horizontal(|ui| {
                    ui.monospace(self.paths.root.display().to_string());
                    if ui.small_button("Show").clicked() {
                        system::reveal(&self.paths.root);
                    }
                });
                ui.label(
                    RichText::new("Apps are downloaded only when you install or update them, and their installers are cleaned up afterwards.")
                        .small()
                        .color(p.subtle),
                );
                ui.add_space(14.0);

                if telemetry::available() || telemetry::opted_out_by_environment() {
                    ui.label(RichText::new("Privacy").family(theme::strong()));
                    let blocked = telemetry::opted_out_by_environment();
                    let mut on = self.state.settings.share_usage && !blocked;
                    if ui.add_enabled(!blocked, egui::Checkbox::new(&mut on, "Share anonymous usage statistics")).changed() {
                        self.set_share_usage(on);
                    }
                    let note = if blocked {
                        "Off on this computer: DO_NOT_TRACK or CRAFTCRAFT_NO_TELEMETRY is set, so CraftCraft sends no statistics at all."
                    } else {
                        "Launches, and which apps you install, update, open and remove, with your CraftCraft version, operating system and processor type, under a random id. Never your name, files or IP address."
                    };
                    ui.label(RichText::new(note).small().color(p.subtle));
                    if ui.link("Privacy statement").clicked() {
                        system::open_url(PRIVACY_STATEMENT);
                    }
                    ui.add_space(14.0);
                }

                ui.label(RichText::new("Troubleshooting").family(theme::strong()));
                if ui.checkbox(&mut self.state.settings.debug, "Debug mode").changed() {
                    debug::set(self.state.settings.debug);
                }
                ui.label(
                    RichText::new("Logs what CraftCraft does to craftcraft.log, and keeps downloaded installers and their logs instead of deleting them.")
                        .small()
                        .color(p.subtle),
                );
                ui.horizontal(|ui| {
                    ui.monospace(self.paths.logs().display().to_string());
                    if ui.small_button("Show").clicked() {
                        system::reveal(&self.paths.logs());
                    }
                });
            });
        if !open {
            self.state.save(&self.paths);
        }
        self.show_settings = open;
    }
}

fn section_label(ui: &mut egui::Ui, p: Palette, text: &str) {
    ui.add_space(2.0);
    ui.label(RichText::new(text.to_uppercase()).font(FontId::monospace(10.5)).color(p.subtle).extra_letter_spacing(1.2));
    ui.add_space(2.0);
}

/// A sidebar row; returns whether it was clicked.
fn nav_item(ui: &mut egui::Ui, p: Palette, label: &str, count: usize, selected: bool, highlight: bool) -> bool {
    let (rect, response) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 32.0), Sense::click());
    let painter = ui.painter();
    if selected {
        painter.rect_filled(rect, CornerRadius::same(8), p.surface2);
        painter.rect_filled(egui::Rect::from_min_size(rect.min + Vec2::new(0.0, 8.0), Vec2::new(3.0, 16.0)), CornerRadius::same(2), p.action);
    } else if response.hovered() {
        painter.rect_filled(rect, CornerRadius::same(8), p.hover);
    }
    let text_color = if selected { p.ink } else { p.muted };
    let font = if selected { FontId::new(14.0, theme::strong()) } else { FontId::proportional(14.0) };
    painter.text(rect.left_center() + Vec2::new(12.0, 0.0), Align2::LEFT_CENTER, label, font, text_color);
    if count > 0 {
        let right = rect.right_center() - Vec2::new(10.0, 0.0);
        if highlight {
            let galley = painter.layout_no_wrap(count.to_string(), FontId::monospace(11.0), Color32::WHITE);
            let badge = egui::Rect::from_center_size(right - Vec2::new(galley.size().x / 2.0 + 4.0, 0.0), galley.size() + Vec2::new(12.0, 6.0));
            painter.rect_filled(badge, CornerRadius::same(9), p.action);
            painter.galley(badge.center() - galley.size() / 2.0, galley, Color32::WHITE);
        } else {
            painter.text(right, Align2::RIGHT_CENTER, count.to_string(), FontId::monospace(11.5), p.subtle);
        }
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand).clicked()
}

/// All of a failure, for its hover text and Copy error.
fn failure_details(failure: &Failure) -> String {
    let mut text = failure.to_string();
    if let Some(log) = &failure.log {
        text.push_str(&format!("\n\nWindows Installer's log: {} (⋯ → Installer log)", log.display()));
    }
    if !debug::enabled() {
        text.push_str("\n\nDebug mode, in Settings, logs more.");
    }
    text
}

/// Whether updating means installing a separate copy. An installer upgrades a system install in
/// place; an archive can't replace a copy someone else installed, so the launcher installs its own
/// beside it.
fn separate_copy(app: &App, platform: Platform, external: bool) -> bool {
    external && app.asset_for(platform).is_some_and(|(_, m)| m != Method::Installer)
}

fn primary(ui: &mut egui::Ui, p: Palette, text: &str) -> egui::Response {
    ui.add(
        egui::Button::new(RichText::new(text).family(theme::strong()).color(Color32::WHITE))
            .fill(p.action)
            .stroke(Stroke::NONE)
            .min_size(Vec2::new(76.0, 30.0)),
    )
}

fn spinner_label(ui: &mut egui::Ui, p: Palette, text: &str) {
    ui.add(egui::Spinner::new().size(14.0).color(p.accent));
    ui.label(RichText::new(text).small().color(p.muted));
}

/// A full-width notice with an icon, text and buttons (rightmost first); returns which was clicked.
fn banner(ui: &mut egui::Ui, p: Palette, color: Color32, glyph: &str, title: &str, body: Option<&str>, buttons: &[&str]) -> Option<usize> {
    let mut clicked = None;
    Frame::new()
        .fill(p.surface)
        .stroke(Stroke::new(1.0, color.gamma_multiply(0.55)))
        .corner_radius(CornerRadius::same(12))
        .inner_margin(Margin::symmetric(16, 12))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                let (rect, _) = ui.allocate_exact_size(Vec2::splat(26.0), Sense::hover());
                ui.painter().rect_filled(rect, CornerRadius::same(7), color.gamma_multiply(0.22));
                ui.painter().text(rect.center(), Align2::CENTER_CENTER, glyph, FontId::proportional(14.0), color);
                ui.add_space(4.0);
                ui.vertical(|ui| {
                    ui.label(RichText::new(title).family(theme::strong()).color(p.ink));
                    if let Some(body) = body {
                        ui.label(RichText::new(body).color(p.muted));
                    }
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    for (i, button) in buttons.iter().enumerate() {
                        if ui.button(*button).clicked() {
                            clicked = Some(i);
                        }
                    }
                });
            });
        });
    ui.add_space(10.0);
    clicked
}

/// GitHub's star count, right-aligned at `top_right`.
fn stars_badge(ui: &mut egui::Ui, p: Palette, top_right: egui::Pos2, stars: u64) {
    let galley = ui.painter().layout_no_wrap(index::short(stars), FontId::proportional(12.5), p.muted);
    let text_pos = egui::pos2(top_right.x - galley.size().x, top_right.y + 2.0);
    let center = egui::pos2(text_pos.x - 9.0, text_pos.y + galley.size().y / 2.0);
    let rect = egui::Rect::from_min_max(egui::pos2(center.x - 8.0, text_pos.y), egui::pos2(top_right.x, text_pos.y + galley.size().y));
    ui.painter().add(star(center, 5.6, Color32::from_rgb(0xe3, 0xb3, 0x41)));
    ui.painter().galley(text_pos, galley, p.muted);
    ui.interact(rect, ui.scope_id().with(("stars", top_right.x as i32, top_right.y as i32)), Sense::hover()).on_hover_text(format!("{stars} stars on GitHub"));
}

/// A filled five-pointed star: a fan of triangles from its centre.
fn star(center: egui::Pos2, radius: f32, color: Color32) -> egui::Shape {
    let mut mesh = egui::Mesh::default();
    mesh.colored_vertex(center, color);
    for i in 0..10 {
        let r = if i % 2 == 0 { radius } else { radius * 0.45 };
        let angle = std::f32::consts::PI * (i as f32 / 5.0 - 0.5);
        mesh.colored_vertex(center + Vec2::angled(angle) * r, color);
    }
    for i in 1..=10u32 {
        mesh.add_triangle(0, i, i % 10 + 1);
    }
    egui::Shape::mesh(mesh)
}

fn pill(ui: &mut egui::Ui, text: &str, fill: Color32, color: Color32) {
    Frame::new().fill(fill).corner_radius(CornerRadius::same(5)).inner_margin(Margin::symmetric(6, 2)).show(ui, |ui| {
        ui.label(RichText::new(text).font(FontId::monospace(10.0)).color(color));
    });
}

fn initials(name: &str) -> String {
    let words: Vec<&str> = name.split(|c: char| !c.is_ascii_alphanumeric()).filter(|w| !w.is_empty()).collect();
    let s: String = match words.as_slice() {
        [a, b, ..] => a.chars().take(1).chain(b.chars().take(1)).collect(),
        _ => name.chars().take(2).collect(),
    };
    s.to_uppercase()
}

/// The CraftCraft mark: two overlapping squares on an orange tile.
fn logo(ui: &mut egui::Ui, size: f32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    let painter = ui.painter();
    let u = size / 32.0;
    painter.rect_filled(rect, CornerRadius::same((7.0 * u) as u8), theme::DARK.action);
    let back = egui::Rect::from_min_size(rect.min + Vec2::splat(7.0 * u), Vec2::splat(11.0 * u));
    painter.rect_stroke(back, CornerRadius::same((2.5 * u) as u8), Stroke::new(2.6 * u, Color32::WHITE), egui::StrokeKind::Middle);
    let front = egui::Rect::from_min_size(rect.min + Vec2::splat(14.0 * u), Vec2::splat(11.0 * u));
    painter.rect_filled(front, CornerRadius::same((2.5 * u) as u8), Color32::WHITE);
}

fn ago(secs: i64) -> String {
    match secs {
        ..60 => "just now".into(),
        60..3600 => format!("{} min ago", secs / 60),
        3600..86_400 => format!("{} h ago", secs / 3600),
        _ => format!("{} days ago", secs / 86_400),
    }
}

fn minutes(secs: i64) -> String {
    let m = (secs.max(0) + 59) / 60;
    if m <= 1 { "a minute".into() } else { format!("{m} minutes") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monograms() {
        assert_eq!(initials("ripgrep"), "RI");
        assert_eq!(initials("uutils coreutils"), "UC");
        assert_eq!(initials("Zed"), "ZE");
    }
}
