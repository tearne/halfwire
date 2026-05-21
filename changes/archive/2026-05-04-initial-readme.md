# Initial README

**Mode:** Wander

## Intent

Establish the project's initial README so HalfWire is legible to a first-time reader without external context.

HalfWire is an inline USB gadget that lets a trusted USB HID device be reached by a host computer while preventing the host from reaching back. Two RP2350 microcontrollers sit back-to-back; the link between them is a UART with one direction physically not wired, so the data-diode property is a fact about the hardware rather than a promise from firmware. The downstream MCU validates HID reports — alphanumeric and punctuation only, plausible typing rates, no modifier-only or OS-shortcut sequences — and forwards them as small framed messages. The host sees a generic USB keyboard; the upstream device sees nothing of the host.

The README should stand on its own — no reference to any specific upstream device. It should cover what HalfWire is, why it exists, how the one-way property is enforced, the threat model (and its limits — host-side keyloggers, physical tampering, supply chain), explicit non-goals (no UI, no storage, no general USB passthrough), and enough about the architecture and intended scope that a security-minded reader can decide whether to engage further.

Status is planning / pre-implementation; intended licences are Apache 2.0 for code and CC-BY 4.0 for docs/hardware; the repository is public from day one.

## Conclusion

README written at `README.md`. The first draft came in long (~190 lines, full sections for architecture, roadmap, success criteria, references); the user asked for terseness and the file was cut to roughly 40 lines. The roadmap was then dropped entirely — premature for a planning-stage project — and a short **Auditability** section was added to make explicit that small, plain code is itself a security property.

Shape that survived: opening diagram, diode-is-hardware framing, threat-model split into defended / accepted-risks, non-goals, glossary. No map exists yet; nothing to catch up.

