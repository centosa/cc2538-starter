
#![feature(const_fn)]
#![feature(const_panic)]
#![feature(llvm_asm)]
#![feature(prelude_import)]
#![allow(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation, clippy::doc_markdown, clippy::wildcard_imports)]
#![no_std]

mod logger;
mod set_log;
mod sysctrl;
mod uart;
mod pads;
mod iocsel;
mod gpio;

pub use self::{
    logger::{flush, is_enabled, write_bytes, Logger},
    uart::baud_rate,
};

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;

#[doc(hidden)]
#[link_section = ".dronereg"]
#[no_mangle]
#[used]
static mut DSO_PORTS: u32 = 0;
