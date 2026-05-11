use esp_hal::gpio::{Level, Output, OutputConfig, OutputPin};

use crate::state::VentConfig;

pub struct Solenoids<'d> {
    center: Output<'d>,
    defrost: Output<'d>,
    foot: Output<'d>,
    recirc: Output<'d>,
    main_airflow: Output<'d>,
}

impl<'d> Solenoids<'d> {
    pub fn new(
        center: impl OutputPin + 'd,
        defrost: impl OutputPin + 'd,
        foot: impl OutputPin + 'd,
        recirc: impl OutputPin + 'd,
        main_airflow: impl OutputPin + 'd,
    ) -> Self {
        Self {
            center: Output::new(center, Level::Low, OutputConfig::default()),
            defrost: Output::new(defrost, Level::Low, OutputConfig::default()),
            foot: Output::new(foot, Level::Low, OutputConfig::default()),
            recirc: Output::new(recirc, Level::Low, OutputConfig::default()),
            main_airflow: Output::new(main_airflow, Level::Low, OutputConfig::default()),
        }
    }

    pub fn apply(&mut self, config: &VentConfig) {
        toggle_pin(&mut self.center, config.center);
        toggle_pin(&mut self.defrost, config.defrost);
        toggle_pin(&mut self.foot, config.foot);
        toggle_pin(&mut self.recirc, config.recirc);
        toggle_pin(&mut self.main_airflow, config.main_airflow);
    }
}

fn toggle_pin(pin: &mut Output, state: bool) {
    if state { pin.set_high() } else { pin.set_low() }
}
