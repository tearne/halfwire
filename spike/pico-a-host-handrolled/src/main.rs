//! Pico A USB host — hand-rolled spike. Substrate-only at this point: brings the
//! Pico 2 up with `rp235x-hal`, configures clocks (including PLL_USB at 48 MHz),
//! and blinks the on-board LED while logging via `defmt`. Once this runs end-to-end
//! the USB host code goes here.

#![no_std]
#![no_main]

use defmt_rtt as _;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use panic_probe as _;
use rp235x_hal as hal;

/// Tells the RP2350 boot ROM that this image is a plain (unsigned) executable.
#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

/// Pico 2 carries a 12 MHz crystal.
const XTAL_FREQ_HZ: u32 = 12_000_000;

#[hal::entry]
fn main() -> ! {
    let mut pac = hal::pac::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // PLL_SYS for the core, PLL_USB at 48 MHz for the USB peripheral.
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();

    let mut timer = hal::Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks);
    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS);
    let mut led = pins.gpio25.into_push_pull_output();

    defmt::info!("Substrate up. PLL_USB at 48 MHz. Blinking.");

    let mut tick: u32 = 0;
    loop {
        led.set_high().unwrap();
        timer.delay_ms(250);
        led.set_low().unwrap();
        timer.delay_ms(250);
        tick = tick.wrapping_add(1);
        defmt::info!("tick {}", tick);
    }
}

/// Picotool / `picotool info` metadata. Optional; nice for the operator.
#[link_section = ".bi_entries"]
#[used]
pub static PICOTOOL_ENTRIES: [hal::binary_info::EntryAddr; 3] = [
    hal::binary_info::rp_cargo_bin_name!(),
    hal::binary_info::rp_cargo_version!(),
    hal::binary_info::rp_program_description!(c"Pico A hand-rolled USB host spike"),
];
