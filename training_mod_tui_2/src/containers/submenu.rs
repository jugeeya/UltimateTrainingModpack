use serde::ser::Serializer;
use serde::Serialize;

use crate::{InputControl, StatefulSlider, StatefulTable, Toggle};

#[derive(Clone)]
pub struct SubMenu<'a> {
    pub title: &'a str,
    pub id: &'a str,
    pub help_text: &'a str,
    pub submenu_type: SubMenuType,
    pub toggles: StatefulTable<Toggle<'a>>,
    pub slider: Option<StatefulSlider>,
}

impl<'a> Serialize for SubMenu<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.submenu_type {
            SubMenuType::ToggleMultiple | SubMenuType::ToggleSingle => {
                self.toggles.serialize(serializer)
            }
            SubMenuType::Slider => self.slider.serialize(serializer),
            SubMenuType::None => panic!("At the disco"),
        }
    }
}

impl<'a> InputControl for SubMenu<'a> {
    fn on_a(&mut self) {
        match self.submenu_type {
            SubMenuType::ToggleSingle => {
                // Set all values to 0 first before incrementing the selected toggle
                // This ensure that exactly one toggle has a nonzero value
                for ind in 0..self.toggles.len() {
                    self.toggles.get_by_idx_mut(ind).unwrap().value = 0;
                }
                self.selected_toggle().increment();
            }
            SubMenuType::ToggleMultiple => self.selected_toggle().increment(),
            SubMenuType::Slider => {
                let slider = self.slider.as_mut().expect("No slider selected!");
                slider.select_deselect();
            }
            SubMenuType::None => {}
        }
    }
    fn on_b(&mut self) {
        match self.submenu_type {
            SubMenuType::ToggleSingle => {}
            SubMenuType::ToggleMultiple => {}
            SubMenuType::Slider => {
                let slider = self.slider.as_mut().expect("No slider selected!");
                if slider.is_handle_selected() {
                    slider.deselect()
                }
            }
            SubMenuType::None => {}
        }
    }
    fn on_x(&mut self) {}
    fn on_y(&mut self) {}
    fn on_up(&mut self) {
        match self.submenu_type {
            SubMenuType::ToggleSingle => self.toggles.prev_row_checked(),
            SubMenuType::ToggleMultiple => self.toggles.prev_row_checked(),
            SubMenuType::Slider => {}
            SubMenuType::None => {}
        }
    }
    fn on_down(&mut self) {
        match self.submenu_type {
            SubMenuType::ToggleSingle => self.toggles.next_row_checked(),
            SubMenuType::ToggleMultiple => self.toggles.next_row_checked(),
            SubMenuType::Slider => {}
            SubMenuType::None => {}
        }
    }
    fn on_left(&mut self) {
        match self.submenu_type {
            SubMenuType::ToggleSingle => self.toggles.prev_col_checked(),
            SubMenuType::ToggleMultiple => self.toggles.prev_col_checked(),
            SubMenuType::Slider => {
                let slider = self.slider.as_mut().expect("No slider selected!");
                if slider.is_handle_selected() {
                    slider.decrement_selected_slow();
                } else {
                    slider.switch_hover();
                }
            }
            SubMenuType::None => {}
        }
    }
    fn on_right(&mut self) {
        match self.submenu_type {
            SubMenuType::ToggleSingle => self.toggles.next_col_checked(),
            SubMenuType::ToggleMultiple => self.toggles.next_col_checked(),
            SubMenuType::Slider => {
                let slider = self.slider.as_mut().expect("No slider selected!");
                if slider.is_handle_selected() {
                    slider.increment_selected_slow();
                } else {
                    slider.switch_hover();
                }
            }
            SubMenuType::None => {}
        }
    }
    fn on_start(&mut self) {}
    fn on_l(&mut self) {}
    fn on_r(&mut self) {}
    fn on_zl(&mut self) {}
    fn on_zr(&mut self) {}
}

impl<'a> SubMenu<'a> {
    pub fn selected_toggle(&mut self) -> &mut Toggle<'a> {
        self.toggles.get_selected().expect("No toggle selected!")
    }

    pub fn update_from_vec(&mut self, values: Vec<u8>) {
        match self.submenu_type {
            SubMenuType::ToggleSingle | SubMenuType::ToggleMultiple => {
                for (idx, value) in values.iter().enumerate() {
                    if let Some(toggle) = self.toggles.get_by_idx_mut(idx) {
                        toggle.value = *value;
                    }
                }
            }
            SubMenuType::Slider => {
                assert_eq!(
                    values.len(),
                    2,
                    "Exactly two values need to be passed to submenu.set() for slider!"
                );
                if let Some(s) = self.slider {
                    self.slider = Some(StatefulSlider {
                        lower: values[0].into(),
                        upper: values[1].into(),
                        ..s
                    });
                }
            }
            SubMenuType::None => {}
        }
    }
}

#[derive(Clone, Copy, Serialize)]
pub enum SubMenuType {
    ToggleSingle,
    ToggleMultiple,
    Slider,
    None,
}
