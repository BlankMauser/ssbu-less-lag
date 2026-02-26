use skyline::nn::ro;
use symbaker::symbaker;

// "disabler-symbol" feature turns off ssbusync automatically
pub const SSBUSYNC_EXPORTED_DISABLE_SYMBOL: &[u8] = b"ssbusync_external_disabler\0";
pub const SSBUSYNC_STATUS_SYMBOL: &[u8] = b"ssbusync_status\0";

/// Status values for the ssbusync plugin lifecycle.
pub mod Status {
    /// Initial state — not yet installed or claimed
    pub const PENDING: u8 = 0;
    /// Another plugin has claimed ownership
    pub const CLAIMED: u8 = 1;
    /// Installed and running
    pub const INSTALLED: u8 = 2;
    /// Disabled by an external disabler symbol or explicit request
    pub const DISABLED: u8 = 3;
}

#[cfg(feature = "nro-entry")]
pub mod SSBUSyncHost {
    use std::sync::atomic::{AtomicU8, Ordering};
    use super::Status;
    
    pub static STATUS: AtomicU8 = AtomicU8::new(Status::PENDING);
    
    /// Returns true if install should be skipped (already installed, claimed, or disabled).
    pub fn should_skip_install() -> bool {
        let s = STATUS.load(Ordering::Acquire);
        s != Status::PENDING
    }
    
    pub fn is_disabled() -> bool {
        STATUS.load(Ordering::Acquire) == Status::DISABLED
    }
    
    pub fn is_installed() -> bool {
        STATUS.load(Ordering::Acquire) == Status::INSTALLED
    }
    
    pub fn is_claimed() -> bool {
        STATUS.load(Ordering::Acquire) == Status::CLAIMED
    }
    
    pub fn try_claim_install() -> bool {
        STATUS
            .compare_exchange(Status::PENDING, Status::INSTALLED, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }
    
    pub fn set_disabled() {
        STATUS.store(Status::DISABLED, Ordering::Release);
    }
    
    pub fn set_claimed() {
        STATUS.store(Status::CLAIMED, Ordering::Release);
    }
}

// ── Symbol lookup ──

fn lookup_symbol_addr(sym_nul: &[u8], caller: &str) -> Option<usize> {
    let mut addr = 0usize;
    unsafe {
        let rc = ro::LookupSymbol(&mut addr, sym_nul.as_ptr());
        println!(
            "[ssbusync] LookupSymbol caller={} sym={:?} rc={} addr=0x{:x}",
            caller,
            sym_nul,
            rc,
            addr
        );
        if rc == 0 && addr != 0 {
            Some(addr)
        } else {
            None
        }
    }
}

fn lookup_symbol_exists(sym_nul: &[u8], caller: &str) -> bool {
    lookup_symbol_addr(sym_nul, caller).is_some()
}

#[cfg(feature = "nro-entry")]
pub fn check_external_disabler() -> bool {
    let sym = SSBUSYNC_EXPORTED_DISABLE_SYMBOL;
    let sym_str = unsafe { core::ffi::CStr::from_ptr(sym.as_ptr().cast()) };
    println!(
        "[ssbusync] external_disabler sym bytes={:?} str={:?}",
        sym,
        sym_str.to_str().ok()
    );
    lookup_symbol_exists(sym, "external_disabler")
}

// ── Remote Status Query ──

#[cfg(not(feature = "nro-entry"))]
pub fn query_remote_status() -> Option<u32> {
    lookup_symbol_addr(SSBUSYNC_STATUS_SYMBOL, "query_remote_status").map(|addr| unsafe {
        let func: extern "C" fn() -> u32 = core::mem::transmute(addr);
        func()
    })
}

/// Returns true if a remote ssbusync.nro is present (its status symbol exists).
#[cfg(not(feature = "nro-entry"))]
pub fn remote_ssbusync_present() -> bool {
    lookup_symbol_exists(SSBUSYNC_STATUS_SYMBOL, "remote_ssbusync_present")
}

/// Look up the `ssbusync_status` symbol exported by the main ssbusync.nro
/// Returns `None` if the symbol is not found (no main NRO loaded).
#[cfg(not(feature = "nro-entry"))]
pub fn check_ssbusync_status() -> Option<u32> {
    match query_remote_status() {
        Some(status) => {
            match status as u8 {
                Status::PENDING   => println!("[ssbusync] status: PENDING (not yet installed)"),
                Status::CLAIMED   => println!("[ssbusync] status: CLAIMED (owned by another plugin)"),
                Status::INSTALLED => println!("[ssbusync] status: INSTALLED (running)"),
                Status::DISABLED  => println!("[ssbusync] status: DISABLED"),
                other             => println!("[ssbusync] status: UNKNOWN ({})", other),
            }
            Some(status)
        }
        None => {
            println!("[ssbusync] remote ssbusync.nro not found");
            None
        }
    }
}
