pub enum GaugeState {
    MinHover,
    MaxHover,
    MinSelected,
    MaxSelected,
    None,
}

pub struct DoubleEndedGauge {
    pub state: GaugeState,
    pub selected_min: u32,
    pub selected_max: u32,
    pub abs_min: u32,
    pub abs_max: u32,
}

impl DoubleEndedGauge {
    pub fn new() -> DoubleEndedGauge {
        DoubleEndedGauge {
            state: GaugeState::None,
            selected_min: 0,
            selected_max: 150,
            abs_min: 0,
            abs_max: 150,
        }
    }
}
