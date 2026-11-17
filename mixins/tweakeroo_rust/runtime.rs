// Tweakeroo Rust Edition bootstrap.
// This file is both the standalone Rust library and the module injected by
// horizon-mixins. Gameplay hooks are added only after their Horizon API is
// verified against the pinned source revision.

use std::cell::RefCell;

pub const ID: &str = "tweakeroo_rust";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DESCRIPTION: &str =
    "Tweakeroo-inspired client tweaks for Horizon; feature set is under development.";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum Feature {
    FreeCamera,
    GammaOverride,
    HandRestock,
    HotbarSwap,
    PermanentSprint,
    PermanentSneak,
    FastPlacement,
    FlexiblePlacement,
    AccuratePlacement,
    InventoryTweaks,
}

impl Feature {
    pub const ALL: [Self; 10] = [
        Self::FreeCamera,
        Self::GammaOverride,
        Self::HandRestock,
        Self::HotbarSwap,
        Self::PermanentSprint,
        Self::PermanentSneak,
        Self::FastPlacement,
        Self::FlexiblePlacement,
        Self::AccuratePlacement,
        Self::InventoryTweaks,
    ];

    pub const fn key(self) -> &'static str {
        match self {
            Self::FreeCamera => "free_camera",
            Self::GammaOverride => "gamma_override",
            Self::HandRestock => "hand_restock",
            Self::HotbarSwap => "hotbar_swap",
            Self::PermanentSprint => "permanent_sprint",
            Self::PermanentSneak => "permanent_sneak",
            Self::FastPlacement => "fast_placement",
            Self::FlexiblePlacement => "flexible_placement",
            Self::AccuratePlacement => "accurate_placement",
            Self::InventoryTweaks => "inventory_tweaks",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::FreeCamera => "Free Camera",
            Self::GammaOverride => "Gamma Override",
            Self::HandRestock => "Hand Restock",
            Self::HotbarSwap => "Hotbar Swap",
            Self::PermanentSprint => "Permanent Sprint",
            Self::PermanentSneak => "Permanent Sneak",
            Self::FastPlacement => "Fast Placement",
            Self::FlexiblePlacement => "Flexible Placement",
            Self::AccuratePlacement => "Accurate Placement",
            Self::InventoryTweaks => "Inventory Tweaks",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    enabled: [bool; Feature::ALL.len()],
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            enabled: [false; Feature::ALL.len()],
        }
    }
}

impl Settings {
    pub fn read(mut get: impl FnMut(&str) -> Option<String>) -> Self {
        let mut settings = Self::default();
        for feature in Feature::ALL {
            settings.enabled[feature as usize] = get(feature.key())
                .is_some_and(|value| value.trim().eq_ignore_ascii_case("true"));
        }
        settings
    }

    pub fn is_enabled(&self, feature: Feature) -> bool {
        self.enabled[feature as usize]
    }

    pub fn set(&mut self, feature: Feature, enabled: bool) {
        self.enabled[feature as usize] = enabled;
    }

    pub fn toggle(&mut self, feature: Feature) {
        self.set(feature, !self.is_enabled(feature));
    }

    pub fn labels(&self) -> Vec<String> {
        Feature::ALL
            .iter()
            .map(|feature| {
                format!(
                    "{}: {}",
                    feature.label(),
                    if self.is_enabled(*feature) { "ON" } else { "OFF" }
                )
            })
            .collect()
    }

    pub fn stored(&self) -> Vec<(&'static str, String)> {
        Feature::ALL
            .iter()
            .map(|feature| {
                (
                    feature.key(),
                    if self.is_enabled(*feature) { "true" } else { "false" }.to_owned(),
                )
            })
            .collect()
    }

    pub fn click(&mut self, index: usize) {
        if let Some(feature) = Feature::ALL.get(index).copied() {
            self.toggle(feature);
        }
    }
}

thread_local! {
    static SETTINGS: RefCell<Settings> = RefCell::new(Settings::default());
}

pub fn settings(get: impl FnMut(&str) -> Option<String>) -> Settings {
    SETTINGS.with_borrow(|current| {
        // The host helper is queried on every page open, so persisted values
        // remain authoritative across game sessions.
        let loaded = Settings::read(get);
        if *current == Settings::default() {
            loaded
        } else {
            *current
        }
    })
}

pub fn set_settings(settings: Settings) {
    SETTINGS.with_borrow_mut(|current| *current = settings);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_disabled() {
        let settings = Settings::default();
        assert!(Feature::ALL.iter().all(|feature| !settings.is_enabled(*feature)));
    }

    #[test]
    fn reads_and_round_trips_settings() {
        let settings = Settings::read(|key| {
            (key == "free_camera").then(|| "true".to_owned())
        });
        assert!(settings.is_enabled(Feature::FreeCamera));
        assert!(!settings.is_enabled(Feature::GammaOverride));

        let stored = settings.stored();
        let restored = Settings::read(|key| {
            stored.iter().find(|(stored_key, _)| *stored_key == key)
                .map(|(_, value)| value.clone())
        });
        assert_eq!(settings, restored);
    }

    #[test]
    fn settings_page_click_toggles_the_selected_feature() {
        let mut settings = Settings::default();
        settings.click(Feature::ALL.iter().position(|f| *f == Feature::FastPlacement).unwrap());
        assert!(settings.is_enabled(Feature::FastPlacement));
        assert!(settings.labels().iter().any(|label| label == "Fast Placement: ON"));
    }
}
