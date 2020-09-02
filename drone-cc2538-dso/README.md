[![crates.io](https://img.shields.io/crates/v/drone-nrf91-dso.svg)](https://crates.io/crates/drone-nrf91-dso)
![maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# drone-nrf91-dso

Nordic Semi nRF91 DSO implementation for Drone, an Embedded Operating System.

**Warning**: currently logging operations are wrapped in critical sections
(temporarily disabling all interrupts.) This can impact the operation of
your program, but only after you run `drone probe log`. It will be fixed in
the future by using proper synchronization methods with minimal changes to
the API.

## Documentation

- [Drone Book](https://book.drone-os.com/)
- [API documentation](https://api.drone-os.com/drone-nrf91-dso/0.12/)

## Usage

Place the following to the Cargo.toml:

```toml
[dependencies]
drone-nrf91-dso = "0.12.1"
```

Exclude the following tokens from your register token index (change "uarte0"
if you choose a different peripheral):

```rust
nrf_reg_tokens! {
    struct Regs;

    !uarte0_ns_tasks_startrx; !uarte0_ns_tasks_stoprx; !uarte0_ns_tasks_starttx;
    !uarte0_ns_tasks_stoptx; !uarte0_ns_tasks_flushrx; !uarte0_ns_subscribe_startrx;
    !uarte0_ns_subscribe_stoprx; !uarte0_ns_subscribe_starttx; !uarte0_ns_subscribe_stoptx;
    !uarte0_ns_subscribe_flushrx; !uarte0_ns_events_cts; !uarte0_ns_events_ncts;
    !uarte0_ns_events_rxdrdy; !uarte0_ns_events_endrx; !uarte0_ns_events_txdrdy;
    !uarte0_ns_events_endtx; !uarte0_ns_events_error; !uarte0_ns_events_rxto;
    !uarte0_ns_events_rxstarted; !uarte0_ns_events_txstarted; !uarte0_ns_events_txstopped;
    !uarte0_ns_publish_cts; !uarte0_ns_publish_ncts; !uarte0_ns_publish_rxdrdy;
    !uarte0_ns_publish_endrx; !uarte0_ns_publish_txdrdy; !uarte0_ns_publish_endtx;
    !uarte0_ns_publish_error; !uarte0_ns_publish_rxto; !uarte0_ns_publish_rxstarted;
    !uarte0_ns_publish_txstarted; !uarte0_ns_publish_txstopped; !uarte0_ns_shorts; !uarte0_ns_inten;
    !uarte0_ns_intenset; !uarte0_ns_intenclr; !uarte0_ns_errorsrc; !uarte0_ns_enable;
    !uarte0_ns_psel_rts; !uarte0_ns_psel_txd; !uarte0_ns_psel_cts; !uarte0_ns_psel_rxd;
    !uarte0_ns_baudrate; !uarte0_ns_rxd_ptr; !uarte0_ns_rxd_maxcnt; !uarte0_ns_rxd_amount;
    !uarte0_ns_txd_ptr; !uarte0_ns_txd_maxcnt; !uarte0_ns_txd_amount; !uarte0_ns_config;
}
```

Set up the logger:

```rust
drone_nrf91_dso::set_log! {
    periph: Uarte0S, // Peripheral from `drone_nrf_map::periph::uarte`
    pin_number: 29,  // Output pin number
    buf_size: 64,    // Output buffer size in bytes
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
