# `ssbu-less-lag`

This repo contains various implementations meant to reduce smash ultimates input delay. Tentatively it can reduce 3-4 frames of input delay depending on whether you're on emulator or console. Credit goes to blujay for their amazing work hacking up smash ultimates render logic!

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

`Cargo.toml` Example:

```toml
[dependencies]
ssbusync = { git = "https://github.com/BlankMauser/ssbu-less-lag", default-features = false }
```

Installing hooks from your own `#[skyline::main]`:

```rust
let mut cfg = ssbusync::SsbuSyncConfig::default();
cfg.disable_vsync = true;
cfg.disable_pacer = false;
ssbusync::install_ssbu_sync(cfg);
```


