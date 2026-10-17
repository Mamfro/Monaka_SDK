# Monaka SDK

The crates a Monaka VR game pack is built with. A pack is a producer DLL that
[Monaka VR](https://github.com/Mamfro/monaka_vr) loads into a game; everything it compiles against
is here. Monaka VR itself (the window, the headset viewer, the loader) lives in its own repository
and builds on the SDK's `monaka_contract`, `monaka_core` and `monaka_channel`.

> **Pre-alpha.** The API still changes from one version to the next. What crosses between a pack
> and Monaka VR is the pack contract (`monaka_contract`): a pack keeps working with every Monaka VR
> that speaks its contract.

## Crates

| Crate | What it is |
|---|---|
| `monaka_contract` | The pack contract: the manifest, the records shared with the host, the host features |
| `monaka_core` | Shared maths, options, cameras, situations, features |
| `monaka_channel` | The channel between a game and the viewer: images, poses, hands, fingers, depth |
| `monaka_hook` | In-process hooking: memory access, modules, checked inline and vtable hooks |
| `monaka_producer` | A pack's life in the game: components, start and stop, the ledger, logging |
| `monaka_stereo` | Stereo: the present loops, frame warping, HUD layers, per-eye upscaling |
| `monaka_body` | Head aim, the hand and finger rigs, melee, held items, gestures |
| `monaka_pad` | The controllers as the game's gamepad, and the game's actions |
| `monaka_warp` | Reprojection: one eye's frame into the other by its depth |
| `monaka_gl`, `monaka_vulkan` | Frames out of OpenGL and Vulkan games |
| `monaka_ffx`, `monaka_fsr11`, `monaka_framegen`, `monaka_streamline` | AMD FidelityFX upscaling and frame generation, NVIDIA Streamline |

## Building a pack against it

Until the crates are published, a pack depends on them by path, with this repository beside
Monaka VR's and the pack's:

```toml
[workspace.dependencies]
monaka_contract = { path = "../monaka_sdk/crates/contract" }
monaka_producer = { path = "../monaka_sdk/crates/producer" }
```

`cargo-config.example.toml` builds the SDK into Monaka VR's `target`, beside the launcher, where
`monaka_play harness` finds the channel harness (`cargo build --release -p monaka_channel
--example harness`).

## Licence

GPL-3.0 (see `LICENSE`), so VR support for a game stays open.
