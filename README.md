# Tweakeroo Rust for Horizon

A Rust-native port project for Tweakeroo, targeting Minecraft 26.3 on Horizon.

> **Status: bootstrap / not yet a feature-complete port.** This first commit establishes the native-mod project layout, compatibility pin, a settings model and a Horizon Mods menu entry. It does not claim that every Tweakeroo feature is implemented or that a packaged `.hnmod` has been built.

## Target

- Minecraft: 26.3
- Horizon Nightly source: `9f42064` (reported by the target launcher)
- Mod id: `tweakeroo_rust`
- Language: Rust
- Native-mod format: `.hnmod` (Horizon mixin bundle)

## Current bootstrap features

- A settings model for the first batch of Tweakeroo-style toggles.
- A native Mods menu row and settings page.
- Settings persisted using Horizon's native mod settings API.

Gameplay behavior is intentionally not wired up until each feature has a verified hook on the pinned Horizon source. A menu toggle alone is not evidence that a feature works.

## Planned port groups

1. Camera and view: Zoom integration, Free Camera.
2. Player controls: permanent sprint/sneak, movement tweaks.
3. Inventory: hand restock, hotbar swap, inventory tweaks.
4. Placement: fast/flexible/accurate placement and block interaction.
5. Visuals: gamma override, weather and HUD options.
6. Remaining Tweakeroo features, audited against the upstream feature list.

## Build

Use Horizon's native-mod tooling for the exact Horizon revision and matching `horizon-mixins` revision. The launcher compiles native mods for the game instance; this repository does not contain Minecraft assets or a prebuilt game binary.

The source pin and Mixins revision are recorded in `compatibility.json`. Do not change either pin without checking the target APIs and rebuilding the native bundle.

## Upstream and licensing

Tweakeroo is the upstream feature reference: https://modrinth.com/mod/tweakeroo

The upstream project is licensed under LGPL-3.0-only. This port must retain applicable copyright notices, provide the corresponding source and preserve LGPL obligations for any code derived from upstream. The current bootstrap code is an independent Rust scaffold, not a translation of upstream Java source.

## Validation status

No successful Horizon build or in-game test is claimed yet. Before calling a release usable, run the native-mod check/build against the pinned Horizon revision, package the `.hnmod`, and test each feature in Minecraft 26.3.
