# `ssbu-sync`

This repo contains various implementations meant to reduce smash ultimates input delay. Tentatively it can reduce 3-4 frames of input delay depending on whether you're on emulator or console. Credit for the original code goes to blujay for their amazing work hacking up smash ultimates render logic!

Before reporting issues please read the following:

## Known Issues (WIPs)

1. High fighter counts (>= 4) might cause frame skips. This means doubles, dittos of characters like Ice Climbers, Pokemon Trainer, and Aegis.
2. Certain stages might cause frame skips with the above fighters more often than other stages
3. Results screen might have frame skips
4. Final kill sparks might have frame skips

Until these issues are fixed this reassures that this mod is best used AS A PRACTICE TOOL on netplay to get closer to offline performance. Please do not use this in netplay tournaments for both you and your opponent's sake.

## Where's the USB Stuff?

The USB stuff requires some more polish, but considering that it's a very small amount of the overall input delay removed (roughly 2.5-5ms average), I would rather release this mod in it's current state and continue working on the others when I have time.

Thanks for understanding!

## Library Usage

This crate can also be installed from another Cargo Skyline plugin.

Terminology used below:

- `ssbusync.nro`: the standalone plugin entry build (`#[skyline::main(name = "ssbusync")]`).
- `library plugin`: your own plugin that depends on this crate with `default-features = false`.
- `Exported Disabler`: your library plugin exports `ssbusync_external_disabler`, and `ssbusync.nro` detects that symbol and disables itself.

`Cargo.toml` Example:

```toml
[dependencies]
ssbusync = { git = "https://github.com/BlankMauser/ssbu-sync", default-features = false }
```

### Easy External Disable

To disable "ssbusync.nro" from your own plugin NRO, enable:

```toml
[dependencies]
ssbusync = { git = "https://github.com/BlankMauser/ssbu-sync", default-features = false, features = ["disabler-symbol"] }
```

That feature exports `ssbusync_external_disabler` from your plugin (not from `ssbusync.nro`), which `ssbusync.nro` probes before loading.

Installing hooks from your own `#[skyline::main]`:

```rust
let mut cfg = ssbusync::SsbuSyncConfig::default();
cfg.disable_vsync = true;
cfg.disable_pacer = false;
ssbusync::Install_SSBU_Sync(cfg);
```

### NRO Hook Disable

Use this if you want to wait for NRO load order and only install your custom path once.

```rust
static mut OVERRIDE_STATE: ssbusync::compatibility::OverrideState =
    ssbusync::compatibility::OverrideState::new();

#[skyline::main(name = "my_plugin")]
pub fn main() {
    let _ = unsafe { ssbusync::compatibility::try_claim_external_disabler() };
    skyline::nro::add_hook(on_nro_load).expect("nro hook unavailable");
}

fn on_nro_load(info: &skyline::nro::NroInfo) {
    let action = unsafe {
        ssbusync::compatibility::observe_and_claim_override(info, &mut OVERRIDE_STATE)
    };
    if action == ssbusync::compatibility::OverrideAction::InstallCustom {
        unsafe { ssbusync::Install_SSBU_Sync(ssbusync::SsbuSyncConfig::default()) };
    }
}
```

### Disable methods 

- `Exported Disabler` symbol: your plugin exports `ssbusync_external_disabler` as a symbol before ssbusync is loaded.
- `ssbusync_register_disabler`: your plugin registers to ssbu-syncs exported symbol before its loaded.
- Multiple plugins may export `ssbusync_external_disabler`

The install process can be time-sensitive so if there are any crashes its most likely from overlapping patches.
