use core::mem::MaybeUninit;
use drone_tisl_map::periph::gpio::GpioPortMap;

pub(crate) struct GpioPortPeriph<T: GpioPortMap> {
    pub gpio_dir: T::UGpioDir,
    pub gpio_afsel: T::UGpioAfsel,
}

impl<T: GpioPortMap> GpioPortPeriph<T> {
    #[allow(clippy::uninit_assumed_init)]
    pub(crate) fn summon() -> Self {
        unsafe { MaybeUninit::uninit().assume_init() }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! gpio_port_assert_taken {
    (GpioA) => {
        $crate::gpio_assert_taken!("gpio_a_dir");
    };
    (GpioB) => {
        $crate::gpio_assert_taken!("gpio_b_dir");
    };
    (GpioC) => {
        $crate::gpio_assert_taken!("gpio_c_dir");
    };
    (GpioD) => {
        $crate::gpio_assert_taken!("gpio_d_dir");
    };
    ($gpio:ident) => {
        compile_error!("Unsupported peripheral");
    };
    ($gpio:literal) => {
        reg::assert_taken!(concat!($gpio));
    }
}
