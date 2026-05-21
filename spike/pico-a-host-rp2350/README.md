# Pico A USB host spike — RP2350 variant

Same shape as `../pico-a-host/` but targeting the Raspberry Pi Pico 2 (RP2350). Created as a separate project so the green RP2040 result stays available as a reference; this one is allowed to fail or break without disturbing it.

## Differences from the RP2040 spike

- `Cargo.toml` — `embassy-rp` features include `rp235xa` and `binary-info` instead of `rp2040`.
- `.cargo/config.toml` — target `thumbv8m.main-none-eabihf`, runner `probe-rs run --chip RP235x`.
- `memory.x` — RP2350 layout (512 KB striped RAM, SRAM8/9, boot ROM start/end blocks).
- `build.rs` — drops `-Tlink-rp.x` (boot setup is handled inside `memory.x` via `INSERT` directives on RP2350).

## Setup

```
rustup target add thumbv8m.main-none-eabihf
cargo run --release
```

Same bench rig as the RP2040 spike — debug probe to the Pico 2's SWD pads, OTG shim into its micro-B port, plain non-hub keyboard into the OTG shim, **VBUS↔VBUS** (pin 40 ↔ pin 40) bridge from the powering Pico across to this one's VBUS pad so the keyboard gets 5 V.

## Watch out for

- **embassy-usb-host is not yet exercised in the rp235x example tree.** This spike is the first attempt; expect bugs. Compile errors most likely point at the host driver's RP2350 path (it lives in the same `embassy-rp/src/usb/host.rs` as RP2040 but is unverified there).
- **D+ pull-up on certain RP2350 boards.** A stock Pico 2's micro-B path should be fine, but if connection is never detected, this is the first thing to check.
- **Driver is full-speed only.** Same caveat as the RP2040 spike. The plain non-hub keyboard that worked on RP2040 was low-speed and *did* enumerate directly, so this is more of a heads-up than a hard limit.

## Verdict

Recorded in `changes/open/derisk-pico-a-usb-host.md`'s Conclusion when the spike concludes.
