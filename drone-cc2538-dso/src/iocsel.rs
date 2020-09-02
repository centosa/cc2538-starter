use core::mem::MaybeUninit;
use drone_tisl_map::periph::ioc::selectors::IocSelectorMap;

pub(crate) struct IocSelectorPeriph<T: IocSelectorMap> {
    pub ioc_uartrxd_uart0: T::UIocUartrxdUart0,
    pub ioc_uartrxd_uart1: T::UIocUartrxdUart1,
}

impl<T: IocSelectorMap> IocSelectorPeriph<T> {
    #[allow(clippy::uninit_assumed_init)]
    pub(crate) fn summon() -> Self {
        unsafe { MaybeUninit::uninit().assume_init() }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! iocsel_assert_taken {
    (IocNs) => {
        $crate::sysctrl_assert_taken!("iocsel_ns");
    };
    ($ioc_pad:literal) => {
        reg::assert_taken!("ioc_uartrxd_uart0");
    }
}
