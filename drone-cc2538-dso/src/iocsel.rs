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
macro_rules! ioc_sel_assert_taken {
    (Uart0) => {
        $crate::ioc_sel_assert_taken!("ioc_uartrxd_uart0");
    };
    (Uart1) => {
        $crate::ioc_sel_assert_taken!("ioc_uartrxd_uart1");
    };
    ($uart:ident) => {
        compile_error!("Unsupported peripheral");
    };
    ($uart:literal) => {
        reg::assert_taken!($uart);
    }
}

