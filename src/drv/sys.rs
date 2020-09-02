//! driver.

use crate::periph::sys::SysPeriph;
use drone_cortexm::reg::prelude::*;

/// Sysctrl driver.
pub struct Sys {
    periph: SysPeriph,
}

impl Sys {
    /// Creates a new [`Sysctrl`].
    #[inline]
    pub fn new(periph: SysPeriph) -> Self {
        Self { periph }
    }

    /// Releases the peripheral.
    #[inline]
    pub fn free(self) -> SysPeriph {
        self.periph
    }

    // Clock control.
    pub fn get_divider(&self, div_code: &str) -> u32 {
        match div_code {
            "32MHz" => 0,
            _ => panic!("invalid divider code"),
        }
    }

    pub fn clock_set(&self, div: u32) {
        // Enable AMP detect to make sure XOSC starts correctly.
        // Set 32kHz clock, Osc and SysDiv.
        self.periph.sys_ctrl_clock_ctrl.modify(|r| {
            r.set_amp_det()
                .clear_osc32k()
                .clear_osc()
                .write_sys_div(div)
        });
        // Wait for 32-MHz crystal oscillator
        while self.periph.sys_ctrl_clock_sta.osc.read_bit() {
            Sys::sys_ctrl_delay(16);
        }
    }

    /// Sets the IO clocking of the device.
    //  This function configures the IO clocking of the device
    //  (that is, the Baud rate clock for SSI and UART).
    pub fn io_clock_set(&self, div: u32) {
        self.periph
            .sys_ctrl_clock_ctrl
            .modify(|r| r.write_io_div(div));
    }

    /// Enable a peripheral.
    pub fn enable_peripheral(&self, peri: &str) {
        match peri {
            "uart0" => {
                // Enable peripheral in run mode.
                self.periph.sys_ctrl_rcgcuart.modify(|r| r.set_uart0());
            }
            _ => panic!("invalid periheral"),
        }
    }

    // -- Provide a small delay.
    pub fn sys_ctrl_delay(cycles: u32) {
        for _ in 0..cycles {
            unsafe { llvm_asm!("" :::: "volatile") };
        }
    }

    pub fn mdelay(duration: u32) {
        // Approximitly milliseconds.
        for _ in 1..duration {
            Sys::sys_ctrl_delay(10800);
        }
    }
}
