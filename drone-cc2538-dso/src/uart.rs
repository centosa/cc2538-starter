use core::mem::MaybeUninit;
use drone_tisl_map::periph::uart::UartMap;

pub(crate) struct UartPeriph<T: UartMap> {
    pub uart_ctl_txe: T::UUartCtlTxe,
    pub uart_ctl_uarten: T::UUartCtlUarten,
    pub uart_ctl: T::UUartCtl,
    pub uart_ibrd: T::UUartIbrd,
    pub uart_fbrd: T::UUartFbrd,
    pub uart_lcrh: T::UUartLcrh,
    pub uart_fr: T::UUartFr,
    pub uart_dr: T::UUartDr,
    pub uart_im: T::UUartIm,
    pub uart_cc: T::UUartCc,
}

impl<T: UartMap> UartPeriph<T> {
    #[allow(clippy::uninit_assumed_init)]
    pub(crate) fn summon() -> Self {
        unsafe { MaybeUninit::uninit().assume_init() }
    }
}

#[doc(hidden)]
#[must_use]
pub const fn baud_rate(baud_rate: u32) -> u32 {
    match baud_rate {
        1_200 => 0x0004_F000,
        2_400 => 0x0009_D000,
        4_800 => 0x0013_B000,
        9_600 => 0x0027_5000,
        14_400 => 0x003A_F000,
        19_200 => 0x004E_A000,
        28_800 => 0x0075_C000,
        31_250 => 0x0080_0000,
        38_400 => 0x009D_0000,
        56_000 => 0x00E5_0000,
        57_600 => 0x00EB_0000,
        76_800 => 0x013A_9000,
        115_200 => 0x01D6_0000,
        230_400 => 0x03B0_0000,
        250_000 => 0x0400_0000,
        460_800 => 0x0740_0000,
        921_600 => 0x0F00_0000,
        1_000_000 => 0x1000_0000,
        _ => panic!("Unsupported UART baud rate"),
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! uart_assert_taken {
    (Uarte0Ns) => {
        $crate::uarte_assert_taken!("uarte0_ns");
    };
}
