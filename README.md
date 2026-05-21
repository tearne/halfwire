# HalfWire

An inline USB gadget that lets a trusted HID device reach a host while preventing the host from reaching back.

**Status:** planning / pre-implementation
**Licence (intended):** Apache 2.0 (code), CC-BY 4.0 (docs/hardware)

## What it is

```
[Trusted HID] --USB--> [Pico A: USB host] --UART (TX only)--> [Pico B: USB device] --USB--> [Host]
```

Two RP2350 microcontrollers back-to-back. The UART between them has one direction *physically not wired* — the data-diode property is hardware, not firmware. Pico A enumerates the upstream HID keyboard and validates each report (alphanumeric/punctuation only, plausible typing rates, no modifier-only or OS-shortcut sequences) before forwarding. Pico B presents a generic HID keyboard to the host. Half a wire, hence the name.

## Wiring

Two Raspberry Pi Pico 2 boards (RP2350, non-wireless). Both Picos use their native micro-B USB ports — Pico A's faces the trusted device (via a USB-A-to-micro-B OTG shim); Pico B's faces the laptop. UART0 (remapped) carries the link on Pico A's right-edge pins and Pico B's left-edge pins, so the wires run straight across — only Pico A's TX is wired to Pico B's RX, the return direction is left as bare board.

```
┌─────────┐ USB ┌────────┐                            ┌────────┐ USB ┌────────┐
│ Trusted │◄═══►│ Pico A │ GP16(p21) ────► GP13(p17)  │ Pico B │◄═══►│ Laptop │
│   HID   │     │        │ GP17(p22) ╶╶╶╶╶ GP12(p16)  │        │     │        │
└─────────┘     │        │ GND/(p23) ───── GND/(p18)  │        │     └────────┘
                │        │ VBUS/(p40)◄─5V─ VBUS/(p40) │        │
                └────────┘                            └────────┘
```

Pin numbers are board-physical (Pico 2's 40-pin header); both boards share the same pinout.

`────►` is the one TX wire; `╶╶╶╶` marks the deliberately absent return. The laptop powers Pico B over USB; Pico B's VBUS feeds Pico A, which in turn supplies the trusted device.

## Why

A USB device is exposed to whatever runs on its host through the same wire it uses for input. Software mitigations narrow that surface; HalfWire removes it. With no return path, the upstream device cannot be probed, reflashed, or exploited from the host — and the property survives full firmware compromise of Pico B.

## Threat model

**Defends against:** host-originated attacks on the trusted device over USB; downstream firmware compromise; malformed HID reports.

**Does not defend against:** host-side keyloggers (inherent to emitting via the keyboard channel); physical tampering with HalfWire; supply-chain compromise; sophisticated adversaries with lab equipment.

## Auditability

The firmware on each side should be small and plain enough for a security-minded reader to audit in an evening. Simplicity is a security property here, not a stylistic preference: every line is attack surface for the thing HalfWire exists to protect.

## Non-goals

HID keyboards only. No UI, no storage, no signing, no general USB passthrough, no certification claim. Feature creep is the enemy of an auditable filter.

