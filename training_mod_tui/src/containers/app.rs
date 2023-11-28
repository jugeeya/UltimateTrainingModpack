use serde::ser::{SerializeMap, Serializer};
use serde::Serialize;
use std::collections::HashMap;

use crate::{InputControl, StatefulList, SubMenu, SubMenuType, Tab};

#[derive(PartialEq, Serialize)]
pub enum AppPage {
    SUBMENU,
    TOGGLE,
    SLIDER,
    CONFIRMATION,
    CLOSE,
}

// Menu structure is:
// App <StatefulTable<Tab>>
// │
// └─ Tab <StatefulTable<Submenu>>
//    │
//    └─ Submenu <Struct>
//       │
//       ├─ StatefulTable<Toggle>
//       │
//       │  OR
//       │
//       └─ Option<Slider>

pub struct App<'a> {
    pub tabs: StatefulList<Tab<'a>>,
    pub page: AppPage,
    pub serialized_settings: String,
    pub serialized_default_settings: String,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            tabs: StatefulList::new(),
            page: AppPage::SUBMENU,
            serialized_settings: String::new(),
            serialized_default_settings: String::new(),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("Could not serialize the menu to JSON!")
    }

    pub fn save_settings(&mut self) {
        self.serialized_settings = self.to_json();
    }

    pub fn save_default_settings(&mut self) {
        self.serialized_default_settings = self.to_json();
    }

    pub fn load_defaults(&mut self) {
        // TODO!() is there a way to do this without cloning?
        let json = self.serialized_default_settings.clone();
        self.update_from_json(&json);
    }

    pub fn update_from_json(&mut self, json: &str) {
        let all_settings: HashMap<String, Vec<u8>> =
            serde_json::from_str(json).expect("Could not parse the json!");
        for tab in self.tabs.iter_mut() {
            for submenu_opt in tab.submenus.iter_mut() {
                if let Some(submenu) = submenu_opt {
                    if let Some(val) = all_settings.get(submenu.title) {
                        submenu.update_from_vec(val.clone());
                    }
                }
            }
        }
    }

    pub fn selected_tab(&mut self) -> &mut Tab<'a> {
        self.tabs.get_selected().expect("No tab selected!")
    }

    pub fn selected_submenu(&mut self) -> &mut SubMenu<'a> {
        self.selected_tab()
            .submenus
            .get_selected()
            .expect("No submenu selected!")
    }
}

impl<'a> Serialize for App<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serializes as a mapping between submenu titles and values
        // Need to iterate through tabs to avoid making a list of mappings
        let len: usize = self.tabs.iter().map(|tab| tab.len()).sum();
        let mut map = serializer.serialize_map(Some(len))?;
        for tab in self.tabs.iter() {
            for submenu in tab.submenus.iter() {
                map.serialize_entry(submenu.title, submenu)?;
            }
        }
        map.end()
    }
}

impl<'a> InputControl for App<'a> {
    fn on_a(&mut self) {
        match self.page {
            AppPage::SUBMENU => {
                let tab = self.tabs.get_selected().expect("No tab selected!");
                let submenu_type = tab
                    .submenus
                    .get_selected()
                    .expect("No submenu selected!")
                    .submenu_type;
                self.page = match submenu_type {
                    SubMenuType::ToggleSingle => AppPage::TOGGLE,
                    SubMenuType::ToggleMultiple => AppPage::TOGGLE,
                    SubMenuType::Slider => AppPage::SLIDER,
                    SubMenuType::None => AppPage::SUBMENU,
                };
                tab.on_a()
            }
            AppPage::TOGGLE => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_a(),
            AppPage::SLIDER => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_a(),
            AppPage::CONFIRMATION => {}
            AppPage::CLOSE => {}
        }
    }
    fn on_b(&mut self) {
        match self.page {
            AppPage::SUBMENU => {
                // Exit the app
                self.page = AppPage::CLOSE;
            }
            AppPage::TOGGLE => {
                // Return to the list of submenus
                self.page = AppPage::SUBMENU;
            }
            AppPage::SLIDER => {
                // Return to the list of submenus if we don't have a slider handle selected
                let slider = self
                    .selected_submenu()
                    .slider
                    .as_mut()
                    .expect("No slider selected!");
                if !slider.is_handle_selected() {
                    self.page = AppPage::SUBMENU;
                } else {
                    self.selected_submenu().on_b();
                }
            }
            AppPage::CONFIRMATION => {
                // Return to the list of submenus
                self.page = AppPage::SUBMENU;
            }
            AppPage::CLOSE => {}
        }
    }
    fn on_x(&mut self) {
        self.save_default_settings();
    }
    fn on_y(&mut self) {}
    fn on_up(&mut self) {
        match self.page {
            AppPage::SUBMENU => self.tabs.get_selected().expect("No tab selected!").on_up(),
            AppPage::TOGGLE => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_up(),
            AppPage::SLIDER => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_up(),
            AppPage::CONFIRMATION => {}
            AppPage::CLOSE => {}
        }
    }
    fn on_down(&mut self) {
        match self.page {
            AppPage::SUBMENU => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .on_down(),
            AppPage::TOGGLE => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_down(),
            AppPage::SLIDER => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_down(),
            AppPage::CONFIRMATION => {}
            AppPage::CLOSE => {}
        }
    }
    fn on_left(&mut self) {
        match self.page {
            AppPage::SUBMENU => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .on_left(),
            AppPage::TOGGLE => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_left(),
            AppPage::SLIDER => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_left(),
            AppPage::CONFIRMATION => {}
            AppPage::CLOSE => {}
        }
    }
    fn on_right(&mut self) {
        match self.page {
            AppPage::SUBMENU => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .on_right(),
            AppPage::TOGGLE => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_right(),
            AppPage::SLIDER => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_right(),
            AppPage::CONFIRMATION => {}
            AppPage::CLOSE => {}
        }
    }
    fn on_start(&mut self) {
        // Close menu
        self.page = AppPage::CLOSE;
    }
    fn on_l(&mut self) {
        // Reset current selection to default
        // TODO!() Confirmation
    }
    fn on_r(&mut self) {
        // Reset all settings to default
        // TODO!() Confirmation
        self.load_defaults();
    }
    fn on_zl(&mut self) {
        match self.page {
            AppPage::SUBMENU => {
                self.tabs.previous();
            }
            _ => {}
        }
    }
    fn on_zr(&mut self) {
        match self.page {
            AppPage::SUBMENU => self.tabs.next(),
            _ => {}
        }
    }
}
