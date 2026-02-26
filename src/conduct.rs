#[cfg(feature = "main-nro")]
#[allow(non_snake_case)]
pub mod SyncConductor {
    use crate::api::{Status, SSBUSYNC_EXPORTED_DISABLE_SYMBOL};
    use std::sync::atomic::{AtomicU8, Ordering};

    static STATUS: AtomicU8 = AtomicU8::new(Status::PENDING);

    #[inline]
    pub fn status() -> u8 {
        STATUS.load(Ordering::Acquire)
    }

    /// Returns true if install should be skipped (already installed, claimed, or disabled).
    pub fn should_skip_install() -> bool {
        status() != Status::PENDING
    }

    pub fn try_claim_install() -> bool {
        STATUS
            .compare_exchange(
                Status::PENDING,
                Status::INSTALLED,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_ok()
    }

    pub fn claim_for_host() -> u32 {
        let current = status();
        if current == Status::CLAIMED {
            return current as u32;
        }

        match STATUS.compare_exchange(
            Status::PENDING,
            Status::CLAIMED,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => Status::CLAIMED as u32,
            Err(existing) => existing as u32,
        }
    }

    pub fn set_disabled() {
        STATUS.store(Status::DISABLED, Ordering::Release);
    }

    pub fn external_disabler() -> bool {
        let sym = SSBUSYNC_EXPORTED_DISABLE_SYMBOL;
        let sym_str = unsafe { core::ffi::CStr::from_ptr(sym.as_ptr().cast()) };
        println!(
            "[ssbusync] external_disabler sym bytes={:?} str={:?}",
            sym,
            sym_str.to_str().ok()
        );
        crate::remote::lookup_symbol_exists(sym, "external_disabler")
    }
}
