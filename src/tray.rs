//! The tray icon (the menu bar on macOS), and on Windows, keeping to one running launcher.
//!
//! The icon stays while the launcher runs, so closing the window can leave it running there and
//! still checking for updates. On Windows, clicking it opens the window and right-clicking shows
//! its menu; on macOS, clicking shows the menu, whose first item opens the window.

use std::sync::mpsc::{Receiver, Sender, channel};

use eframe::egui;
use tray_icon::menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem};
use tray_icon::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};

use crate::i18n;

/// What the tray icon (or another copy of the launcher) asks for.
pub enum Action {
    Show,
    Check,
    Quit,
}

pub struct Tray {
    icon: TrayIcon,
    /// The menu's items, to relabel when the language changes.
    items: [MenuItem; 3],
    #[cfg(windows)]
    tx: Sender<Action>,
    actions: Receiver<Action>,
    tooltip: String,
}

impl Tray {
    pub fn new(ctx: &egui::Context) -> Result<Self, String> {
        let (tx, actions) = channel();
        let [open, check, quit] = labels().map(|label| MenuItem::new(label, true, None));
        let menu = Menu::new();
        menu.append_items(&[&open, &check, &PredefinedMenuItem::separator(), &quit]).map_err(|e| e.to_string())?;

        let items = [open.clone(), check.clone(), quit.clone()];
        let (open, check, quit) = (open.id().clone(), check.id().clone(), quit.id().clone());
        let send = notifier(&tx, ctx);
        MenuEvent::set_event_handler(Some(move |e: MenuEvent| match e.id {
            id if id == open => send(Action::Show),
            id if id == check => send(Action::Check),
            id if id == quit => send(Action::Quit),
            _ => {}
        }));
        if cfg!(windows) {
            let send = notifier(&tx, ctx);
            TrayIconEvent::set_event_handler(Some(move |e: TrayIconEvent| {
                if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = e {
                    send(Action::Show);
                }
            }));
        }

        let image = |i: egui::IconData| tray_icon::Icon::from_rgba(i.rgba, i.width, i.height).map_err(|e| e.to_string());
        let builder = TrayIconBuilder::new()
            .with_tooltip("CraftCraft")
            .with_menu(Box::new(menu))
            // A menu bar icon opens its menu on a click, as they all do.
            .with_menu_on_left_click(cfg!(target_os = "macos"));
        // macOS draws a template image in the menu bar's own colour, light or dark.
        #[cfg(target_os = "macos")]
        let builder = builder.with_icon_templated(image(crate::menu_bar_icon())?);
        #[cfg(windows)]
        let builder = builder.with_icon(image(crate::icon())?);
        let icon = builder.build().map_err(|e| e.to_string())?;
        Ok(Self {
            icon,
            items,
            #[cfg(windows)]
            tx,
            actions,
            tooltip: "CraftCraft".into(),
        })
    }

    pub fn actions(&self) -> impl Iterator<Item = Action> + '_ {
        self.actions.try_iter()
    }

    /// Puts the menu in the current language.
    pub fn relabel(&self) {
        for (item, label) in self.items.iter().zip(labels()) {
            item.set_text(label);
        }
    }

    pub fn set_tooltip(&mut self, text: String) {
        if text != self.tooltip {
            _ = self.icon.set_tooltip(Some(&text));
            self.tooltip = text;
        }
    }
}

/// The menu's items, in the current language. The system draws them, so they're as written.
fn labels() -> [String; 3] {
    let quit = if cfg!(target_os = "macos") { "tray.quit_macos" } else { "tray.quit" };
    ["tray.open", "tray.check", quit].map(|key| i18n::text(key, &[]))
}

/// Sends an action and wakes the UI, which runs even while the window is hidden.
fn notifier(tx: &Sender<Action>, ctx: &egui::Context) -> impl Fn(Action) + Send + Sync + 'static {
    let (tx, ctx) = (tx.clone(), ctx.clone());
    move |action| {
        _ = tx.send(action);
        ctx.request_repaint();
    }
}

