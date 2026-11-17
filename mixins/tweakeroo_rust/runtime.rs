// Tweakeroo Rust Edition: first implemented feature, Zoom.
// Uses the same stable native hook API as Horizon's official Zoom mod.
use std::cell::RefCell;

pub const ID: &str = "tweakeroo_rust";
pub const VERSION: &str = "0.2.0";
pub const DESCRIPTION: &str =
    "Tweakeroo-style client tweaks for Horizon. Zoom is implemented; more features are planned.";
pub const ZOOM_KEY: &str = "zoom";
pub const DEFAULT_ZOOM_KEY: &str = "key.keyboard.c";
const MIN_ZOOM: f32 = 1.25;
const MAX_ZOOM: f32 = 20.0;
const WHEEL_STEP: f32 = 1.25;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    pub starting_zoom: u8,
    pub smooth: bool,
    pub slower_turning: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { starting_zoom: 4, smooth: true, slower_turning: true }
    }
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let default = Self::default();
        let starting_zoom = get("starting_zoom")
            .and_then(|v| v.trim().parse::<i64>().ok())
            .map_or(default.starting_zoom, |v| v.clamp(2, 20) as u8);
        let flag = |key: &str, fallback: bool| {
            get(key).map_or(fallback, |v| v.trim().eq_ignore_ascii_case("true"))
        };
        Self {
            starting_zoom,
            smooth: flag("smooth", default.smooth),
            slower_turning: flag("slower_turning", default.slower_turning),
        }
    }

    pub fn labels(&self) -> Vec<String> {
        let on = |value| if value { "ON" } else { "OFF" };
        vec![
            format!("Starting zoom: {}x", self.starting_zoom),
            format!("Smooth zoom: {}", on(self.smooth)),
            format!("Slower turning: {}", on(self.slower_turning)),
        ]
    }

    pub fn click(&mut self, index: usize) {
        match index {
            0 => self.starting_zoom = if self.starting_zoom >= 20 { 2 } else { self.starting_zoom + 2 },
            1 => self.smooth = !self.smooth,
            2 => self.slower_turning = !self.slower_turning,
            _ => {}
        }
    }

    pub fn stored(&self) -> [(&'static str, String); 3] {
        [
            ("starting_zoom", self.starting_zoom.to_string()),
            ("smooth", self.smooth.to_string()),
            ("slower_turning", self.slower_turning.to_string()),
        ]
    }
}

#[derive(Default)]
struct State {
    settings: Option<Settings>,
    held_zoom: Option<f32>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    STATE.with_borrow_mut(|state| *state.settings.get_or_insert_with(|| Settings::read(get)))
}

pub fn set_settings(settings: Settings) {
    STATE.with_borrow_mut(|state| state.settings = Some(settings));
}

pub fn key(playing: bool, pressed: bool, get: impl Fn(&str) -> Option<String>) {
    if pressed && !playing { return; }
    let level = settings(get).starting_zoom;
    STATE.with_borrow_mut(|state| {
        state.held_zoom = pressed.then(|| f32::from(level).clamp(MIN_ZOOM, MAX_ZOOM));
    });
}

pub fn wheel(steps: f32) -> bool {
    STATE.with_borrow_mut(|state| {
        let Some(zoom) = &mut state.held_zoom else { return false; };
        *zoom = (*zoom * WHEEL_STEP.powf(steps)).clamp(MIN_ZOOM, MAX_ZOOM);
        true
    })
}

pub fn view() -> Option<(f32, bool, f32)> {
    STATE.with_borrow(|state| {
        let factor = state.held_zoom?;
        let settings = state.settings?;
        let turn = if settings.slower_turning { 1.0 / factor } else { 1.0 };
        Some((factor, settings.smooth, turn))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_default_and_cycle() {
        let mut s = Settings::default();
        assert_eq!(s.starting_zoom, 4);
        s.click(0);
        assert_eq!(s.starting_zoom, 6);
        s.click(1);
        assert!(!s.smooth);
    }

    #[test]
    fn settings_round_trip() {
        let s = Settings { starting_zoom: 8, smooth: false, slower_turning: true };
        let values = s.stored();
        let restored = Settings::read(|key| values.iter().find(|(k, _)| *k == key).map(|(_, v)| v.clone()));
        assert_eq!(s, restored);
    }

    #[test]
    fn zoom_uses_wheel_and_releases() {
        let get = |_: &str| None;
        key(true, true, get);
        assert_eq!(view().map(|v| v.0), Some(4.0));
        assert!(wheel(1.0));
        assert_eq!(view().map(|v| v.0), Some(5.0));
        key(false, false, get);
        assert_eq!(view(), None);
    }
}
