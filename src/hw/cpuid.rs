use rp2040_pac::Sio;

/// Get the core ID of the current CPU.
#[inline]
pub(crate) fn get_core_id() -> u32 {
    // SAFETY: Safe to steal SIO since we're only
    // reading the CPUID register, which is RO
    let sio = unsafe { Sio::steal() };
    sio.cpuid().read().bits()
}
