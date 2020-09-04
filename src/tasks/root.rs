//! The root task.

use crate::{drv::sys::Sys, thr, thr::ThrsInit, Regs};
use drone_cortexm::reg::prelude::*;

/// System Resources
pub struct SystemRes {
    pub sys: Sys,
}

/// The root task handler.
#[inline(never)]
pub fn handler(reg: Regs, thr_init: ThrsInit) {
    let _thr = thr::init(thr_init);

    // Allocate global system resources.
    let res = SystemRes {
        sys: Sys::new(periph_sys!(reg)),
    };

    // Set the clock tree to run directly from the external crystal/oscillator.
    res.sys.clock_set(res.sys.get_divider("32MHz"));

    // Set IO clock to the same as system clock
    // (Concerns the Baud rate clock for SSI and UART)
    res.sys.io_clock_set(res.sys.get_divider("32MHz"));

    println!("Go!");

    // Note:  PD4 on FireFly is GREEN LED, PD4 on Re-Mote is RED LED.
    let pd4_bp: u32 = 0b10_000; // Bit position in GPIO_DIR and GPIO_DATA
    reg.gpio_d_dir.dir.write_bits(0b10_000);
    // Note:  CC2538 uses address bus filtering mechanism to write
    //        to the GPIO_DATA register. Implemented here with raw pointer
    //        arithmetic for the sake of simplicity.
    let ptr: *mut u32 = reg.gpio_d_data.as_mut_ptr();
    unsafe {
        *ptr.offset(0b10_000) ^= 0;
    }
    
    loop {
        unsafe {
            *ptr.offset(pd4_bp as isize) = pd4_bp;
        }
        Sys::mdelay(500);
        println!("hallo!");
        unsafe {
            *ptr.offset(pd4_bp as isize) ^= pd4_bp;
        }
        Sys::mdelay(500);
    }
}
