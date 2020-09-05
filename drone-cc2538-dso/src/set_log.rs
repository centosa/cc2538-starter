/// Sets DSO as default logger.
#[macro_export]
macro_rules! set_log {(
        uart_ty: $uart_ty:ident,
        pad_ty_tx: $pad_ty_tx:ident,
        pad_ty_rx: $pad_ty_rx:ident,
        baud_rate: $baud_rate:expr,
    ) => {
        const _: () = {
            use ::core::{cell::UnsafeCell, ptr::NonNull, slice};
            use ::drone_core::log;
            use ::drone_cortexm::reg;
            use ::drone_tisl_map::periph::uart::$uart_ty;
            use ::drone_tisl_map::periph::ioc::pads::{
                IocA0,IocA1,IocA2,IocA3,IocA4,IocA5,IocA6,IocA7,
                IocB0,IocB1,IocB2,IocB3,IocB4,IocB5,IocB6,IocB7,
                IocC0,IocC1,IocC2,IocC3,IocC4,IocC5,IocC6,IocC7,
                IocD0,IocD1,IocD2,IocD3,IocD4,IocD5,IocD6,IocD7,
            };
            use ::drone_tisl_map::periph::gpio::{
                GpioA, GpioB, GpioC, GpioD,
            };
            use ::drone_tisl_map::periph::ioc::selectors::IocSel;
            use ::drone_tisl_map::periph::sysctrl::Sysctrl;

            $crate::uart_assert_taken!($uart_ty);
            $crate::ioc_pad_assert_taken!($pad_ty_tx);
            $crate::ioc_pad_assert_taken!($pad_ty_rx);
            $crate::ioc_sel_assert_taken!($uart_ty);
            $crate::sysctrl_assert_taken!($uart_ty);

            struct Logger;

            #[repr(C, align(4))]
            struct Buf(UnsafeCell<[u8; 64]>);

            static BUF: Buf = Buf(UnsafeCell::new([0; 64]));

            unsafe impl $crate::Logger for Logger {
                type UartMap = $uart_ty;
                type IocPadMapTx = $pad_ty_tx;
                type IocPadMapRx = $pad_ty_rx;
                type IocSelectorMap = IocSel;
                type SysCtrlMap = Sysctrl;

                const BAUD_RATE: u32 = $baud_rate;
                const BUF_SIZE: u32 = 64;

                #[inline]
                fn buf() -> NonNull<u8> {
                    unsafe { NonNull::new_unchecked((&mut *BUF.0.get()).as_mut_ptr()) }
                }
            }

            unsafe impl Sync for Buf {}

            #[no_mangle]
            extern "C" fn drone_log_is_enabled(port: u8) -> bool {
                $crate::is_enabled::<Logger>(port)
            }

            #[no_mangle]
            extern "C" fn drone_log_write_bytes(port: u8, buffer: *const u8, count: usize) {
                $crate::write_bytes::<Logger>(port, unsafe { slice::from_raw_parts(buffer, count) })
            }

            #[no_mangle]
            extern "C" fn drone_log_write_u8(port: u8, value: u8) {
                $crate::write_bytes::<Logger>(port, &value.to_be_bytes())
            }

            #[no_mangle]
            extern "C" fn drone_log_write_u16(port: u8, value: u16) {
                $crate::write_bytes::<Logger>(port, &value.to_be_bytes())
            }

            #[no_mangle]
            extern "C" fn drone_log_write_u32(port: u8, value: u32) {
                $crate::write_bytes::<Logger>(port, &value.to_be_bytes())
            }

            #[no_mangle]
            extern "C" fn drone_log_flush() {
                $crate::flush::<Logger>();
            }
        };
    };
}
