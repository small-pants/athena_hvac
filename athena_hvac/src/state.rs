pub enum HVACMode {
    Off,
    FanOnly,
    Auto,
    MaxCool,
    MaxHeat,
    Defrost,
}

pub struct HVACState {
    pub mode: HVACMode,
    pub set_point: i32,
    pub cabin_temp: i32,
    pub output_temp: i32,
    pub blower_duty: u8,
    pub compressor_on: bool,
    pub pressure_switch: bool,
    pub solenoids: VentConfig,
}

impl HVACState {
    pub fn new() -> HVACState{
        return Self {
            mode: HVACMode::Auto,
            set_point: 68,
            cabin_temp: 68,
            output_temp: 68,
            blower_duty: 0,
            compressor_on: false,
            pressure_switch: false,
            solenoids: VentConfig::default(),
        }
    }

    pub fn update(&mut self) {
        match self.mode {
            HVACMode::Off => {
                self.blower_duty = 0;
                self.compressor_on = false;
                self.solenoids = VentConfig::off();
            }

            HVACMode::FanOnly => {
                // TODO: set fan directly to dial setting
                self.compressor_on = false;
                self.solenoids = VentConfig::from_mode(&self.mode);
            }

            HVACMode::Auto => {
                self.blower_duty = self.calculate_auto_blower();
                self.compressor_on = self.should_run_compressor();
                self.solenoids = VentConfig::from_mode(&self.mode);
            }

            HVACMode::MaxCool => {
                self.blower_duty = 255;
                self.compressor_on = self.should_run_compressor();
                self.solenoids = VentConfig::from_mode(&self.mode);
            }

            HVACMode::MaxHeat => {
                self.blower_duty = 255;
                self.compressor_on = false;
                self.solenoids = VentConfig::from_mode(&self.mode);
            }

            HVACMode::Defrost => {
                self.blower_duty = 255;
                self.compressor_on = self.should_run_compressor();
                self.solenoids = VentConfig::from_mode(&self.mode);
            }
        }
    }

    fn calculate_auto_blower(&self) -> u8 {
        let error: i32 = self.set_point - self.cabin_temp;

        // Below setpoint — scale blower proportionally to how far off we are
        // Clamp to MIN_BLOWER_DUTY so the fan never fully stops in auto mode
        const MIN_BLOWER_DUTY: u8 = 40;
        const GAIN: f32 = 15.0; // TODO: tune this for blower motor

        if error <= 0 {
            MIN_BLOWER_DUTY
        } else {
            let duty: u8 = (error as f32 * GAIN) as u8;
            duty.clamp(MIN_BLOWER_DUTY, 255)
        }
    }

    fn should_run_compressor(&self) -> bool {
        // Protect compressor from running if evap is too cold (ice up)
        const EVAP_FREEZE_THRESHOLD: f32 = 2.0;

        // Protect from low pressure switch
        if !self.pressure_switch {
            return false;
        }

        // Prevent evaporator icing
        if (self.output_temp as f32) < EVAP_FREEZE_THRESHOLD {
            return false;
        }

        true
    }
}

#[derive(Default)]
pub struct VentConfig {
    pub center: bool,
    pub defrost: bool,
    pub foot: bool,
    pub recirc: bool,
    pub main_airflow: bool,
}

impl VentConfig {
    pub fn off() -> Self {
        Self {
            center: false,
            defrost: false,
            foot: false,
            recirc: false,
            main_airflow: false,
        }
    }

    pub fn from_mode(mode: &HVACMode) -> Self {
        match mode {
            HVACMode::Off => Self::off(),
            HVACMode::FanOnly | HVACMode::Auto | HVACMode::MaxCool => Self {
                center: true,
                defrost: false,
                foot: false,
                recirc: false,
                main_airflow: true,
            },
            HVACMode::MaxHeat => Self {
                center: false,
                defrost: false,
                foot: true,
                recirc: false,
                main_airflow: true,
            },
            HVACMode::Defrost => Self {
                center: false,
                defrost: true,
                foot: false,
                recirc: false,
                main_airflow: true,
            },
        }
    }
}
