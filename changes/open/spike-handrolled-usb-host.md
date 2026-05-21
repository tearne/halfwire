# Spike a hand-rolled minimal USB host

**Mode:** TBD

## Intent

Find out whether HalfWire can stand on a hand-rolled minimal USB host in Rust, scoped to exactly the case it needs: a single low-speed boot-protocol HID keyboard, directly attached, full duplex with the host PC mediated by Pico B. No hubs, no high-speed, no isochronous, no class drivers beyond boot keyboard.

The motivation is auditability. The recent polling-cadence investigation showed the dependency stack we're sitting on (`embassy-usb-host` plus `embassy-rp`'s host driver) is bleeding-edge and carries `FIXME` paths in security-adjacent code. For a project whose ethos is "any subsystem auditable in an evening", small-app-on-top-of-a-bleeding-edge-dependency is a weaker claim than small-app-on-top-of-our-own-deliberately-narrow-implementation. The C+TinyUSB alternative is mature but walks back the Rust intent on Pico A; this spike asks whether we can match its conceptual simplicity in Rust by accepting that we write the small narrow stack ourselves rather than depending on a general one.

The deliverable is a verdict — viable, viable but more work than expected, or not viable — supported by a working spike or a clear account of where the wall is. Boundaries: the spike runs on a Pico 2 (RP2350), enumerates the Holtek keyboard the bench rig already has, and prints decoded keyboard reports over `defmt`. It does not include the diode wiring, Pico B, the validating filter, or any work toward integration. TinyUSB is a reference for what to do, not a dependency.

## Approach

### Reader profile

Competent Rust dev, new to embedded and to USB. Rust is not the obstacle; chip plumbing and USB protocol state are. Every choice judged on whether it makes those smaller.

### Code that reads, not code that performs

Named helper functions, shallow nesting, one responsibility each. Comments explain registers and USB stages, not Rust syntax. Obvious beats clever.

### No async, no executor

Async hides state-machine flow, which is what the reader needs to see. Synchronous `loop { host.poll(); ... }`, all state in structs, all side-effects in `poll()` — same shape as TinyUSB's `tuh_task()`, which is what we're auditing-by-comparison against.

### Substrate

`cortex-m-rt`, `cortex-m`, `rp-pac`, `rp235x-hal::clocks` (one call to bring up `PLL_USB` at 48 MHz — chip plumbing, not USB code). `defmt` + `defmt-rtt` for spike-lifetime logging.

### USB via rp-pac directly

Register-level. TinyUSB's `hcd_rp2040.c` is the line-by-line reference; the RP2350 USB block reuses the RP2040 IP. Same state machine, same register writes, same control flow. Urges to "improve" the structure are yellow flags — the goal is a Rust translation a reader can verify against TinyUSB.

### Hardcode HalfWire's case

One low-speed device, address 1, single configuration, single HID interface, boot protocol, single interrupt-IN endpoint. No hubs, composites, high-speed paths, or non-boot class drivers.

### Liveness signal

On-board LED flashes on each report arrival, alongside `defmt`.

### Layout and effort

New project at `spike/pico-a-host-handrolled/`. Open-ended; only a wall turns the spike red.

## Plan

### Topics

- Project skeleton at `spike/pico-a-host-handrolled/` — RP2350 target, `cortex-m-rt`, `rp-pac`, `rp235x-hal::clocks`, `defmt-rtt`, no embassy crates.
- Substrate brought up: blink + `defmt` log running on Pico 2 with `PLL_USB` at 48 MHz.
- USB controller in host mode: powered, DPRAM cleared, connection event detected on plug-in.
- Device reset and `SET_ADDRESS` to address 1.
- Device and configuration descriptors fetched; HID interrupt-IN endpoint and its `bInterval` extracted.
- HID configured: `SET_CONFIGURATION`, `SET_PROTOCOL(boot)`, `SET_IDLE(0, 0)`.
- Interrupt-IN endpoint pipe set up using the keyboard's actual `bInterval`; polling loop reading and logging boot-keyboard reports.
- LED flashes on each report arrival.
- Verdict recorded in this document's Conclusion, with reasoning and (on green) a reference to the spike code, or (on red) the wall hit.

### Done when

Keypresses arrive promptly as decoded boot-keyboard reports with the LED responding, and the code reads cleanly against the reader profile — *or* a specific wall is documented.

