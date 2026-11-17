# Tweakeroo Rust for Horizon

Rust-native Tweakeroo-inspired mod for Minecraft 26.3 on Horizon.

**Status: early development; no installable .hnmod release yet.** Source currently contains Zoom and Permanent Sneak implementations. Rust unit tests are configured in GitHub Actions. The Horizon mixin bundle has not yet been checked/packaged against the user's pinned game build, and in-game behavior has not been verified.

## Target
- Minecraft 26.3
- Horizon Nightly commit: 9f42064c11efe7ef3e5ed2baad7ddc2a3b36eea0
- Native mod id: tweakeroo_rust
- Language: Rust
- Package format: .hnmod

## Implemented in source (not yet game-tested)
- Zoom: hold Z (rebindable), wheel zoom, 2x/3x/4x/6x/8x/10x starting level, smooth zoom and reduced turning sensitivity.
- Permanent Sneak: setting forces the local movement-input sneak flag while enabled.

## Planned / not implemented
- Gamma Override
- Free Camera
- Permanent Sprint
- Hand Restock
- Hotbar Swap
- Fast/Flexible/Accurate Placement
- Remaining Tweakeroo inventory and client tweaks

See HOOK-AUDIT.md for which Horizon hook signatures were verified and why the remaining features need additional API work.

## Tests
GitHub Actions runs:

    cargo test --manifest-path Cargo.toml

Passing unit tests do not replace the native Horizon mixin check or in-game testing.

## Packaging
No .hnmod artifact is currently published. Once the native mixin check succeeds against the pinned Horizon revision, package and test the bundle in the target launcher before publishing a release.

## Upstream and licensing
Tweakeroo feature reference: https://modrinth.com/mod/tweakeroo (LGPL-3.0-only). This repository's current logic is independently written and does not translate Tweakeroo Java source. Zoom state behavior is adapted from Horizon's official native Zoom mod; see ATTRIBUTION.md.
