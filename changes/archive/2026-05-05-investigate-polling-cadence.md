# Investigate polling cadence

**Mode:** Explore

## Intent

Find out why HID reports arrive at exactly 1024 ms intervals on both spikes (RP2040 and RP2350) instead of the change-driven cadence `SET_IDLE(0, 0)` is documented to produce. The current behaviour drops keystrokes at normal typing speed; HalfWire is unusable as a daily driver until reports come through promptly after each press and release.

The investigation should determine which of three causes is responsible:

- A bug in `embassy-usb-host`'s handling of `set_idle` or its interrupt-endpoint polling.
- Our own misuse of the API — wrong parameters, wrong order, missing call.
- A behaviour of the specific keyboard (Holtek VID=04d9 PID=4545) that masks an otherwise correct host implementation.

The deliverable is a fix or a clear next step (an upstream issue with a minimal reproducer; a workaround we can carry; a known limitation to document). The fix itself is not in scope unless it turns out to be ours.

## Approach

### Source first, hardware second

Read embassy-usb-host's interrupt-pipe and `set_idle` handling before touching the rig. This is faster than scope-on-the-wire and usually narrows the hypothesis tree to one branch.

### Decode the keyboard's bInterval

The HID interrupt-IN endpoint descriptor in the configuration data already logged carries a `bInterval`. If 1024 ms matches `bInterval × 4` or some power-of-2 multiplier, the cadence is the keyboard's polling interval being honoured (or mishonoured) — not an idle-suppression bug.

### Strip back to the literal example

The example we adapted didn't call `set_protocol` or use `KeyboardReport::parse`. Run the literal example briefly to see whether the 1024 ms cadence is present there too. Same cadence → our additions are not the cause. Different → bisect.

### Outcomes

- Fault is ours → fix in the spike, retest.
- Fault is the driver → minimal reproducer prepared; whether to file upstream or carry a local workaround is decided when we get there, depending on the shape of the fix.
- Fault is the keyboard's quirk → document as a known limitation, since we can't cross-check without a second device today.

## Plan

### Topics

- Read `embassy-usb-host`'s interrupt-pipe and `SET_IDLE` paths in source; identify how the polling cadence is set and what `set_idle(0, 0)` actually does on the wire.
- Decode the keyboard's HID interrupt-IN endpoint `bInterval` from the configuration descriptor already in `config_buf` (log it from the spike); compare against the observed 1024 ms.
- Re-run the spike with the literal embassy example body (no `set_protocol`, no `KeyboardReport::parse`); compare cadence to the current adapted firmware.
- Conclude: fault location identified — ours, driver, or keyboard — with enough specificity to act per the Approach's Outcomes.
- Act: fix locally, draft an upstream reproducer, or record the limitation. Whichever it is, capture the reasoning in the Conclusion.

### Done when

Either the cadence is fixed (reports arrive promptly on press/release in the spike), or root cause is identified to a single source with a documented next step.

## Log

- Source-read first topic complete. Two-layer bug: (a) `embassy-usb-host`'s `HidHost::new` allocates the interrupt-IN pipe with `EndpointInfo { interval_ms: 0, .. }` hardcoded — the keyboard's `bInterval` is parsed from the descriptor but never threaded through. (b) `embassy-rp/src/usb/host.rs` then computes `let interval = self.interval as u32 - 1;` (with a self-acknowledging `// FIXME: host_poll_interval (bits 16:25)` comment), which underflows from 0 in release mode to `0xFFFF_FFFF`, masks into a 10-bit field, and produces 1023 — a 1023-USB-frame poll interval ≈ the 1024 ms we measured.
- Verdict: fault is in the driver stack, not our usage and not the keyboard. Bug is well-understood and small.
- Direction shift: the user opted not to maintain a fork or wait on upstream, but to spike a hand-rolled minimal USB host in Rust scoped to the low-speed-single-device-boot-keyboard case HalfWire needs. The polling fix happens by avoidance.

## Conclusion

Investigation complete. The 1024 ms cadence is a known-incomplete polling-interval path in `embassy-rp`'s host driver, fed by `embassy-usb-host` passing `interval_ms: 0` from `HidHost::new` (`bInterval` parsed but not threaded through). Both layers documented in the Log with file references. No fix applied; how the project responds is left open.

