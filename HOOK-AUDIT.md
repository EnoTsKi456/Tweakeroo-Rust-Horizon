# Horizon 26.3 hook audit

Target Horizon commit: 9f42064c11efe7ef3e5ed2baad7ddc2a3b36eea0

Source inspected:
- crates/horizon-client/src/game/native_hooks.rs at the target commit
- official Horizon native Zoom mod in horizon-rust/horizon-mods

## Verified hook signatures at the target commit

- native_key(&mut self, key: &str, pressed: bool)
- native_wheel(&mut self, steps: f32, claimed: &mut bool)
- native_zoom(&self, zoom: &mut Option<NativeZoom>)
- native_mods(&mut self, mods: &mut Vec<NativeMod>)
- native_settings(&mut self, mod_id: &str, buttons: &mut Vec<String>)
- native_settings_click(&mut self, mod_id: &str, button: usize)
- native_movement_input(&self, forward: &mut f32, strafe: &mut f32, jump: &mut bool, sneak: &mut bool)
- native_camera_pose(&self, partial: f32, position: &mut [f64; 3], yaw: &mut f32, pitch: &mut f32)

The existing Zoom injection points match the official native Zoom mod's hook targets and bindings.

## Feature status

| Feature | Status | Reason |
|---|---|---|
| Zoom | Source implemented; unbuilt | Uses the verified key, wheel and zoom hooks |
| Gamma Override | Not implemented | No gamma/lightmap hook was identified in the inspected hook file |
| Free Camera | Not implemented | Camera pose hook exists, but independent camera movement/input and safe detach/reattach behavior have not been implemented |
| Permanent Sneak | Feasible hook found; not implemented | movement_input exposes the sneak flag |
| Permanent Sprint | Not yet mapped | movement_input does not expose a sprint flag |
| Hand Restock | Not yet mapped | Needs inventory/stack APIs and client interaction flow |
| Hotbar Swap | Not yet mapped | Needs a verified local inventory/input path |
| Fast/Flexible/Accurate Placement | Not yet mapped | Needs verified interaction and placement context APIs |
| Inventory tweaks | Not yet mapped | Needs verified client inventory hooks |

## Validation still required

- Run cargo test in GitHub Actions.
- Run Horizon native mixin check/package against the pinned commit.
- Install the resulting .hnmod in the user's Horizon 26.3 instance.
- Test key binding, zoom wheel, settings persistence and gameplay behavior in-game.

A source-level hook match is not a successful build or an in-game test. No release artifact is claimed by this repository yet.