/// Shows or hides the Dock icon. A launcher closed to the menu bar has none, like any menu bar
/// app: clicking it would have nothing to show.
#[cfg(target_os = "macos")]
pub fn show_in_dock(show: bool) {
    use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy};
    let Some(mtm) = objc2::MainThreadMarker::new() else { return };
    let policy = if show { NSApplicationActivationPolicy::Regular } else { NSApplicationActivationPolicy::Accessory };
    NSApplication::sharedApplication(mtm).setActivationPolicy(policy);
}

/// Whether the launcher is the active app, as it becomes when opened again from Finder or
/// Launchpad while it's in the menu bar.
#[cfg(target_os = "macos")]
pub fn is_active() -> bool {
    objc2::MainThreadMarker::new().is_some_and(|mtm| objc2_app_kit::NSApplication::sharedApplication(mtm).isActive())
}

#[cfg(windows)]
pub use instance::Instance;

#[cfg(windows)]
mod instance {
    use windows_sys::Win32::Foundation::{CloseHandle, ERROR_ALREADY_EXISTS, GetLastError, WAIT_OBJECT_0};
    use windows_sys::Win32::System::Threading::{CreateEventW, INFINITE, SetEvent, WaitForSingleObject};
    use windows_sys::Win32::UI::WindowsAndMessaging::{ASFW_ANY, AllowSetForegroundWindow};

    use super::{Action, Tray, notifier};

    /// The first launcher running in this Windows session. Starting another one just brings this
    /// one's window forward, so it can't be lost in the tray. (On macOS, the system sees to that.)
    pub struct Instance(usize);

    impl Instance {
        /// `None` when a launcher is already running; it has been asked to show its window, unless
        /// `quietly`.
        pub fn claim(quietly: bool) -> Option<Self> {
            // Debug builds have their own, so one can run beside the installed launcher.
            let name = if cfg!(debug_assertions) { r"Local\CraftCraft.Show.debug" } else { r"Local\CraftCraft.Show" };
            let name: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
            // SAFETY: `name` is a NUL-terminated UTF-16 string that outlives the call.
            let event = unsafe { CreateEventW(std::ptr::null(), 0, 0, name.as_ptr()) };
            if event.is_null() {
                return Some(Self(0));
            }
            // SAFETY: called straight after CreateEventW, on the same thread.
            if unsafe { GetLastError() } != ERROR_ALREADY_EXISTS {
                return Some(Self(event as usize));
            }
            if !quietly {
                // SAFETY: `event` is a valid event handle.
                unsafe {
                    // The user just started this copy, so it may hand the running one the foreground.
                    AllowSetForegroundWindow(ASFW_ANY);
                    SetEvent(event);
                }
            }
            // SAFETY: `event` is a valid handle, not used again. Closed, so the event goes once the
            // running launcher exits, for `after_restart`.
            unsafe { CloseHandle(event) };
            None
        }

        /// For a launcher an update started: waits (up to half a minute) for the old version to
        /// quit, rather than showing its window.
        pub fn after_restart() -> Option<Self> {
            for _ in 0..120 {
                if let Some(instance) = Self::claim(true) {
                    return Some(instance);
                }
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
            None
        }
    }

    impl Tray {
        /// Shows the window whenever another copy of the launcher is started.
        pub fn listen(&self, instance: Instance, ctx: &eframe::egui::Context) {
            if instance.0 == 0 {
                return;
            }
            let (event, send) = (instance.0, notifier(&self.tx, ctx));
            let spawned = std::thread::Builder::new().name("instance".into()).spawn(move || {
                // SAFETY: the handle stays open for the life of the process.
                while unsafe { WaitForSingleObject(event as _, INFINITE) } == WAIT_OBJECT_0 {
                    send(Action::Show);
                }
            });
            if let Err(e) = spawned {
                eprintln!("couldn't start instance: {e}");
            }
        }
    }
}
