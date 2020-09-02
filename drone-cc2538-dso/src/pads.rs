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
macro_rules! ioc_pad_tx_assert_taken {
    (IocNs) => {
        $crate::ioc_pad_tx_assert_taken!("ioc_pd0_ns");
    };
    ($ioc_pad:literal) => {
        reg::assert_taken!("ioc_pd0_sel");
        reg::assert_taken!("ioc_pd0_over");
    }
}
