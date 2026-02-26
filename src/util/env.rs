use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use paste::paste;

static ENV_INITIALIZED: AtomicBool = AtomicBool::new(false);
static ENV_FLAGS: AtomicU32 = AtomicU32::new(0);

#[inline]
fn flag(mask: u32) -> bool {
    if !ENV_INITIALIZED.load(Ordering::Acquire) {
        return false;
    }
    (ENV_FLAGS.load(Ordering::Acquire) & mask) != 0
}

#[inline]
fn set_flag(mask: u32, value: bool) {
    if !ENV_INITIALIZED.load(Ordering::Acquire) {
        return;
    }
    let _ = ENV_FLAGS.fetch_update(Ordering::AcqRel, Ordering::Acquire, |flags| {
        let next = if value { flags | mask } else { flags & !mask };
        Some(next)
    });
}

#[inline]
pub fn initialize() {
    ENV_INITIALIZED.store(true, Ordering::Release);
}

#[inline]
pub fn is_initialized() -> bool {
    ENV_INITIALIZED.load(Ordering::Acquire)
}

#[inline]
pub fn flags() -> u32 {
    if !is_initialized() {
        return 0;
    }
    ENV_FLAGS.load(Ordering::Acquire)
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
    
    
