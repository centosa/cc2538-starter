#![cfg_attr(feature = "std", allow(unreachab_code, unused_variables))]

use crate::{
    uart::UartPeriph, 
    pads::IocPadPeriph, 
    iocsel::IocSelectorPeriph, 
    sysctrl::SysCtrlPeriph, 
    gpio::GpioPortPeriph,
};
#[cfg(not(debug))] use crate::DSO_PORTS;
use core::{
    cmp::min,
    ptr,
};
#[cfg(not(debug))] use core::ptr::{read_volatile, NonNull};
#[cfg(debug)] use core::ptr::NonNull;
use drone_cortexm::reg::prelude::*;
use drone_tisl_map::periph::uart::{traits::*, UartMap};
use drone_tisl_map::periph::ioc::pads::{traits::*, IocPadMap, IocPadOverBits};
use drone_tisl_map::periph::ioc::selectors::{traits::*, IocSelectorMap};
use drone_tisl_map::periph::sysctrl::{traits::*, SysCtrlMap};
use drone_tisl_map::periph::gpio::{
    traits::*, 
    GpioPortMap, 
    GpioA, GpioB, GpioC, GpioD,
};

const KEY: u16 = 0b100_1011;
const MAX_PACKET_SIZE: usize = 16;

#[doc(hidden)]
pub unsafe trait Logger {
    type UartMap: UartMap;
    type IocPadMapTx: IocPadMap + IocPadOverBits;
    type IocPadMapRx: IocPadMap + IocPadOverBits;
    type IocSelectorMap: IocSelectorMap;
    type SysCtrlMap: SysCtrlMap; 

    const BAUD_RATE: u32;
    const BUF_SIZE: u32;

    fn buf() -> NonNull<u8>;
}

#[doc(hidden)]
#[must_use]
#[inline]
// This flag is set in 'just log'
#[cfg(not(debug))]
pub fn is_enabled<T: Logger>(port: u8) -> bool {
    unsafe { read_volatile(&DSO_PORTS) & 1 << port != 0 }
}
#[cfg(debug)]
pub fn is_enabled<T: Logger>(_port: u8) -> bool {
    true
}

#[doc(hidden)]
#[inline]
pub fn write_bytes<T: Logger>(port: u8, bytes: &[u8]) {
    for bytes in bytes.chunks(min(T::BUF_SIZE as usize - 2, MAX_PACKET_SIZE)) {
        unsafe { write_packet::<T>(port, bytes) };
    }
}

#[doc(hidden)]
// Returns current functional frequency for system clock.
pub fn get_clock_speed<T: SysCtrlMap>(periph: &mut SysCtrlPeriph<T>) -> u32 {
    let sta = periph.sysctrl_clock_sta.sys_div().read_bits();
    match sta {
        0x000 => { 32000000 }
        0x001 => { 16000000 }
        0x010 => { 8000000 }
        0x011 => { 4000000 }
        0x100 => { 2000000 }
        0x101 => { 1000000 }
        0x110 => { 500000 }
        0x111 => { 250000 }
        _ => { panic!("in get_clock_speed") }
    }
}

#[doc(hidden)]
// Busy wait until FIFO is empty.
pub fn flush<T: Logger>() {
    let periph = UartPeriph::<T::UartMap>::summon();
    // UART transmitter has started.
    if !is_init(&periph) || periph.uart_fr.txfe().read_bit() {
        return;
    } 
    // Last TX byte transmitted (FIFO empty).
    while !periph.uart_fr.txfe().read_bit() {
        // Tiny delay.
        for _ in 0..50 {
            unsafe { llvm_asm!("" :::: "volatile") };
        }
    }
}

#[doc(hidden)]
// Check if UART is enabled.
fn is_init<T: UartMap>(periph: &UartPeriph<T>) -> bool {
    periph.uart_ctl_uarten.read_bit()
}

// Set UARTx TX signal for the desired GPIO pin.
fn tx_uart_signal<T: IocPadMap + IocPadOverBits>(periph: &mut IocPadPeriph<T>, uart_selector: u8) {
    let selval;
    match uart_selector {
        0 => { selval = 0x0; }  // Signal is UART0 TXD 
        1 => { selval = 0x2; }  // Signal is UART1 TXD
        _ => { panic!("in tx_uart_signal"); }
    }
    periph.ioc_sel.store_reg(|r, v| {
        r.pad_sel().write(v, selval);
    });
}

// Configure TX pin for the output enable.
fn tx_output_enable<T: IocPadMap + IocPadOverBits>(periph: &mut IocPadPeriph<T>) {
    periph.ioc_over.modify_reg(|r, v| {
        r.pad_over_bits().write(v, 0x8);
    });
}

// Configure TX pin for a peripheral output function.
fn tx_mode_set<T: GpioPortMap>(periph: &mut GpioPortPeriph<T>, pin_selector: u8) {
    periph.gpio_dir.modify_reg(|r, v| {
        let mut val = r.dir().read(v);
        val |= (1 << pin_selector) as u32;
        r.dir().write(v, val);
    });
    periph.gpio_afsel.modify_reg(|r, v| {
        let mut val = r.afsel().read(v);
        val |= (1 << pin_selector) as u32;
        r.afsel().write(v, val);
    });
}

