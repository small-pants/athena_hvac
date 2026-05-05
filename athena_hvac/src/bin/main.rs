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

// WS2812 timing at 80MHz RMT clock with clk_divider=2
// Each RMT tick = 25ns
// T0H = 400ns  = 16 ticks
// T0L = 850ns  = 34 ticks
// T1H = 800ns  = 32 ticks
// T1L = 450ns  = 18 ticks
const T0H: u16 = 16;
const T0L: u16 = 34;
const T1H: u16 = 32;
const T1L: u16 = 18;

fn encode_byte(byte: u8, buffer: &mut [PulseCode]) {
    for i in 0..8 {
        let bit = (byte >> (7 - i)) & 1;
        buffer[i] = if bit == 1 {
            PulseCode::new(Level::High, T1H, Level::Low, T1L)
        } else {
            PulseCode::new(Level::High, T0H, Level::Low, T0L)
        };
    }
}

#[allow(clippy::large_stack_frames)]
#[main]
fn main() -> ! {
    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let freq = Rate::from_mhz(80);
    let rmt = Rmt::new(peripherals.RMT, freq).unwrap();

    let tx_config = TxChannelConfig::default()
        .with_clk_divider(2)
        .with_idle_output_level(Level::Low)
        .with_idle_output(true)
        .with_carrier_modulation(false);

    let mut channel = rmt
        .channel0
        .configure_tx(peripherals.GPIO8, tx_config)
        .unwrap();

    // 24 bits per LED (GRB order) + 1 end marker
    let mut pulse_buffer = [PulseCode::end_marker(); 25];

    let colors: [(u8, u8, u8); 2] = [
        (26, 26, 26), // white
        (0, 0, 26), // blue
    ];
    let mut color_idx = 0;

    loop {
        let (r, g, b) = colors[color_idx];
        // WS2812 expects GRB order
        encode_byte(g, &mut pulse_buffer[0..8]);
        encode_byte(r, &mut pulse_buffer[8..16]);
        encode_byte(b, &mut pulse_buffer[16..24]);
        pulse_buffer[24] = PulseCode::end_marker();

        let transaction = channel.transmit(&pulse_buffer).unwrap();
        channel = transaction.wait().unwrap();

        color_idx = (color_idx + 1) % colors.len();

        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }
}
