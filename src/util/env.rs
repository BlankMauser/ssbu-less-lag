use std::sync::atomic::{AtomicU32, Ordering};
use paste::paste;

static ENV_FLAGS: AtomicU32 = AtomicU32::new(0);

#[inline]
fn flag(mask: u32) -> bool {
    (ENV_FLAGS.load(Ordering::Acquire) & mask) != 0
}

#[inline]
fn set_flag(mask: u32, value: bool) {
    let _ = ENV_FLAGS.fetch_update(Ordering::AcqRel, Ordering::Acquire, |flags| {
        let next = if value { flags | mask } else { flags & !mask };
        Some(next)
    });
}

macro_rules! flag_accessors {
    ($($name:ident => $bit:expr;)+) => {
        $(
            const $name: u32 = 1 << $bit;

            paste! {
                #[inline]
                pub fn [<$name:lower>]() -> bool {
                    flag($name)
                }

                #[inline]
                pub fn [<set_ $name:lower>](value: bool) {
                    set_flag($name, value);
                }
            }
        )+
    };
}

flag_accessors! {
    ALLOW_BUFFER_SWAP => 0;
    EMULATOR_KNOWN => 1;
    EMULATOR_VALUE => 2;
    SWAPPING_BUFFER => 3;
    TRIPLE_ENABLED => 4;
    ONLINE_ONLY => 5;
    ONLINE_FIX_ENABLED => 6;
}

pub fn set_emulator_status(value: bool) {
    let _ = ENV_FLAGS.fetch_update(Ordering::AcqRel, Ordering::Acquire, |flags| {
        let next = (flags | EMULATOR_KNOWN) & !EMULATOR_VALUE;
        Some(if value { next | EMULATOR_VALUE } else { next })
    });
}

pub fn emulator_cached() -> Option<bool> {
    if !emulator_known() {
        None
    } else {
        Some(emulator_value())
    }
    
    
}
