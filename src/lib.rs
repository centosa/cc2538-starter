#![feature(llvm_asm)]
#![feature(optimize_attribute)]
#![feature(allocator_api)]
#![feature(const_fn)]
#![feature(prelude_import)]
#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[macro_use]
pub mod drv;
#[macro_use]
pub mod periph;

pub mod tasks;
pub mod thr;

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;

use drone_core::heap;
use drone_tisl_map::tisl_reg_tokens;

// Create the logger.
drone_cc2538_dso::set_log! {
    uart_ty: Uart0,      // One of Uart0, Uart1.
    uart_nr: 0,          // Range 0 ... 1 
                         // TX:
    gpio_ty_tx: GpioD,   // One of GpioA, GpioB, GpioC, GpioD.
    port_nr_tx: 3,       // Range 0 ... 3 corresponding to port A to D.
    pad_ty_tx: IocD2,    // Output pad type, range IocA0 .. IocD7.
    pin_nr_tx: 2,        // Range 0 ... 7, must match last digit in output pad tpye.
                         // RX:
    gpio_ty_rx: GpioD,   // One of GpioA, GpioB, GpioC, GpioD.
    port_nr_rx: 3,       // Range 0 ... 3 corresponding to port A to D.
    pad_ty_rx: IocD0,    // Input pad type, range IocA0 .. IocD7.
    pin_nr_rx: 0,        // Range 0 ... 7, must match last digit in input pad tpye.
    baud_rate: 115200,   // Transmission speed.
}

tisl_reg_tokens! {
    /// A set of tokens for all memory-mapped registers.
    pub struct Regs;

    !scb_ccr;
    !mpu_type; !mpu_ctrl; !mpu_rnr; !mpu_rbar; !mpu_rasr;
}

heap! {
    /// A heap allocator generated from the `Drone.toml`.
    pub struct Heap;
}

/// The global allocator.
#[cfg_attr(not(feature = "std"), global_allocator)]
pub static HEAP: Heap = Heap::new();
