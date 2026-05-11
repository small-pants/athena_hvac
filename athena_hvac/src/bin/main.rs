#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use athena_hvac::{sensors::Sensors, solenoids::Solenoids, state::{HVACState, VentConfig}};
use defmt_rtt as _;
use esp_hal::{clock::CpuClock, main};
use esp_hal::delay::Delay;
use esp_backtrace as _;

esp_bootloader_esp_idf::esp_app_desc!();

#[allow(clippy::large_stack_frames)]
#[main]
fn main() -> ! {
    let delay = Delay::new();
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut state = HVACState::new();
    // let mut blower = Blower::new(peripherals.GPIO_X);
    let mut sensors = Sensors::new(
        peripherals.ADC1,
        peripherals.GPIO0,  // cabin temp (TMP36)
        peripherals.GPIO1,  // evap temp (TMP36)
        peripherals.GPIO2,  // temp dial (pot)
        peripherals.GPIO3,  // blower dial (pot)
    );
    // let mut inputs = Inputs::new();
    // let mut display = Display::new();
    let mut solenoids = Solenoids::new(
        peripherals.GPIO4,  // center
        peripherals.GPIO5,  // defrost
        peripherals.GPIO6,  // foot
        peripherals.GPIO7,  // recirc
        peripherals.GPIO9,  // main airflow
    );

    loop {
        // inputs.update(&mut state);
        sensors.update(&mut state);
        state.update();
        // blower.apply(&state);
        let vent_config = VentConfig::from_mode(&state.mode);
        solenoids.apply(&vent_config);
        // display.render(&state);
        defmt::info!("temperature: {}", state.cabin_temp);
        delay.delay_millis(1000);
    }
}
