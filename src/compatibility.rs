use skyline::nn::ro;

unsafe fn symbol_exists(sym: &[u8]) -> bool {
    let mut addr: usize = 0;
    let rc = ro::LookupSymbol(&mut addr as *mut usize, sym.as_ptr());
    rc == 0 && addr != 0
}

unsafe fn hdr_sync_disable() -> bool {
    let mut addr: usize = 0;
    let rc = ro::LookupSymbol(&mut addr as *mut usize, b"hdr_disable_ssbusync\0".as_ptr());
    if rc == 0 && addr != 0 {
        let f: extern "C" fn() -> u32 = core::mem::transmute(addr);
        return f() != 0;
    }
    return symbol_exists(b"hdr_disable_ssbusync\0")
}



pub unsafe fn disablers() -> bool {
    return hdr_sync_disable();
}
