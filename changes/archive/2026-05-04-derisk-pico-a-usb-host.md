# Derisk Pico A USB host

**Mode:** Explore

## Intent

Establish whether HalfWire's Rust-everywhere goal is feasible by answering one question: *can a Raspberry Pi Pico 2 (RP2350) running Rust + Embassy enumerate a USB HID keyboard via its native USB host controller, today?*

This is the highest-risk part of the build. If the answer is yes, the rest of HalfWire (validating filter, UART framing, Pico B device-mode HID) is mechanical. If the answer is no, the project needs to revisit its language choice or its architecture before further investment.

The deliverable is a binary go/no-go decision, supported by either a working spike or a clear account of what blocks it. Anything beyond that — the validating filter, the diode wiring, Pico B, the OTG shim integration as a permanent fixture — is out of scope.

## Approach

### Two phases, gated

First a short desk-research pass on Embassy's RP2350 USB host support — relevant crates, examples, open issues. If the picture is "no working path", the spike doesn't start; we reach a red decision without burning hardware time.

### Single-Pico bench rig

One Pico 2 + the OTG shim + a standard Raspberry Pi USB keyboard. No Pico B, no UART link, no validation. Smallest setup that can answer the question.

### SWD programming, separate UART for logs

The Pico's micro-B is consumed by the host-mode port, so flashing goes over SWD via the existing debugprobe and logs ride a separate UART to a USB-serial adapter on the workstation. Avoids BOOTSEL toggling and frees the host port entirely.

### Green / red

Green = at least one keypress from the keyboard decoded into an HID report and printed. Anything less is red.

## Plan

### Topics

- Embassy RP2350 USB host: current state of the art — crates, examples, open issues, gating bugs.
- Bench rig: Pico 2, OTG shim, Pi keyboard, debugprobe, USB-serial adapter wired and verified end-to-end with a trivial firmware.
- Rust + Embassy project skeleton with `defmt` over UART (or RTT, whichever fits the rig) and SWD flashing through the debugprobe.
- USB host enumeration: keyboard recognised and its device descriptor logged.
- HID report capture: at least one keypress parsed and logged as an HID report.
- Verdict recorded in this document's Conclusion, with the working code (or the blocker) attached.

### Done when

A green or red verdict is recorded, with enough detail — working firmware on green, a specific blocker on red — to act on next.

## Log

- Desk-research topic complete. `embassy-usb-host` 0.1.0 was published 2026-05-03 (one day before this change opened). Driver lives in `embassy-rp` git `main` (not in the 0.10.0 crates.io release). Working RP2040 example at `examples/rp/src/bin/usb_host_keyboard.rs`; no rp235x example yet. Verdict: viable but bleeding-edge.
- Surprise risk: some RP2350 boards (per qsantos.fr 2025-11-21) carry an R13 D+ pull-up that prevents host operation and must be desoldered. Need to check whether a stock Pico 2's micro-B path is affected; the OTG shim is passive so it doesn't add or remove this.
- Suggested spike sequencing from research: prove the toolchain on RP2040 first using the existing example, then port to RP2350. Reduces the surface where things can be wrong on first attempt. User has an RP2040 to hand, so we're going staged.
- Project skeleton scaffolded at `spike/pico-a-host/`. Cargo.toml pins all embassy crates to git `main` via `[patch.crates-io]` because `embassy-usb-host` 0.1.0's matching `embassy-rp` host driver is not in the published 0.10.0 release. `src/main.rs` is a near-direct copy of embassy's `examples/rp/src/bin/usb_host_keyboard.rs`. Targets RP2040 first; switch to RP2350 documented in the spike's README.
- Initial bring-up needed two fixes the example's tree provides but a hand-built skeleton has to recreate: a BOOT2 region in `memory.x`, and the `-Tlink-rp.x` linker arg (along with `-Tlink.x` and `-Tdefmt.x`) injected from `build.rs` rather than `.cargo/config.toml`'s rustflags. With those in place a blink + defmt sanity firmware ran cleanly on RP2040, confirming the rig (probe-rs, RTT, defmt, embassy executor) end-to-end.
- Power topology: the user's setup is a Pico 2 acting as debug probe powering the target Pico 1 via a VSYS↔VSYS bridge. That left target-Pico VBUS at 0 V (the on-board Schottky blocks VSYS→VBUS), so the OTG shim's keyboard pin had no power. Fix: switch the bridge to VBUS↔VBUS (pin 40↔pin 40). Power for the target chip still reaches it via the diode (VBUS→VSYS). Worth surfacing prominently in the v1 hardware notes.
- First enumeration attempt detected a hub (VID=05e3 PID=0610, Genesys Logic) — the official Raspberry Pi keyboard has a built-in USB hub; the keyboard sits behind it and the example's `BusRoute::Direct` cannot see past the hub. Confirms `embassy-usb-host` does enumerate at the bus level, but also flags hub-traversal as deferred work for v1.
- A second, plain non-hub keyboard (VID=04d9 PID=4545, Holtek, low-speed) enumerated cleanly. Boot-protocol set via `HidHost::set_protocol(PROTOCOL_BOOT)` then `set_idle(0, 0)`, and `KeyboardReport::parse` decoded real keypresses into modifiers + keycodes. **Verdict on RP2040: green.**
- Polling cadence anomaly: reports arrive at exactly 1024 ms intervals instead of the change-driven cadence `SET_IDLE(0, 0)` is documented to produce. Not a blocker for the verdict; flagged as a quality issue for v1.
- RP2350 spike scaffolded as a separate project at `spike/pico-a-host-rp2350/` (different memory.x with start/end blocks, `rp235xa` + `binary-info` features, `thumbv8m.main-none-eabihf` target, `link-rp.x` not used). Initially appeared not to enumerate, but on a subsequent attempt with the same wiring it came up cleanly: Holtek keyboard enumerated at low speed, boot protocol set, real keypresses decoded into `KeyboardReport`. The earlier failure was transient (reseating wires resolved it). Same 1024 ms polling cadence as RP2040, so the quirk is upstream-of-chip and platform-agnostic. **Verdict on RP2350: green.**

## Conclusion

Both verdicts green: Rust + Embassy + native USB host enumerates a HID keyboard on RP2040 *and* RP2350. Working spikes preserved at `spike/pico-a-host/` and `spike/pico-a-host-rp2350/`.

Notable findings worth carrying into v1 planning:

- **Power topology.** The keyboard's USB peripheral side gets 5 V from the target Pico's VBUS rail. Bridges between Picos must be VBUS↔VBUS (pin 40↔pin 40), not VSYS↔VSYS — the on-board Schottky blocks the wrong direction. The HalfWire wiring diagram in the README already specifies pin 40↔pin 40, but this spike confirmed empirically why it has to be that way.
- **Polling cadence quirk.** Reports arrive at exactly 1024 ms intervals on both chips, regardless of `SET_IDLE(0, 0)`. Means fast typing drops keys today. Real bug somewhere — driver, idle handling, or our usage of the API. Investigate before v1.
- **Hub traversal.** The official Raspberry Pi keyboard sits behind an integrated USB hub; the example's `BusRoute::Direct` only sees the hub. Whatever the eventual upstream HID device for HalfWire is, plan for hub support — `embassy-usb-host` exposes hooks for it but the scaffolding work is non-trivial.
- **`embassy-usb-host` is two days old at the time of this spike.** Bleeding-edge with churn risk; v1 should pin a known-good revision in `[patch.crates-io]` rather than tracking `main` indefinitely.




