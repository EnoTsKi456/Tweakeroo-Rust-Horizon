# Tweakeroo Rust for Horizon

Rust-native Tweakeroo-inspired mod for Minecraft 26.3 on Horizon.

**Status: early port.** The first gameplay feature candidate is configurable Zoom. It is wired to Horizon's native key, wheel and zoom hooks and has unit tests for its state/settings logic. It has **not yet been compiled against the pinned Horizon checkout or tested in-game**, so treat this as development code, not an installable release.

## Target

- Minecraft: 26.3
- Horizon Nightly: 2026-11-13
- Horizon commit: 9f42064c11efe7ef3e5ed2baad7ddc2a3b36eea0
- Native mod format: .hnmod
- Mod id: tweakeroo_rust

## Implemented in source

- Zoom key (default Z; rebindable in Options > Mods)
- Mouse-wheel zoom while the key is held
- Starting zoom level: 2x, 3x, 4x, 6x, 8x, 10x
- Smooth zoom setting
- Reduced turning sensitivity setting
- Settings persisted through Horizon's native mod settings API

## Planned

1. Gamma Override
2. Free Camera
3. Permanent Sprint / Sneak
4. Hand Restock and Hotbar Swap
5. Placement and inventory tweaks
6. Remaining Tweakeroo feature audit and implementation

A planned item is not implemented merely because it appears in this list. Each feature will be marked implemented only after its hooks are added and it passes tests.

## Build and test

The source uses Horizon's native hook API 1 and the matching horizon-mixins revision in compatibility.json.

Run the Rust logic tests:

    cargo test --manifest-path Cargo.toml

For the actual Horizon bundle, use the native-mod tooling from the matching Horizon Mods checkout:

    python3 native/tools/native.py check tweakeroo_rust --horizon 9f42064c11efe7ef3e5ed2baad7ddc2a3b36eea0
    python3 native/tools/native.py package tweakeroo_rust

The packaged .hnmod is not included yet. A successful build and in-game proof must be recorded before publishing an installable release.

## Upstream and licensing

Tweakeroo is the feature reference: https://modrinth.com/mod/tweakeroo

Tweakeroo is LGPL-3.0-only; this repository's independently written scaffolding does not contain translated Tweakeroo Java implementation. The Zoom state/behavior is adapted from Horizon's official native Zoom mod, which is MIT licensed; see ATTRIBUTION.md. Keep notices and source attribution with redistributed derivatives.
