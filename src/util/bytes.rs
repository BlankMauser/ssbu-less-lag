pub const MAX_PREFIX_LEN: usize = 16;

pub fn split_ascii_prefix(prefix: &str) -> Option<(u64, u64, u8)> {
    let bytes = prefix.as_bytes();
    if bytes.len() > MAX_PREFIX_LEN || !bytes.is_ascii() {
        return None;
    }

    let mut lo = 0u64;
    let mut hi = 0u64;
    for (i, b) in bytes.iter().copied().enumerate() {
        if i < 8 {
            lo |= (b as u64) << (i * 8);
        } else {
            hi |= (b as u64) << ((i - 8) * 8);
        }
    }
    Some((lo, hi, bytes.len() as u8))
}

pub fn write_prefix_bytes(lo: u64, hi: u64, len: usize, out: &mut [u8; MAX_PREFIX_LEN]) -> usize {
    let capped_len = core::cmp::min(len, MAX_PREFIX_LEN);
    for i in 0..capped_len {
        out[i] = if i < 8 {
            ((lo >> (i * 8)) & 0xFF) as u8
        } else {
            ((hi >> ((i - 8) * 8)) & 0xFF) as u8
        };
    }
    capped_len
}
