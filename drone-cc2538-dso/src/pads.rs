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
    (IocA0) => {
        $crate::ioc_pad_assert_taken!("ioc_pa0");
    };
    (IocA1) => {
        $crate::ioc_pad_assert_taken!("ioc_pa1");
    };
    (IocA2) => {
        $crate::ioc_pad_assert_taken!("ioc_pa2");
    };
    (IocA3) => {
        $crate::ioc_pad_assert_taken!("ioc_pa3");
    };
    (IocA4) => {
        $crate::ioc_pad_assert_taken!("ioc_pa4");
    };
    (IocA5) => {
        $crate::ioc_pad_assert_taken!("ioc_pa5");
    };
    (IocA6) => {
        $crate::ioc_pad_assert_taken!("ioc_pa6");
    };
    (IocA7) => {
        $crate::ioc_pad_assert_taken!("ioc_pa7");
    };

    (IocB0) => {
        $crate::ioc_pad_assert_taken!("ioc_pb0");
    };
    (IocB1) => {
        $crate::ioc_pad_assert_taken!("ioc_pb1");
    };
    (IocB2) => {
        $crate::ioc_pad_assert_taken!("ioc_pb2");
    };
    (IocB3) => {
        $crate::ioc_pad_assert_taken!("ioc_pb3");
    };
    (IocB4) => {
        $crate::ioc_pad_assert_taken!("ioc_pb4");
    };
    (IocB5) => {
        $crate::ioc_pad_assert_taken!("ioc_pb5");
    };
    (IocB6) => {
        $crate::ioc_pad_assert_taken!("ioc_pb6");
    };
    (IocB7) => {
        $crate::ioc_pad_assert_taken!("ioc_pb7");
    };

    (IocC0) => {
        $crate::ioc_pad_assert_taken!("ioc_pc0");
    };
    (IocC1) => {
        $crate::ioc_pad_assert_taken!("ioc_pc1");
    };
    (IocC2) => {
        $crate::ioc_pad_assert_taken!("ioc_pc2");
    };
    (IocC3) => {
        $crate::ioc_pad_assert_taken!("ioc_pc3");
    };
    (IocC4) => {
        $crate::ioc_pad_assert_taken!("ioc_pc4");
    };
    (IocC5) => {
        $crate::ioc_pad_assert_taken!("ioc_pc5");
    };
    (IocC6) => {
        $crate::ioc_pad_assert_taken!("ioc_pc6");
    };
    (IocC7) => {
        $crate::ioc_pad_assert_taken!("ioc_pc7");
    };

    (IocD0) => {
        $crate::ioc_pad_assert_taken!("ioc_pd0");
    };
    (IocD1) => {
        $crate::ioc_pad_assert_taken!("ioc_pd1");
    };
    (IocD2) => {
        $crate::ioc_pad_assert_taken!("ioc_pd2");
    };
    (IocD3) => {
        $crate::ioc_pad_assert_taken!("ioc_pd3");
    };
    (IocD4) => {
        $crate::ioc_pad_assert_taken!("ioc_pd4");
    };
    (IocD5) => {
        $crate::ioc_pad_assert_taken!("ioc_pd5");
    };
    (IocD6) => {
        $crate::ioc_pad_assert_taken!("ioc_pd6");
    };
    (IocD7) => {
        $crate::ioc_pad_assert_taken!("ioc_pd7");
    };

    ($uart:ident) => {
        compile_error!("Unsupported peripheral");
    };
    ($ioc_pad:literal) => {
        reg::assert_taken!(concat!($ioc_pad, "_sel"));
        reg::assert_taken!(concat!($ioc_pad, "_over"));
    }
}
