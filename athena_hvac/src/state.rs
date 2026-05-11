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
    pub cabin_temp: f32,
    pub output_temp: f32,
    pub blower_duty: u8,
    pub compressor_on: bool,
    pub solenoids: [bool; 6],
}

pub struct VentConfig {
    pub center: bool,
    pub defrost: bool,
    pub foot: bool,
    pub recirc: bool,
    pub main_airflow: bool,
}
