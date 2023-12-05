use serde::{Serialize, Serializer};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SliderState {
    LowerHover,
    UpperHover,
    LowerSelected,
    UpperSelected,
    None,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct StatefulSlider {
    pub state: SliderState,
    pub lower: u32,
    pub upper: u32,
    pub min: u32,
    pub max: u32,
    pub incr_amount_slow: u32,
    pub incr_amount_fast: u32,
}

impl StatefulSlider {
    pub fn new() -> StatefulSlider {
        StatefulSlider {
            state: SliderState::LowerHover,
            lower: 0,
            upper: 150,
            min: 0,
            max: 150,
            incr_amount_slow: 1,
            incr_amount_fast: 10,
        }
    }

    pub fn increment_selected_slow(&mut self) {
        match self.state {
            SliderState::LowerSelected => {
                self.lower = self
                    .lower
                    .saturating_add(self.incr_amount_slow)
                    .min(self.upper); // Don't allow lower > upper
            }
            SliderState::UpperSelected => {
                self.upper = self
                    .upper
                    .saturating_add(self.incr_amount_slow)
                    .min(self.max); // Don't allow upper > max
            }
            _ => {}
        }
    }

    pub fn increment_selected_fast(&mut self) {
        match self.state {
            SliderState::LowerSelected => {
                self.lower = self
                    .lower
                    .saturating_add(self.incr_amount_fast)
                    .min(self.upper); // Don't allow lower > upper
            }
            SliderState::UpperSelected => {
                self.upper = self
                    .upper
                    .saturating_add(self.incr_amount_fast)
                    .min(self.max); // Don't allow upper > max
            }
            _ => {}
        }
    }

    pub fn decrement_selected_slow(&mut self) {
        match self.state {
            SliderState::LowerSelected => {
                self.lower = self
                    .lower
                    .saturating_sub(self.incr_amount_slow)
                    .max(self.min); // Don't allow lower < min
            }
            SliderState::UpperSelected => {
                self.upper = self
                    .upper
                    .saturating_sub(self.incr_amount_slow)
                    .max(self.lower); // Don't allow upper < lower
            }
            _ => {}
        }
    }

    pub fn decrement_selected_fast(&mut self) {
        match self.state {
            SliderState::LowerSelected => {
                self.lower = self
                    .lower
                    .saturating_sub(self.incr_amount_fast)
                    .max(self.min); // Don't allow lower < min
            }
            SliderState::UpperSelected => {
                self.upper = self
                    .upper
                    .saturating_sub(self.incr_amount_fast)
                    .max(self.lower); // Don't allow upper < lower
            }
            _ => {}
        }
    }

    pub fn select_deselect(&mut self) {
        self.state = match self.state {
            SliderState::LowerHover => SliderState::LowerSelected,
            SliderState::LowerSelected => SliderState::LowerHover,
            SliderState::UpperHover => SliderState::UpperSelected,
            SliderState::UpperSelected => SliderState::UpperHover,
            SliderState::None => SliderState::None,
        }
    }

    pub fn deselect(&mut self) {
        self.state = match self.state {
            SliderState::LowerSelected => SliderState::LowerHover,
            SliderState::UpperSelected => SliderState::UpperHover,
            _ => self.state,
        }
    }

    pub fn switch_hover(&mut self) {
        self.state = match self.state {
            SliderState::LowerHover => SliderState::UpperHover,
            SliderState::UpperHover => SliderState::LowerHover,
            _ => self.state,
        }
    }

    pub fn is_handle_selected(&mut self) -> bool {
        self.state == SliderState::LowerSelected || self.state == SliderState::UpperSelected
    }
}

impl Serialize for StatefulSlider {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        [self.lower, self.upper].serialize(serializer)
    }
}
