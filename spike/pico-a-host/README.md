# Pico A USB host spike

Throwaway. Answers one question: *can a Rust + Embassy firmware on a Pico-class board enumerate a USB HID keyboard via the native USB host controller?*

## Bench rig

- Raspberry Pi Pico (RP2040) for the first attempt; Pico 2 (RP2350) once RP2040 works.
- Raspberry Pi Debug Probe wired to the target's SWD pads (SWCLK, SWDIO, GND).
- USB-A-to-micro-B OTG shim plugged into the target's micro-B port.
- A standard Raspberry Pi USB keyboard plugged into the OTG shim.
- The target is powered via the debug probe's UART pin block (or a separate 5 V to VBUS pad — see `wiring` below). **Do not** plug a host PC into the target's micro-B at the same time as the keyboard.

```
[Workstation] ── USB ──► [Debug Probe] ── SWD ──► [Pico target]
                                                     │ micro-B
                                                     ▼
                                               [OTG shim]
                                                     │ USB-A
                                                     ▼
                                            [Pi USB keyboard]
```

## Build & flash

```
rustup target add thumbv6m-none-eabi      # RP2040
# rustup target add thumbv8m.main-none-eabihf   # RP2350, when we get there
cargo install probe-rs --features cli     # if not already
cargo run --release
```

`cargo run` invokes `probe-rs run` (configured in `.cargo/config.toml`), which flashes via SWD and streams `defmt` output back over RTT.

## Switching to RP2350

When ready:

1. In `Cargo.toml`, change `embassy-rp` feature `rp2040` → `rp235xa`.
2. In `.cargo/config.toml`, change the default target to `thumbv8m.main-none-eabihf`.
3. Replace `memory.x` with the RP2350 layout.
4. `cargo run --release`.

## Caveats

- `embassy-usb-host` 0.1.0 was published 2026-05-03; `[patch.crates-io]` pins everything to embassy git `main` so the host driver is actually present.
- Driver is full-speed only; some keyboards are low-speed and will not enumerate directly.
- Some RP2350 boards have a D+ pull-up that prevents host operation. If the RP2350 step fails to detect a connection, check the board's USB schematic.
- Expect breakage from upstream churn; embassy `main` is not stable.

## Verdict

Recorded in `changes/open/derisk-pico-a-usb-host.md`'s Conclusion when the spike concludes.
