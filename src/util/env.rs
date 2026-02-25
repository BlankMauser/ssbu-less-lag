use std::sync::atomic::{AtomicU32, Ordering};

static ENV_FLAGS: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Copy, Debug, Default)]
struct EnvFlags(u32);

impl EnvFlags {
    // Bitfield layout:
    // bit 0: allow_buffer_swap
    // bit 1: emulator_known
    // bit 2: emulator_value
    // bit 3: swapping_buffer
    // bit 4: triple_enabled
    // bit 5: online_only
    // bit 6: online_fix_enabled
    const ALLOW_BUFFER_SWAP: u32 = 1 << 0;
    const EMULATOR_KNOWN: u32 = 1 << 1;
    const EMULATOR_VALUE: u32 = 1 << 2;
    const SWAPPING_BUFFER: u32 = 1 << 3;
    const TRIPLE_ENABLED: u32 = 1 << 4;
    const ONLINE_ONLY: u32 = 1 << 5;
    const ONLINE_FIX_ENABLED: u32 = 1 << 6;

    fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    fn bits(self) -> u32 {
        self.0
    }

    fn set_flag(&mut self, mask: u32, value: bool) {
        if value {
            self.0 |= mask;
        } else {
            self.0 &= !mask;
        }
    }

    fn flag(self, mask: u32) -> bool {
        (self.0 & mask) != 0
    }

    fn set_triple_enabled(&mut self, value: bool) {
        self.set_flag(Self::TRIPLE_ENABLED, value);
    }

    fn triple_enabled(self) -> bool {
        self.flag(Self::TRIPLE_ENABLED)
    }

    fn set_online_only(&mut self, value: bool) {
        self.set_flag(Self::ONLINE_ONLY, value);
        self.set_flag(Self::ONLINE_FIX_ENABLED, true);
    }

    fn online_only(self) -> Option<bool> {
        if !self.flag(Self::ONLINE_FIX_ENABLED) {
            None
        } else {
            Some(self.flag(Self::ONLINE_ONLY))
        }
    }
}

pub fn set_allow_buffer_swap(value: bool) {
    let mut flags = EnvFlags::from_bits(ENV_FLAGS.load(Ordering::Acquire));
    flags.set_flag(EnvFlags::ALLOW_BUFFER_SWAP, value);
    ENV_FLAGS.store(flags.bits(), Ordering::Release);
}

pub fn allow_buffer_swap() -> bool {
    let flags = EnvFlags::from_bits(ENV_FLAGS.load(Ordering::Acquire));
    flags.flag(EnvFlags::ALLOW_BUFFER_SWAP)
}

pub fn set_emulator_status(value: bool) {
    let mut flags = EnvFlags::from_bits(ENV_FLAGS.load(Ordering::Acquire));
    flags.set_flag(EnvFlags::EMULATOR_VALUE, value);
    flags.set_flag(EnvFlags::EMULATOR_KNOWN, true);
    ENV_FLAGS.store(flags.bits(), Ordering::Release);
}

pub fn emulator_cached() -> Option<bool> {
    let flags = EnvFlags::from_bits(ENV_FLAGS.load(Ordering::Acquire));
    if !flags.flag(EnvFlags::EMULATOR_KNOWN) {
        None
    } else {
        Some(flags.flag(EnvFlags::EMULATOR_VALUE))
    }
}

pub fn set_swapping_buffer(value: bool) {
    let mut flags = EnvFlags::from_bits(ENV_FLAGS.load(Ordering::Acquire));
    flags.set_flag(EnvFlags::SWAPPING_BUFFER, value);
    ENV_FLAGS.store(flags.bits(), Ordering::Release);
}

pub fn swapping_buffer() -> bool {
    let flags = EnvFlags::from_bits(ENV_FLAGS.load(Ordering::Acquire));
    flags.flag(EnvFlags::SWAPPING_BUFFER)
}

pub fn set_triple_enabled(value: bool) {
    let mut flags = EnvFlags::from_bits(ENV_FLAGS.load(Ordering::Acquire));
    flags.set_triple_enabled(value);
    ENV_FLAGS.store(flags.bits(), Ordering::Release);
}

pub fn triple_enabled() -> bool {
    let flags = EnvFlags::from_bits(ENV_FLAGS.load(Ordering::Acquire));
    flags.triple_enabled()
}

pub fn set_online_only(value: bool) {
    let mut flags = EnvFlags::from_bits(ENV_FLAGS.load(Ordering::Acquire));
    flags.set_online_only(value);
    ENV_FLAGS.store(flags.bits(), Ordering::Release);
}

pub fn online_only_cached() -> Option<bool> {
    let flags = EnvFlags::from_bits(ENV_FLAGS.load(Ordering::Acquire));
    flags.online_only()
}
