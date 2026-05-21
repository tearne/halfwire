// Pico A USB host spike, RP2350 variant. Same firmware as the RP2040 spike — only the
// chip target, memory layout, and build glue differ. The RP2350 example tree in embassy
// has no host-mode example yet, so this is the first attempt at confirming that
// embassy-usb-host works on RP2350 in practice.

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_usb_host::class::hid::{HidHost, KeyboardReport, PROTOCOL_BOOT};
use embassy_usb_host::{BusRoute, BusState};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => embassy_rp::usb::host::InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let driver = embassy_rp::usb::host::Driver::new(p.USB, Irqs);

    static BUS_STATE: BusState = BusState::new();
    let (mut bus_ctrl, bus) = embassy_usb_host::bus(driver, &BUS_STATE);

    info!("USB host initialised (RP2350), waiting for device...");

    loop {
        let speed = bus_ctrl.wait_for_connection().await;
        info!("Device connected at speed {:?}", speed);

        let mut config_buf = [0u8; 256];
        let (enum_info, config_len) = match bus.enumerate(BusRoute::Direct(speed), &mut config_buf).await {
            Ok(r) => r,
            Err(e) => {
                error!("Enumeration failed: {:?}", e);
                continue;
            }
        };

        info!(
            "Enumerated: VID={:04x} PID={:04x} addr={}",
            enum_info.device_desc.vendor_id, enum_info.device_desc.product_id, enum_info.device_address
        );

        let mut hid = match HidHost::new(&bus, &config_buf[..config_len], &enum_info) {
            Ok(h) => h,
            Err(e) => {
                error!("HID init failed: {:?}", e);
                continue;
            }
        };

        if let Err(e) = hid.set_protocol(PROTOCOL_BOOT).await {
            error!("SET_PROTOCOL(BOOT) failed: {:?}", e);
            continue;
        }
        if let Err(e) = hid.set_idle(0, 0).await {
            error!("SET_IDLE failed: {:?}", e);
            continue;
        }

        info!("HID device ready (boot protocol), reading reports...");

        let mut buf = [0u8; 8];
        loop {
            match hid.read(&mut buf).await {
                Ok(n) if n >= 8 => {
                    if let Some(report) = KeyboardReport::parse(&buf[..n]) {
                        info!("Keyboard report: mods={:08b} keys={:?}", report.modifiers, report.keycodes);
                    } else {
                        info!("Short report ({} bytes): {:x}", n, &buf[..n]);
                    }
                }
                Ok(n) => info!("HID report ({} bytes): {:x}", n, &buf[..n]),
                Err(e) => {
                    error!("HID read failed: {:?}", e);
                    break;
                }
            }
        }

        info!("Device disconnected, waiting for next...");
    }
}
