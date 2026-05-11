// src/sensors.rs
use esp_hal::analog::adc::{Adc, AdcCalCurve, AdcConfig, AdcPin, Attenuation};
use esp_hal::peripherals::ADC1;

use crate::state::HVACState;

//  - Cabin Temp Sensor
//  - HVAC Temp Sensor

pub struct Sensors<'d, CabinPin, EvapPin, TempDialPin, BlowerDialPin>
where
    CabinPin: esp_hal::analog::adc::AdcChannel + esp_hal::gpio::AnalogPin,
    EvapPin: esp_hal::analog::adc::AdcChannel + esp_hal::gpio::AnalogPin,
    TempDialPin: esp_hal::analog::adc::AdcChannel + esp_hal::gpio::AnalogPin,
    BlowerDialPin: esp_hal::analog::adc::AdcChannel + esp_hal::gpio::AnalogPin,
{
    adc: Adc<'d, ADC1<'d>, esp_hal::Blocking>,
    cabin_pin: AdcPin<CabinPin, ADC1<'d>, AdcCalCurve<ADC1<'d>>>,
    evap_pin: AdcPin<EvapPin, ADC1<'d>, AdcCalCurve<ADC1<'d>>>,
    temp_dial_pin: AdcPin<TempDialPin, ADC1<'d>, AdcCalCurve<ADC1<'d>>>,
    blower_dial_pin: AdcPin<BlowerDialPin, ADC1<'d>, AdcCalCurve<ADC1<'d>>>,
}

impl<'d, CabinPin, EvapPin, TempDialPin, BlowerDialPin>
    Sensors<'d, CabinPin, EvapPin, TempDialPin, BlowerDialPin>
where
    CabinPin: esp_hal::analog::adc::AdcChannel + esp_hal::gpio::AnalogPin,
    EvapPin: esp_hal::analog::adc::AdcChannel + esp_hal::gpio::AnalogPin,
    TempDialPin: esp_hal::analog::adc::AdcChannel + esp_hal::gpio::AnalogPin,
    BlowerDialPin: esp_hal::analog::adc::AdcChannel + esp_hal::gpio::AnalogPin,
{
    pub fn new(
        adc1: ADC1<'d>,
        cabin_pin: CabinPin,
        evap_pin: EvapPin,
        temp_dial_pin: TempDialPin,
        blower_dial_pin: BlowerDialPin,
    ) -> Self {
        let mut config = AdcConfig::new();

        let cabin_pin = config.enable_pin_with_cal::<_, AdcCalCurve<_>>(
            cabin_pin,
            Attenuation::_6dB, // _6dB gives 0-3.3V range
        );
        let evap_pin = config.enable_pin_with_cal::<_, AdcCalCurve<_>>(
            evap_pin,
            Attenuation::_6dB,
        );
        let temp_dial_pin = config.enable_pin_with_cal::<_, AdcCalCurve<_>>(
            temp_dial_pin,
            Attenuation::_6dB,
        );
        let blower_dial_pin = config.enable_pin_with_cal::<_, AdcCalCurve<_>>(
            blower_dial_pin,
            Attenuation::_6dB,
        );

        let adc = Adc::new(adc1, config);

        Self {
            adc,
            cabin_pin,
            evap_pin,
            temp_dial_pin,
            blower_dial_pin,
        }
    }

    pub fn update(&mut self, state: &mut HVACState) {
        // AdcCalCurve returns millivolts directly
        if let Ok(mv) = nb::block!(self.adc.read_oneshot(&mut self.cabin_pin)) {
            state.cabin_temp = mv_to_celsius(mv);
        }
        if let Ok(mv) = nb::block!(self.adc.read_oneshot(&mut self.evap_pin)) {
            state.output_temp = mv_to_celsius(mv);
        }
        if let Ok(mv) = nb::block!(self.adc.read_oneshot(&mut self.temp_dial_pin)) {
            // Map 0-3300mV to your temperature setpoint range e.g. 16-30°C
            state.set_point = map_range(mv, 0, 3300, 16, 30);
        }
        if let Ok(mv) = nb::block!(self.adc.read_oneshot(&mut self.blower_dial_pin)) {
            // Map 0-3300mV to 0-255 blower duty
            state.blower_duty = map_range(mv, 0, 3300, 0, 255) as u8;
        }
    }
}

/// Convert NTC thermistor millivolt reading to Celsius
fn mv_to_celsius(mv: u16) -> f32 {
    (mv as f32 - 500.0) / 10.0
}

fn map_range(val: u16, in_min: u16, in_max: u16, out_min: i32, out_max: i32) -> i32 {
    let val = val.clamp(in_min, in_max);
    out_min + (val - in_min) as i32 * (out_max - out_min) / (in_max - in_min) as i32
}