// Configure RX pin for a peripheral input function.
fn rx_mode_set<T: GpioPortMap>(periph: &mut GpioPortPeriph<T>, pin_selector: u8) {
    let mask = 0b1111111 ^ 1 << pin_selector;
    periph.gpio_dir.modify_reg(|r, v| {
        let mut val = r.dir().read(v);
        val &= mask as u32;
        r.dir().write(v, val);
    });
    periph.gpio_afsel.modify_reg(|r, v| {
        let mut val = r.afsel().read(v);
        val |= (1 << pin_selector) as u32;
        r.afsel().write(v, val);
    });
}

// Set the pad that is used for UARTx TX
fn rx_uart_pad_set<T: IocSelectorMap>(periph: &mut IocSelectorPeriph<T>, uart_selector: u8, port_selector: u8, pin_selector: u8) {
    let port_pin = (port_selector * 8) + pin_selector;
    match uart_selector {
        0 => {
            periph.ioc_uartrxd_uart0.modify_reg(|r, v| {
                r.input_sel().write(v, port_pin as u32);
            });
        }
        1 => {
            periph.ioc_uartrxd_uart1.modify_reg(|r, v| {
                r.input_sel().write(v, port_pin as u32);
            });
        }
        _ => panic!("in rx_uart_pad_set")
    }
}

fn disable_uart<T: UartMap>(periph: &mut UartPeriph<T>) {

    // Disable the UART0.
    // Wait for end of TX.
    while periph.uart_fr.busy().read_bit() {};
    // Disable the FIFO.
    periph.uart_lcrh.modify_reg(|r, v| {
        r.fen().clear(v);
    });
    // Disable the UART.
    periph.uart_ctl.modify_reg(|r, v| {
        r.uarten().clear(v);
        r.txe().clear(v);
        r.rxe().clear(v);
    });

    // Disable all UART module interrupts.
    periph.uart_im.modify_reg(|r, v| {
        r.rxim().clear(v);
        r.txim().clear(v);
        r.rtim().clear(v);
        r.feim().clear(v);
        r.peim().clear(v);
        r.beim().clear(v);
        r.oeim().clear(v);
        r.ninebitim().clear(v);
        r.lmsbim().clear(v);
        r.lme1im().clear(v);
        r.lme5im().clear(v);
    });

}

fn enable_uart<T: UartMap>(periph: &mut UartPeriph<T>, baud_rate: u32, clk_speed: u32) {
    let mut b_rate = baud_rate;
    // Is the required baud rate greater than the maximum rate supported
    // without the use of high speed mode?
    if (baud_rate * 16) > clk_speed {
        // Enable high speed mode.
        periph.uart_ctl.modify_reg(|r, v| {
            r.hse().set(v);
        });
        // Half the supplied baud rate to compensate for enabling high speed
        // mode.  This allows the following code to be common to both cases.
        b_rate = b_rate / 2;
    }
    else {
        // Disable high speed mode
        periph.uart_ctl.modify_reg(|r, v| {
            r.hse().clear(v);
        });
    }

    // Compute the fractional baud rate divider.
    let div = (((clk_speed * 8) / b_rate) + 1) / 2;

    // Set the baud rate.
    periph.uart_ibrd.modify_reg(|r, v| {
        r.divint().write(v, div / 64);
    });
    periph.uart_fbrd.modify_reg(|r, v| {
        r.divfrac().write(v, div % 64);
    });

    // Set parity, data length, and number of stop bits.
    periph.uart_lcrh.modify_reg(|r, v| {
        r.wlen().write(v, 0x3); // 8-bit word.
        r.pen().clear(v);       // Paritiy is disabled.
    });

    // Enable the FIFO.
    periph.uart_lcrh.modify_reg(|r, v| {
        r.fen().set(v);         // Enable the FIFO.
    });

    periph.uart_ctl.modify_reg(|r, v| {
        r.uarten().set(v);
        r.txe().set(v);
        r.rxe().set(v);
    });
}

