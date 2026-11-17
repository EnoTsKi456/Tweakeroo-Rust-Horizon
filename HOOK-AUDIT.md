# Horizon 26.3 hook audit

Target Horizon commit: 9f42064c11efe7ef3e5ed2baad7ddc2a3b36eea0

Source inspected: crates/horizon-client/src/game/native_hooks.rs at the target commit, plus Horizon's official native Zoom implementation.

## Verified hook signatures
- native_key(&mut self, key: &str, pressed: bool)
- native_wheel(&mut self, steps: f32, claimed: &mut bool)
- native_zoom(&self, zoom: &mut Option<NativeZoom>)
- native_mods(&mut self, mods: &mut Vec<NativeMod>)
- native_settings(&mut self, mod_id: &str, buttons: &mut Vec<String>)
- native_settings_click(&mut self, mod_id: &str, button: usize)
- native_movement_input(&self, forward: &mut f32, strafe: &mut f32, jump: &mut bool, sneak: &mut bool)
- native_camera_pose(&self, partial: f32, position: &mut [f64; 3], yaw: &mut f32, pitch: &mut f32)

## Feature status
| Feature | Status | Notes |
|---|---|---|
| Zoom | Implemented in source; unbuilt | Verified key, wheel and zoom hooks; behavior adapted from Horizon's official Zoom mod |
| Permanent Sneak | Implemented in source; unbuilt | Verified movement_input hook exposes the sneak flag |\n| Zoom lifecycle cleanup | Implemented in source; unbuilt | Clears held zoom when the mod is disabled or the client unloads a world |
| Gamma Override | Not implemented | No gamma/lightmap hook identified in inspected hook source |
| Free Camera | Not implemented | Camera pose hook exists, but independent movement/input and safe camera lifecycle still need implementation |
| Permanent Sprint | Not implemented | movement_input does not expose sprint state |
| Hand Restock | Not implemented | Requires verified inventory and stack interaction APIs |
| Hotbar Swap | Not implemented | Requires verified local inventory/input path |
| Fast/Flexible/Accurate Placement | Not implemented | Requires verified interaction and placement context APIs |
| Inventory tweaks | Not implemented | Requires verified client inventory hooks |

## Validation still required
1. GitHub Actions Rust unit tests.
2. Horizon native mixin check and package against the pinned commit.
3. Install .hnmod in Horizon Minecraft 26.3.
4. Test keybind, zoom, setting persistence and Permanent Sneak in-game.

A hook signature match or passing unit tests is not proof of a successful native build or in-game behavior. No release artifact is claimed.
