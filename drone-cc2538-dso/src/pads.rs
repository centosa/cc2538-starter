use core::mem::MaybeUninit;
use drone_tisl_map::periph::ioc::pads::{IocPadMap, IocPadOverBits};

pub(crate) struct IocPadPeriph<T: IocPadMap + IocPadOverBits> {
    pub ioc_sel: T::UIocPadSel,
    pub ioc_over: T::UIocPadOverBits,
}

impl<T: IocPadMap + IocPadOverBits> IocPadPeriph<T> {
    #[allow(clippy::uninit_assumed_init)]
    pub(crate) fn summon() -> Self {
        unsafe { MaybeUninit::uninit().assume_init() }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! ioc_pad_assert_taken {
    (IocD0) => {
        $crate::ioc_pad_assert_taken!("ioc_pd0");
    };
    (IocD2) => {
        $crate::ioc_pad_assert_taken!("ioc_pd2");
    };
    ($uart:ident) => {
        compile_error!("Unsupported peripheral");
    };
    ($ioc_pad:literal) => {
        reg::assert_taken!(concat!($ioc_pad, "_sel"));
        reg::assert_taken!(concat!($ioc_pad, "_over"));
    }
}
