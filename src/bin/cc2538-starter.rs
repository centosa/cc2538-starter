#![feature(naked_functions)]
#![no_main]
#![no_std]

use drone_core::{mem, token::Token};
use drone_cortexm::processor;
use cc2538_starter::{
    tasks,
    thr::{ThrsInit, Vtable},
    Regs,
};

pub struct FlashCca {
    _bootloader_backdoor_disable: u32,
    _is_valid: u32,
    _flash_start_addr: u32,
    _padding: u32,
}

#[doc(hidden)]
#[link_section = ".flash_cca"]
#[no_mangle]
pub static FLASH_CCA: FlashCca = FlashCca {
    _bootloader_backdoor_disable: 0xEFFFFFFF,
    _is_valid: 0,
    _flash_start_addr: 0x00200000,
    _padding: 0xFFFFFFFF,
};

/// The vector table.
#[no_mangle]
#[used]
pub static VTABLE: Vtable = Vtable::new(reset);

/// The entry point.
///
/// # Safety
///
/// This function should not be called by software.
#[no_mangle]
#[naked]
pub unsafe extern "C" fn reset() -> ! {
    mem::bss_init();
    mem::data_init();
    processor::fpu_init(true);
    tasks::root(Regs::take(), ThrsInit::take());
    loop {
        processor::wait_for_int();
    }
}
