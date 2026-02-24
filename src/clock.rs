//! Clock module
//!
//! On bootup, after RTT init, we can move to a
//! higher speed clock source (XOSC + PLL)

use log::{debug, info};
use rp2040_pac::{Clocks, PllSys, Resets, Xosc};

#[derive(Debug)]
pub(crate) enum ClockError {
    XoscBadWrite,
}

fn xosc_init(xosc: &mut Xosc) -> Result<(), ClockError> {
    debug!("Initializing XOSC...");
    // Disable the oscillator first before configuring it
    xosc.ctrl().write(|w| w.enable().disable());

    xosc.status().write(|w| w.badwrite().clear_bit_by_one());
    xosc.ctrl().write(|w| w.freq_range()._1_15mhz().enable().enable());

    if xosc.status().read().badwrite().bit_is_set() {
        return Err(ClockError::XoscBadWrite);
    }

    while xosc.status().read().stable().bit_is_clear() {}

    debug!("XOSC initialized");
    Ok(())
}

fn move_refclk_to_xosc(clocks: &mut Clocks) {
    debug!("Moving reference clock to XOSC...");
    clocks.clk_ref_ctrl().write(|w| w.src().xosc_clksrc());
    while !clocks.clk_ref_ctrl().read().src().is_xosc_clksrc() {}
    debug!("Reference clock moved to XOSC");
}

fn pll_sys_init_125mhz(
    pll_sys: &mut PllSys,
    resets: &mut Resets,
) -> Result<(), ClockError> {
    debug!("Initializing PLL_SYS to 125MHz...");
    resets.reset().modify(|_, w| w.pll_sys().clear_bit());
    while resets.reset_done().read().pll_sys().bit_is_clear() {}

    pll_sys.pwr().write(|w| {
        w.pd().set_bit()
         .vcopd().set_bit()
         .postdivpd().set_bit()
    });

    pll_sys.cs().write(|w| unsafe {
        w.refdiv().bits(1)
    });
    pll_sys.fbdiv_int().write(|w| unsafe {
        w.fbdiv_int().bits(125)
    });

    pll_sys.pwr().modify(|_, w| {
        w.pd().clear_bit()
         .vcopd().clear_bit()
    });

    while pll_sys.cs().read().lock().bit_is_clear() {}

    pll_sys.prim().write(|w| unsafe {
        w.postdiv1().bits(6)
         .postdiv2().bits(2)
    });

    pll_sys.pwr().modify(|_, w| {
        w.postdivpd().clear_bit()
    });

    debug!("PLL_SYS locked at 125MHz");
    Ok(())
}

fn move_sysclk_to_pll(clocks: &mut Clocks) {
    debug!("Switching clk_sys to PLL_SYS...");

    clocks.clk_sys_ctrl().write(|w| {
        w.auxsrc().clksrc_pll_sys()
    });

    clocks.clk_sys_ctrl().write(|w| {
        w.src().clksrc_clk_sys_aux()
    });

    while !clocks.clk_sys_ctrl().read().src().is_clksrc_clk_sys_aux() {}

    debug!("clk_sys now running from PLL at 125MHz");
}

pub fn init_clocks() -> Result<(), ClockError>{
    info!("Initializing clocks...");
    let mut resets = unsafe { Resets::steal() };
    let mut xosc = unsafe { Xosc::steal() };
    let mut clocks = unsafe { Clocks::steal() };
    let mut pll_sys = unsafe { PllSys::steal() };

    if let Err(e) = xosc_init(&mut xosc) {
        return Err(e);
    }

    move_refclk_to_xosc(&mut clocks);
    pll_sys_init_125mhz(&mut pll_sys, &mut resets)?;
    move_sysclk_to_pll(&mut clocks);
    info!("Clock init done. Source: PLL (125MHz)");

    Ok(())
}
