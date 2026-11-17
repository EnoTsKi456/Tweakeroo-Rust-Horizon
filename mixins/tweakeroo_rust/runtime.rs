// Tweakeroo Rust Edition.
// Zoom behavior is adapted from Horizon's official native Zoom mod (MIT).
// Permanent Sneak uses Horizon's verified native_movement_input hook.
use std::cell::RefCell;

pub const ID: &str = "tweakeroo_rust";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DESCRIPTION: &str = "Tweakeroo-style client tweaks for Horizon (development build).";
pub const KEY: &str = "zoom";
pub const DEFAULT_KEY: &str = "key.keyboard.z";
const LEVELS: [u8; 6] = [2, 3, 4, 6, 8, 10];
const MIN: f32 = 1.25;
const MAX: f32 = 10.0;
const STEP: f32 = 1.25;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    pub level: u8,
    pub smooth: bool,
    pub sensitivity: bool,
    pub permanent_sneak: bool,
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let level = get("zoom_level")
            .and_then(|v| v.trim().parse::<i64>().ok())
            .map_or(4, |v| v.clamp(2, MAX as i64) as u8);
        let flag = |key: &str| get(key).is_some_and(|v| v.trim() == "true");
        Self {
            level,
            smooth: get("zoom_smooth").is_none_or(|v| v.trim() != "false"),
            sensitivity: get("zoom_sensitivity").is_none_or(|v| v.trim() != "false"),
            permanent_sneak: flag("permanent_sneak"),
        }
    }

    pub fn labels(&self) -> Vec<String> {
        let on = |b| if b { "ON" } else { "OFF" };
        vec![
            format!("Zoom starting level: {}x", self.level),
            format!("Smooth zoom: {}", on(self.smooth)),
            format!("Reduced turning sensitivity: {}", on(self.sensitivity)),
            format!("Permanent Sneak: {}", on(self.permanent_sneak)),
        ]
    }

    pub fn click(&mut self, index: usize) {
        match index {
            0 => {
                let at = LEVELS.iter().position(|&l| l == self.level).unwrap_or(0);
                self.level = LEVELS[(at + 1) % LEVELS.len()];
            }
            1 => self.smooth = !self.smooth,
            2 => self.sensitivity = !self.sensitivity,
            3 => self.permanent_sneak = !self.permanent_sneak,
            _ => {}
        }
    }

    pub fn stored(&self) -> [(&'static str, String); 4] {
        [
            ("zoom_level", self.level.to_string()),
            ("zoom_smooth", self.smooth.to_string()),
            ("zoom_sensitivity", self.sensitivity.to_string()),
            ("permanent_sneak", self.permanent_sneak.to_string()),
        ]
    }
}

#[derive(Default)]
struct State {
    settings: Option<Settings>,
    held: Option<f32>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    STATE.with_borrow_mut(|s| *s.settings.get_or_insert_with(|| Settings::read(get)))
}

pub fn set_settings(settings: Settings) {
    STATE.with_borrow_mut(|s| s.settings = Some(settings));
}

/// Clear transient input state when the mod is disabled or the game loses its context.
pub fn reset_zoom() {
    STATE.with_borrow_mut(|s| s.held = None);
}

pub fn key(playing: bool, pressed: bool, get: impl Fn(&str) -> Option<String>) {
    if pressed && !playing { return; }
    let level = settings(get).level;
    STATE.with_borrow_mut(|s| s.held = pressed.then(|| f32::from(level).clamp(MIN, MAX)));
}

pub fn wheel(steps: f32) -> bool {
    if !steps.is_finite() {
        return false;
    }
    STATE.with_borrow_mut(|s| {
        let Some(z) = &mut s.held else { return false; };
        *z = (*z * STEP.powf(steps)).clamp(MIN, MAX);
        true
    })
}

pub fn view() -> Option<(f32, bool, f32)> {
    STATE.with_borrow(|s| {
        let factor = s.held?;
        let settings = s.settings?;
        let turn = if settings.sensitivity { 1.0 / factor } else { 1.0 };
        Some((factor, settings.smooth, turn))
    })
}

pub fn permanent_sneak(get: impl Fn(&str) -> Option<String>) -> bool {
    settings(get).permanent_sneak
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_and_settings_round_trip() {
        let s = Settings::read(|_| None);
        assert_eq!(s, Settings { level: 4, smooth: true, sensitivity: true, permanent_sneak: false });
        let stored = s.stored();
        let loaded = Settings::read(|key| stored.iter().find(|(k, _)| *k == key).map(|(_, v)| v.clone()));
        assert_eq!(s, loaded);
    }

    #[test]
    fn settings_buttons_cycle_and_toggle() {
        let mut s = Settings { level: 10, smooth: true, sensitivity: false, permanent_sneak: false };
        s.click(0); s.click(1); s.click(2); s.click(3);
        assert_eq!(s, Settings { level: 2, smooth: false, sensitivity: true, permanent_sneak: true });
    }

    #[test]
    fn key_and_wheel_control_zoom() {
        key(false, true, |_| None);
        assert_eq!(view(), None);
        key(true, true, |_| None);
        assert_eq!(view(), Some((4.0, true, 0.25)));
        assert!(wheel(1.0));
        assert_eq!(view().map(|v| v.0), Some(5.0));
        key(false, false, |_| None);
        assert_eq!(view(), None);
    }

    #[test]
    fn zoom_wheel_clamps_and_rejects_non_finite_input() {
        key(true, true, |_| None);
        assert!(!wheel(f32::NAN));
        assert!(!wheel(f32::INFINITY));
        assert_eq!(view().map(|v| v.0), Some(4.0));
        assert!(wheel(100.0));
        assert_eq!(view().map(|v| v.0), Some(MAX));
        assert!(wheel(-100.0));
        assert_eq!(view().map(|v| v.0), Some(MIN));
        key(false, false, |_| None);
    }

    #[test]
    fn reset_clears_held_zoom_state() {
        key(true, true, |_| None);
        assert!(view().is_some());
        reset_zoom();
        assert_eq!(view(), None);
    }

    #[test]
    fn permanent_sneak_defaults_off_and_can_be_enabled() {
        let s = Settings::read(|key| (key == "permanent_sneak").then(|| "true".to_owned()));
        assert!(s.permanent_sneak);
    }
}
