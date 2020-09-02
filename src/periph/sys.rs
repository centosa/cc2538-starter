//! CC2538 sysctrl periph.

use drone_core::periph;

periph::singular! {
    /// Extracts project's specific register tokens.
    pub macro periph_sys;

    /// System peripherals.
    pub struct SysPeriph;

    drone_tisl_map::reg;
    crate::periph::sys;

    SYS_CTRL {
        CLOCK_CTRL;
        CLOCK_STA;
        RCGCUART;
    }
}
