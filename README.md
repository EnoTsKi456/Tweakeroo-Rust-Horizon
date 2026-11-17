# Tweakeroo Rust for Horizon

A native Rust mod project inspired by Tweakeroo, targeting Minecraft 26.3 and Horizon Nightly.

> **Status: first feature implemented in source — Zoom.** The feature is wired to Horizon's native key, wheel and zoom hooks, but this repository has not yet been built or tested in-game. Do not treat it as a released `.hnmod` yet.

## Target

- Minecraft 26.3
- Horizon Nightly: `9f42064c11efe7ef3e5ed2baad7ddc2a3b36eea0`
- Horizon Mixins: `e8cd0aa38cffef49cb2e088ef716baa107148c02`
- Native mod id: `tweakeroo_rust`
- Native bundle format: `.hnmod`

## Implemented in source

- Hold-to-zoom key (default C; rebindable in Options > Controls).
- Mouse wheel adjusts zoom while the key is held.
- Starting zoom setting (2x–20x).
- Smooth zoom toggle.
- Slower turning toggle.
- Settings persisted through Horizon's native mod settings API.

## Planned

Free Camera, gamma override, hand restock, hotbar swap, permanent sprint/sneak, fast/flexible/accurate placement, inventory tweaks, HUD and remaining Tweakeroo features.

## Build and test

The official Horizon native-mod repository documents the workflow and uses the same native hook API.

For a standalone checkout, copy this directory into Horizon's `native/tweakeroo_rust/` tree alongside the shared `native/tools/native.py`, then run:

```sh
python3 native/tools/native.py check tweakeroo_rust
python3 native/tools/native.py package tweakeroo_rust
```

The launcher can also build registered `.hnmod` bundles itself. A successful compile and an in-game test are still required before release.

## Upstream

Feature reference: https://modrinth.com/mod/tweakeroo

The upstream Tweakeroo project is LGPL-3.0-only. This initial Rust implementation is an independent implementation of common zoom behavior and does not copy upstream Java source. If future work translates or incorporates upstream code, preserve its notices and comply with LGPL-3.0-only.
