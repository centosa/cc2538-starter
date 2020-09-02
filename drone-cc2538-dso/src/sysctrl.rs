use core::mem::MaybeUninit;
use drone_tisl_map::periph::sysctrl::SysCtrlMap;

pub struct SysCtrlPeriph<T: SysCtrlMap> {
    pub sysctrl_clock_sta: T::USysCtrlClockSta,
    pub sysctrl_rcgcuart: T::USysCtrlRcgcuart,
}


impl<T: SysCtrlMap> SysCtrlPeriph<T> {
    #[allow(clippy::uninit_assumed_init)]
    pub(crate) fn summon() -> Self {
        unsafe { MaybeUninit::uninit().assume_init() }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! sysctrl_assert_taken {
    (IocNs) => {
        $crate::sysctrl_assert_taken!("sysctrl_ns");
    };
    ($ioc_pad:literal) => {
        reg::assert_taken!("sysctrl_rcgcuart_uart0");
    }
}
