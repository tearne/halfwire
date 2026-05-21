# Pico A USB host — hand-rolled spike

A from-scratch minimal USB host for the case HalfWire actually needs: one low-speed boot-protocol HID keyboard, directly attached. Built on `rp235x-hal` for chip plumbing, with USB written against `rp-pac` registers using TinyUSB's `hcd_rp2040.c` as a line-by-line reference. Synchronous, no async, no executor.

## Bench rig

Same as the previous spikes:

- Pico 2 (RP2350) as the target.
- Debug Probe over SWD.
- USB-A-to-micro-B OTG shim into the Pico 2's micro-B port.
- Plain non-hub USB keyboard into the OTG shim.
- VBUS↔VBUS bridge (pin 40↔pin 40) from the powering Pico across to the target Pico's VBUS pad.

## Build & flash

```
rustup target add thumbv8m.main-none-eabihf
cargo run --release
```

## Status

This file is updated as the spike proceeds.

- [x] Substrate brought up: blink + `defmt`, PLL_USB at 48 MHz.
- [ ] USB controller in host mode, connection event detected.
- [ ] Device reset and address assignment.
- [ ] Device + configuration descriptors fetched.
- [ ] HID configured (boot protocol, idle 0).
- [ ] Interrupt-IN polling at the keyboard's `bInterval`.
- [ ] LED flashes on each report.
- [ ] Verdict.
