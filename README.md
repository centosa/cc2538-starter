# cc2538-starter
The Drone-OS toolchain setup for Texas Instruments CC2538 System-on-Chip Solution for 2.4-GHz IEEE 802.15.4 and ZigBee®/ZigBee IP® Applications.

# Prerequisites:
- Dev board with the CC2538 processor. Examples: Zolertia FireFly (tested), Zolertia Re-Mote (not tested), Texas Instruments 2538DK (not tested).
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
