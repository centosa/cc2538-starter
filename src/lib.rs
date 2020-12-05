#![feature(llvm_asm)]
#![feature(allocator_api)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(prelude_import)]
#![feature(proc_macro_hygiene)]
#![feature(slice_ptr_get)]
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
    pad_ty_tx: IocD2,    // Output pad type, range IocA0 .. IocD7.
    pad_ty_rx: IocD0,    // Input pad type, range IocA0 .. IocD7.
    baud_rate: 115200,   // Transmission speed.
}

tisl_reg_tokens! {
    /// A set of tokens for all memory-mapped registers.
    index => pub Regs;

    exclude => {

        uart0_ctl, uart0_ibrd, uart0_fbrd, uart0_lcr, uart0_fr, uart0_dr, uart0_im, uart0_cc,
        ioc_pd0_sel, ioc_pd0_over, ioc_pd2_sel, ioc_pd2_over,    
        ioc_uartrxd_uart0,

        scb_ccr,
        mpu_type, mpu_ctrl, mpu_rnr, mpu_rbar, mpu_rasr,
    }
}

heap! {
    /// A heap allocator generated from the `Drone.toml`.
    heap => pub Heap;
}

/// The global allocator.
#[cfg_attr(not(feature = "std"), global_allocator)]
pub static HEAP: Heap = Heap::new();
