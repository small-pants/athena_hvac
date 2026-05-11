#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::clock::CpuClock;
use esp_hal::gpio::Level;
use esp_hal::main;
use esp_hal::rmt::{PulseCode, Rmt, TxChannelConfig, TxChannelCreator};
use esp_hal::time::{Duration, Instant, Rate};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[allow(clippy::large_stack_frames)]
#[main]
fn main() -> ! {
let peripherals = esp_hal::init(config);

    let mut state = HvacState::new();
    let mut blower = Blower::new(peripherals.GPIO_X);
    let mut sensors = Sensors::new(peripherals.ADC);
    let mut inputs = Inputs::new(...);
    // let mut display = Display::new();
    let mut solenoids = Solenoids::new(
        peripherals.GPIO1,  // center
        peripherals.GPIO2,  // defrost
        peripherals.GPIO3,  // foot
        peripherals.GPIO4,  // recirc
        peripherals.GPIO5,  // main airflow
    );

    loop {
        inputs.update(&mut state);
        sensors.update(&mut state);
        state.update();          // state machine logic
        blower.apply(&state);
        solenoids.apply(&state);
        display.render(&state);
    }
}
