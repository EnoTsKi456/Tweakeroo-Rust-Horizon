# Tweakeroo Rust for Horizon

Rust-native Tweakeroo-inspired mod for Minecraft 26.3 on Horizon.

**Status: early development; no gameplay-validated release yet.** Source currently contains Zoom and Permanent Sneak implementations. GitHub Actions runs the standalone Rust tests and creates a deterministic source-only `.hnmod` candidate artifact. The Horizon mixin bundle has not yet passed the full pinned-game native build, and in-game behavior has not been verified.

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
GitHub Actions creates `target/tweakeroo_rust-0.3.0.hnmod` and uploads it as the `tweakeroo-rust-hnmod-candidate` workflow artifact. This is a source-only candidate archive, not proof that the native hooks compile or run. Download it from the latest successful [Actions run](https://github.com/EnoTsKi456/Tweakeroo-Rust-Horizon/actions), then only install after the pinned Horizon native build and in-game checks succeed. Reproduce locally with `python3 tools/package.py`.

## Upstream and licensing
Tweakeroo feature reference: https://modrinth.com/mod/tweakeroo (LGPL-3.0-only). This repository's current logic is independently written and does not translate Tweakeroo Java source. Zoom state behavior is adapted from Horizon's official native Zoom mod; see ATTRIBUTION.md.
