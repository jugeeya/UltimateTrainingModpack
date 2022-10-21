pub enum GaugeState {
    MinHover,
    MaxHover,
    MinSelected,
    MaxSelected,
    None,
}

pub struct DoubleEndedGauge {
    pub state: GaugeState,
    pub abs_min: u32,
    pub abs_max: u32,
    pub max_selected: u32,
    pub min_selected: u32,
}

impl DoubleEndedGauge {
    pub fn new() -> DoubleEndedGauge {
        DoubleEndedGauge {
            state: GaugeState::None,
            abs_min: 0,
            abs_max: 150,
            max_selected: 0,
            min_selected: 150,
        }
    }

    pub fn from(
        abs_min: u32,
        abs_max: u32,
        min_selected: u32,
        max_selected: u32,
    ) -> DoubleEndedGauge {
        DoubleEndedGauge {
            state: GaugeState::None,
            abs_min: abs_min,
            abs_max: abs_max,
            max_selected: min_selected,
            min_selected: max_selected,
        }
    }
}
