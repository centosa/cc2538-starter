# cc2538-starter
The Drone-OS toolchain setup for Texas Instruments CC2538 System-on-Chip Solution for 2.4-GHz IEEE 802.15.4 and ZigBee®/ZigBee IP® Applications.

# Prerequisites:
- Dev board with the CC2538 processor. Examples: Zolertia FireFly (tested), Zolertia Re-Mote (tested), Texas Instruments 2538DK (not tested).
- Segger J-Link.
- For debug/log: UART-to-USB adapter and cable to receive log output via a virtual COM-port on your computer.

# Usage
Clone the repository inside a dedicated workspace folder like this:

```bash
mkdir mycc2538
cd mycc2538
git clone https://github.com/centosa/cc2538-starter.git
```

Inside the workspace folder, clone the latest Drone-OS crates into the sub-directory 'drone-os':

```bash
mkdir drone-os
cd drone-os
git clone https://github.com/drone-os/drone.git -b "ahead"
git clone https://github.com/drone-os/drone-core.git -b "ahead"
git clone https://github.com/drone-os/drone-cortexm.git -b "ahead"
git clone https://github.com/drone-os/drone-tisl-map.git -b "ahead"
git clone https://github.com/drone-os/drone-svd.git -b "ahead"
```
You will have to replace your currently installed `drone` utility with the most recent one:
```bash
cd mycc2538/drone-os/drone
just install
```

And then the following to compile and flash it to your Zolertia FireFly board (no other boards tested yet):

```bash
cd mycc2538/cc2538-starter
just flash

To receive logging messages (needs the UART-to-USB adapter connected to GND and pin PD2 as TX output:
just log

To debug interactively:
just gdb
If you want to single-step through your code, you should copy the Justfile.debug to Justfile and recompile.
With the release version, the binary is highly optimized and gdb won't catch it.
```
## Configuring the DSO API (Drone Serial Logging)
As the CC2538 has no SWO module, DSO is our solution for sending logging output to your computer via UART-USB adapter.
In this example. logging output is done to pin PD2, and PD0 for input. Altough the input pin is not used by DSO, it must be configured.

If your dev board does not allow you to access pin PD2 and PD0, you can re-configure the pins easlily. It is all done in the file src/lib.rs:

Example: you have the Re-MOTE board and want to use the following pins (a possible selection for Re-MOTE):
- PC4 for TX (output)
- PA0 for RX (input, inactive)

```rust
// Create the logger.
drone_cc2538_dso::set_log! {
    uart_ty: Uart0,      // One of Uart0, Uart1.
    pad_ty_tx: IocC4,    // Output pad type, range IocA0 .. IocD7.
    pad_ty_rx: IocA0,    // Input pad type, range IocA0 .. IocD7.
    baud_rate: 115200,   // Transmission speed.
}
```

One more change is needed: The DSO crate needs exclusive access to specific registers, which are no longer available to the application code. A registration macro in src/lib.rs grants access. The pin related registers reserved for the DSO are listed in the exclusion list, like, for instance, `!ioc_pa0_sel`. It means: the application code has no access to IOC_PA0_SEL register because the DSO crate is managing it.

```rust
tisl_reg_tokens! {
    /// A set of tokens for all memory-mapped registers.
    pub struct Regs;

    !scb_ccr;
    !mpu_type; !mpu_ctrl; !mpu_rnr; !mpu_rbar; !mpu_rasr;

    !uart0_ctl; !uart0_ibrd; !uart0_fbrd; !uart0_lcr; !uart0_fr; !uart0_dr; !uart0_im; !uart0_cc;
    !ioc_pa0_sel; !ioc_pa0_over; !ioc_pc4_sel; !ioc_pc4_over;    
    !ioc_uartrxd_uart0;
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