unsafe fn write_packet<T: Logger>(port: u8, bytes: &[u8]) {

    #[cfg(feature = "std")]
    return;

    llvm_asm!("cpsid i" :::: "volatile");
    let mut uart_periph = UartPeriph::<T::UartMap>::summon();

    if !is_init(&uart_periph) {

        let ptr_uart = uart_periph.uart_dr.as_ptr() as u32;
        let uart_nr;
        match ptr_uart {
            0x4000C000 => { uart_nr = 0; }
            0x4000D000 => { uart_nr = 1; }
            _ => { panic!("invalid uart_nr"); }
        }

        let mut sys_ctrl_periph = SysCtrlPeriph::<T::SysCtrlMap>::summon();
        let mut tx_pad_periph = IocPadPeriph::<T::IocPadMapTx>::summon();
        let rx_pad_periph = IocPadPeriph::<T::IocPadMapRx>::summon();
        let mut iocsel_periph = IocSelectorPeriph::<T::IocSelectorMap>::summon();

        let offset_tx = (tx_pad_periph.ioc_sel.as_ptr() as usize - 0x400D4000 as usize) / 4;
        let gpio_nr_tx: u8 = (offset_tx / 8) as u8;
        let pin_nr_tx: u8 = (offset_tx % 8) as u8;

        let offset_rx = (rx_pad_periph.ioc_sel.as_ptr() as usize - 0x400D4000 as usize) / 4;
        let gpio_nr_rx: u8 = (offset_rx / 8) as u8;
        let pin_nr_rx: u8 = (offset_rx % 8) as u8;

        // Find current clock speed.
        let clk_speed = get_clock_speed(&mut sys_ctrl_periph);

        // Enable clock for selected UART.
        sys_ctrl_periph.sysctrl_rcgcuart.modify_reg(|r, v| {
            match uart_nr {
                0 => {
                    r.uart0().set(v);  // UART0
                }
                1 => {
                    r.uart1().set(v);  // UART1
                }
                _ => panic!("wrong uart selector")
            }
        });

        // Disable UART function and all UART module interrupt.
        disable_uart(&mut uart_periph); 

        //Set IO clock as UART clock source.  
        uart_periph.uart_cc.modify_reg(|r, v| {
            r.cs().write(v, 0x1);
        });

        // --- TX output ----
        // -------------------
        // Configure TX pin for a peripheral output function.
        // (modfies GPIO_DIR and GPIO_AFSEL).
        match gpio_nr_tx {
            0 => { 
                tx_mode_set(&mut GpioPortPeriph::<GpioA>::summon(), pin_nr_tx);
            }
            1 => { 
                tx_mode_set(&mut GpioPortPeriph::<GpioB>::summon(), pin_nr_tx);
            }
            2 => { 
                tx_mode_set(&mut GpioPortPeriph::<GpioC>::summon(), pin_nr_tx);
            }
            3 => { 
                tx_mode_set(&mut GpioPortPeriph::<GpioD>::summon(), pin_nr_tx);
            }
            _ => { panic!("invalid gpio_nr_tx"); }
        }

        // Set UARTx TX signal for the desired GPIO pin.
        // (modifies IOC_Pxx_SEL).
        tx_uart_signal(&mut tx_pad_periph, uart_nr);

        // Configure TX pin for the output enabled.
        // (modifies IOC_Pxx_OVER).
        tx_output_enable(&mut tx_pad_periph);

        // ---- RX input ----
        // ------------------
        // Set the pad that is used for UARTx TX
        // (modifes IOC_UARTRXD_UARTx)
        rx_uart_pad_set(&mut iocsel_periph, uart_nr, gpio_nr_rx, pin_nr_rx);

        // Configure RX pin for a peripheral input function.
        // (modfies GPIO_DIR and GPIO_AFSEL).
        match gpio_nr_rx {
            0 => { 
                rx_mode_set(&mut GpioPortPeriph::<GpioA>::summon(), pin_nr_rx);
            }
            1 => { 
                rx_mode_set(&mut GpioPortPeriph::<GpioB>::summon(), pin_nr_rx);
            }
            2 => { 
                rx_mode_set(&mut GpioPortPeriph::<GpioC>::summon(), pin_nr_rx);
            }
            3 => { 
                rx_mode_set(&mut GpioPortPeriph::<GpioD>::summon(), pin_nr_rx);
            }
            _ => { panic!("invalid gpio_nr_tx"); }
        }

        // Set UART operation mode and enable it.
        enable_uart(&mut uart_periph, T::BAUD_RATE, clk_speed);

        // Tiny delay to stabilize the periphery.
        for _ in 0..10000 {
            llvm_asm!("" :::: "volatile");
        }
    }
    flush::<T>();
    let mut buf_ptr = T::buf().as_ptr();
    let count = fill_buf(buf_ptr, port, bytes);
    for _n in 0..count {
        let ch: u32 = (*(buf_ptr)).into();
        // Spin if transmit FIFO is full
        while uart_periph.uart_fr.txff().read_bit() { };
        uart_periph.uart_dr.store_reg(|r, v| {  
            r.data().write(v, ch);  
        });  
        buf_ptr = buf_ptr.add(1);
    }
    llvm_asm!("cpsie i" :::: "volatile");
}

#[allow(clippy::cast_ptr_alignment)]
unsafe fn fill_buf(buf_ptr: *mut u8, port: u8, bytes: &[u8]) -> u32 {
    *(buf_ptr as *mut u16) = (KEY << 9 | u16::from(port) << 4 | (bytes.len() as u16 - 1)).to_be();
    ptr::copy_nonoverlapping(bytes.as_ptr(), buf_ptr.add(2), bytes.len());
    bytes.len() as u32 + 2
}
