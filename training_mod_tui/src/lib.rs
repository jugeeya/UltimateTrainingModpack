use training_mod_consts::{MenuJsonStruct, Slider, SubMenu, SubMenuType, Toggle, UiMenu, ui_menu, TrainingModpackMenu};
use tui::{
    backend::Backend,
    layout::{Constraint, Corner, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, LineGauge, List, ListItem, ListState, Paragraph, Tabs},
    Frame,
};

use std::collections::HashMap;
use serde_json::{Map, json};
pub use tui::{backend::TestBackend, style::Color, Terminal};

pub mod gauge;
mod list;

use crate::gauge::{DoubleEndedGauge, GaugeState};
use crate::list::{MultiStatefulList, StatefulList};

static NX_TUI_WIDTH: u16 = 66;

#[derive(PartialEq)]
pub enum AppPage {
    SUBMENU,
    TOGGLE,
    SLIDER,
    CONFIRMATION
}

/// We should hold a list of SubMenus.
/// The currently selected SubMenu should also have an associated list with necessary information.
/// We can convert the option types (Toggle, OnOff, Slider) to lists
pub struct App<'a> {
    pub tabs: StatefulList<&'a str>,
    pub menu_items: HashMap<&'a str, MultiStatefulList<SubMenu<'a>>>,
    pub selected_sub_menu_toggles: MultiStatefulList<Toggle<'a>>,
    pub selected_sub_menu_slider: DoubleEndedGauge,
    pub page: AppPage,
    pub default_menu: (UiMenu<'a>, String),
}

impl<'a> App<'a> {
    pub fn new(menu: UiMenu<'a>, default_menu: (UiMenu<'a>, String)) -> App<'a> {
        let num_lists = 3;

        let mut menu_items_stateful = HashMap::new();
        menu.tabs.iter().for_each(|tab| {
            menu_items_stateful.insert(
                tab.tab_title,
                MultiStatefulList::with_items(tab.tab_submenus.clone(), num_lists),
            );
        });

        let mut app = App {
            tabs: StatefulList::with_items(menu.tabs.iter().map(|tab| tab.tab_title).collect()),
            menu_items: menu_items_stateful,
            selected_sub_menu_toggles: MultiStatefulList::with_items(vec![], 0),
            selected_sub_menu_slider: DoubleEndedGauge::new(),
            page: AppPage::SUBMENU,
            default_menu: default_menu
        };
        app.set_sub_menu_items();
        app
    }

    /// Takes the currently selected tab/submenu and clones the options into
    /// self.selected_sub_menu_toggles and self.selected_sub_menu_slider
    pub fn set_sub_menu_items(&mut self) {
        let (list_section, list_idx) = self
            .menu_items
            .get(self.tab_selected())
            .unwrap()
            .idx_to_list_idx(self.menu_items.get(self.tab_selected()).unwrap().state);
        let selected_sub_menu = &self.menu_items.get(self.tab_selected()).unwrap().lists
            [list_section]
            .items
            .get(list_idx)
            .unwrap();

        let toggles = selected_sub_menu.toggles.clone();
        let slider = selected_sub_menu.slider.clone();
        match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::TOGGLE => {
                self.selected_sub_menu_toggles = MultiStatefulList::with_items(
                    toggles,
                    if selected_sub_menu.toggles.len() >= 3 {
                        3
                    } else {
                        selected_sub_menu.toggles.len()
                    },
                )
            }
            SubMenuType::SLIDER => {
                let slider = slider.unwrap();
                self.selected_sub_menu_slider = DoubleEndedGauge {
                    state: GaugeState::None,
                    selected_min: slider.selected_min,
                    selected_max: slider.selected_max,
                    abs_min: slider.abs_min,
                    abs_max: slider.abs_max,
                }
            }
        };
    }

    /// Returns the id of the currently selected tab
    pub fn tab_selected(&self) -> &str {
        self.tabs
            .items
            .get(self.tabs.state.selected().unwrap())
            .unwrap()
    }

    /// Returns the currently selected SubMenu struct
    ///
    /// {
    ///   submenu_title: &'a str,
    ///   submenu_id: &'a str,
    ///   help_text: &'a str,
    ///   is_single_option: bool,
    ///   toggles: Vec<Toggle<'a>>,
    ///   slider: Option<Slider>,
    ///   _type: &'a str,
    /// }
    fn sub_menu_selected(&self) -> &SubMenu {
        let (list_section, list_idx) = self
            .menu_items
            .get(self.tab_selected())
            .unwrap()
            .idx_to_list_idx(self.menu_items.get(self.tab_selected()).unwrap().state);
        self.menu_items.get(self.tab_selected()).unwrap().lists[list_section]
            .items
            .get(list_idx)
            .unwrap()
    }

    /// A "next()" function which differs per submenu type
    /// Toggles: calls next()
    /// Slider: Swaps between MinHover and MaxHover
    pub fn sub_menu_next(&mut self) {
        match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::TOGGLE => self.selected_sub_menu_toggles.next(),
            SubMenuType::SLIDER => match self.selected_sub_menu_slider.state {
                GaugeState::MinHover => self.selected_sub_menu_slider.state = GaugeState::MaxHover,
                GaugeState::MaxHover => self.selected_sub_menu_slider.state = GaugeState::MinHover,
                _ => {}
            },
        }
    }

    /// A "next_list()" function which differs per submenu type
    /// Toggles: Calls next_list()
    /// Slider:
    ///     * Swaps between MinHover and MaxHover
    ///     * Increments the selected_min/max if possible
    pub fn sub_menu_next_list(&mut self) {
        match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::TOGGLE => self.selected_sub_menu_toggles.next_list(),
            SubMenuType::SLIDER => match self.selected_sub_menu_slider.state {
                GaugeState::MinHover => self.selected_sub_menu_slider.state = GaugeState::MaxHover,
                GaugeState::MaxHover => self.selected_sub_menu_slider.state = GaugeState::MinHover,
                GaugeState::MinSelected => {
                    if self.selected_sub_menu_slider.selected_min
                        < self.selected_sub_menu_slider.selected_max
                    {
                        self.selected_sub_menu_slider.selected_min += 1;
                    }
                }
                GaugeState::MaxSelected => {
                    if self.selected_sub_menu_slider.selected_max
                        < self.selected_sub_menu_slider.abs_max
                    {
                        self.selected_sub_menu_slider.selected_max += 1;
                    }
                }
                GaugeState::None => {}
            },
        }
    }

    /// A "previous()" function which differs per submenu type
    /// Toggles: calls previous()
    /// Slider: Swaps between MinHover and MaxHover
    pub fn sub_menu_previous(&mut self) {
        match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::TOGGLE => self.selected_sub_menu_toggles.previous(),
            SubMenuType::SLIDER => match self.selected_sub_menu_slider.state {
                GaugeState::MinHover => self.selected_sub_menu_slider.state = GaugeState::MaxHover,
                GaugeState::MaxHover => self.selected_sub_menu_slider.state = GaugeState::MinHover,
                _ => {}
            },
        }
    }

    /// A "previous_list()" function which differs per submenu type
    /// Toggles: Calls previous_list()
    /// Slider:
    ///     * Swaps between MinHover and MaxHover
    ///     * Decrements the selected_min/max if possible
    pub fn sub_menu_previous_list(&mut self) {
        match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::TOGGLE => self.selected_sub_menu_toggles.previous_list(),
            SubMenuType::SLIDER => match self.selected_sub_menu_slider.state {
                GaugeState::MinHover => self.selected_sub_menu_slider.state = GaugeState::MaxHover,
                GaugeState::MaxHover => self.selected_sub_menu_slider.state = GaugeState::MinHover,
                GaugeState::MinSelected => {
                    if self.selected_sub_menu_slider.selected_min
                        > self.selected_sub_menu_slider.abs_min
                    {
                        self.selected_sub_menu_slider.selected_min -= 1;
                    }
                }
                GaugeState::MaxSelected => {
                    if self.selected_sub_menu_slider.selected_max
                        > self.selected_sub_menu_slider.selected_min
                    {
                        self.selected_sub_menu_slider.selected_max -= 1;
                    }
                }
                GaugeState::None => {}
            },
        }
    }

    /// Returns information about the currently selected submenu
    /// 
    /// 0: Submenu Title
    /// 1: Submenu Help Text
    /// 2: Vec(toggle checked, title) for toggles, Vec(nothing) for slider
    /// 3: ListState for toggles, ListState::new() for slider
    /// TODO: Refactor return type into a nice struct
    pub fn sub_menu_strs_and_states(
        &self,
    ) -> (&str, &str, Vec<(Vec<(bool, &str)>, ListState)>) {
        (
            self.sub_menu_selected().submenu_title,
            self.sub_menu_selected().help_text,
            match SubMenuType::from_str(self.sub_menu_selected()._type) {
                SubMenuType::TOGGLE => self
                    .selected_sub_menu_toggles
                    .lists
                    .iter()
                    .map(|toggle_list| {
                        (
                            toggle_list
                                .items
                                .iter()
                                .map(|toggle| (toggle.checked, toggle.toggle_title))
                                .collect(),
                            toggle_list.state.clone(),
                        )
                    })
                    .collect(),
                SubMenuType::SLIDER => {
                    vec![(vec![], ListState::default())]
                }
            },
        )
    }

    /// Returns information about the currently selected slider
    /// 0: Title
    /// 1: Help text
    /// 2: Reference to self.selected_sub_menu_slider
    /// TODO: Refactor return type into a nice struct
    pub fn sub_menu_strs_for_slider(&self) -> (&str, &str, &DoubleEndedGauge) {
        let slider = match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::SLIDER => &self.selected_sub_menu_slider,
            _ => {
                panic!("Slider not selected!");
            }
        };
        (
            self.sub_menu_selected().submenu_title,
            self.sub_menu_selected().help_text,
            slider,
        )
    }

    /// Different behavior depending on the current menu location
    /// Submenu list: Enters toggle or slider submenu
    /// Toggle submenu: Toggles the selected submenu toggle in self.selected_sub_menu_toggles and in the actual SubMenu struct
    /// Slider submenu: Swaps hover/selected state. Updates the actual SubMenu struct if going from Selected -> Hover
    pub fn on_a(&mut self) {
        let tab_selected = self
            .tabs
            .items
            .get(self.tabs.state.selected().unwrap())
            .unwrap();
        let (list_section, list_idx) = self
            .menu_items
            .get(tab_selected)
            .unwrap()
            .idx_to_list_idx(self.menu_items.get(tab_selected).unwrap().state);
        let selected_sub_menu = self.menu_items.get_mut(tab_selected).unwrap().lists
            [list_section]
            .items
            .get_mut(list_idx)
            .unwrap();
        if self.page == AppPage::SUBMENU {
            match SubMenuType::from_str(selected_sub_menu._type) {
                // Need to change the slider state to MinHover so the slider shows up initially
                SubMenuType::SLIDER => {
                    self.page = AppPage::SLIDER;
                    self.selected_sub_menu_slider.state = GaugeState::MinHover;
                }
                SubMenuType::TOGGLE => self.page = AppPage::TOGGLE
            }
        } else {
            match SubMenuType::from_str(selected_sub_menu._type) {
                SubMenuType::TOGGLE => {
                    let is_single_option = selected_sub_menu.is_single_option;
                    let state = self.selected_sub_menu_toggles.state;
                    // Change the toggles in self.selected_sub_menu_toggles (for display)
                    self.selected_sub_menu_toggles
                        .lists
                        .iter_mut()
                        .map(|list| (list.state.selected(), &mut list.items))
                        .for_each(|(state, toggle_list)| {
                            toggle_list.iter_mut().enumerate().for_each(|(i, o)| {
                                if state.is_some() && i == state.unwrap() {
                                    if !o.checked {
                                        o.checked = true;
                                    } else {
                                        o.checked = false;
                                    }
                                } else if is_single_option {
                                    o.checked = false;
                                }
                            })
                        });
                    // Actually change the toggle values in the SubMenu struct
                    selected_sub_menu
                        .toggles
                        .iter_mut()
                        .enumerate()
                        .for_each(|(i, o)| {
                            if i == state {
                                if !o.checked {
                                    o.checked = true;
                                } else {
                                    o.checked = false;
                                }
                            } else if is_single_option {
                                o.checked = false;
                            }
                        });
                }
                SubMenuType::SLIDER => match self.selected_sub_menu_slider.state {
                    GaugeState::MinHover => {
                        self.selected_sub_menu_slider.state = GaugeState::MinSelected;
                    }
                    GaugeState::MaxHover => {
                        self.selected_sub_menu_slider.state = GaugeState::MaxSelected;
                    }
                    GaugeState::MinSelected => {
                        self.selected_sub_menu_slider.state = GaugeState::MinHover;
                        selected_sub_menu.slider = Some(Slider{
                            selected_min: self.selected_sub_menu_slider.selected_min,
                            selected_max: self.selected_sub_menu_slider.selected_max,
                            abs_min: self.selected_sub_menu_slider.abs_min,
                            abs_max: self.selected_sub_menu_slider.abs_max,
                        });
                    }
                    GaugeState::MaxSelected => {
                        self.selected_sub_menu_slider.state = GaugeState::MaxHover;
                        selected_sub_menu.slider = Some(Slider{
                            selected_min: self.selected_sub_menu_slider.selected_min,
                            selected_max: self.selected_sub_menu_slider.selected_max,
                            abs_min: self.selected_sub_menu_slider.abs_min,
                            abs_max: self.selected_sub_menu_slider.abs_max,
                        });
                    }
                    GaugeState::None => {
                        self.selected_sub_menu_slider.state = GaugeState::MinHover;
                    }
                },
            }
        }
    }

    /// Different behavior depending on the current menu location
    /// Submenu selection: None
    /// Toggle submenu: Sets page to submenu selection
    /// Slider submenu: If in a selected state, then commit changes and change to hover. Else set page to submenu selection
    pub fn on_b(&mut self) {
        let tab_selected = self
            .tabs
            .items
            .get(self.tabs.state.selected().unwrap())
            .unwrap();
        let (list_section, list_idx) = self
            .menu_items
            .get(tab_selected)
            .unwrap()
            .idx_to_list_idx(self.menu_items.get(tab_selected).unwrap().state);
        let selected_sub_menu = self.menu_items.get_mut(tab_selected).unwrap().lists[list_section]
            .items
            .get_mut(list_idx)
            .unwrap();
        match SubMenuType::from_str(selected_sub_menu._type) {
            SubMenuType::SLIDER => match self.selected_sub_menu_slider.state {
                GaugeState::MinSelected => {
                    self.selected_sub_menu_slider.state = GaugeState::MinHover;
                    selected_sub_menu.slider = Some(Slider{
                        selected_min: self.selected_sub_menu_slider.selected_min,
                        selected_max: self.selected_sub_menu_slider.selected_max,
                        abs_min: self.selected_sub_menu_slider.abs_min,
                        abs_max: self.selected_sub_menu_slider.abs_max,
                    });
                    // Don't go back to the outer list
                    return;
                }
                GaugeState::MaxSelected => {
                    self.selected_sub_menu_slider.state = GaugeState::MaxHover;
                    selected_sub_menu.slider = Some(Slider{
                        selected_min: self.selected_sub_menu_slider.selected_min,
                        selected_max: self.selected_sub_menu_slider.selected_max,
                        abs_min: self.selected_sub_menu_slider.abs_min,
                        abs_max: self.selected_sub_menu_slider.abs_max,
                    });
                    // Don't go back to the outer list
                    return;
                }
                _ => {}
            },
            _ => {}
        }
        self.page = AppPage::SUBMENU;
        self.set_sub_menu_items();
    }

    /// Save defaults command
    pub fn on_x(&mut self) {
        if self.page == AppPage::SUBMENU {
            let json = self.to_json();
            unsafe {
                self.default_menu = (ui_menu(serde_json::from_str::<TrainingModpackMenu>(&json).unwrap()), json);
            }
        }
    }

    /// Reset current submenu to defaults
    pub fn on_l(&mut self) {
        if self.page == AppPage::TOGGLE || self.page == AppPage::SLIDER {
            let json = self.to_json();
            let mut json_value = serde_json::from_str::<serde_json::Value>(&json).unwrap();
            let selected_sub_menu= self.sub_menu_selected();
            let id = selected_sub_menu.submenu_id;
            let default_json_value = serde_json::from_str::<serde_json::Value>(&self.default_menu.1).unwrap();
            *json_value.get_mut(id).unwrap() = default_json_value.get(id).unwrap().clone();
            let new_menu = serde_json::from_value::<TrainingModpackMenu>(json_value).unwrap();
            *self = App::new(unsafe { ui_menu(new_menu) }, self.default_menu.clone());
        }
    }

    /// Reset all menus to defaults
    pub fn on_r(&mut self) {
        *self = App::new(self.default_menu.0.clone(), self.default_menu.clone());
    }

    pub fn on_zl(&mut self) {
        if self.page == AppPage::SUBMENU {
            self.tabs.previous();
            self.set_sub_menu_items();
        }
    }

    pub fn on_zr(&mut self) {
        if self.page == AppPage::SUBMENU {
            self.tabs.next();
            self.set_sub_menu_items();
        }
    }

    pub fn on_up(&mut self) {
        if self.page == AppPage::SUBMENU {
            self.menu_items
                .get_mut(
                    self.tabs
                        .items
                        .get(self.tabs.state.selected().unwrap())
                        .unwrap(),
                )
                .unwrap()
                .previous();
            self.set_sub_menu_items();
        } else if self.page == AppPage::TOGGLE || self.page == AppPage::SLIDER {
            self.sub_menu_previous();
        }
    }

    pub fn on_down(&mut self) {
        if self.page == AppPage::SUBMENU {
            self.menu_items
                .get_mut(
                    self.tabs
                        .items
                        .get(self.tabs.state.selected().unwrap())
                        .unwrap(),
                )
                .unwrap()
                .next();
            self.set_sub_menu_items();
        } else if self.page == AppPage::TOGGLE || self.page == AppPage::SLIDER {
            self.sub_menu_next();
        }
    }

    pub fn on_left(&mut self) {
        if self.page == AppPage::SUBMENU {
            self.menu_items
                .get_mut(
                    self.tabs
                        .items
                        .get(self.tabs.state.selected().unwrap())
                        .unwrap(),
                )
                .unwrap()
                .previous_list();
            self.set_sub_menu_items();
        } else if self.page == AppPage::TOGGLE || self.page == AppPage::SLIDER {
            self.sub_menu_previous_list();
        }
    }

    pub fn on_right(&mut self) {
        if self.page == AppPage::SUBMENU {
            self.menu_items
                .get_mut(
                    self.tabs
                        .items
                        .get(self.tabs.state.selected().unwrap())
                        .unwrap(),
                )
                .unwrap()
                .next_list();
            self.set_sub_menu_items();
        } else if self.page == AppPage::TOGGLE || self.page == AppPage::SLIDER {
            self.sub_menu_next_list();
        }
    }

    /// Returns JSON representation of current menu settings
    pub fn to_json(&self) -> String {
        let mut settings = Map::new();
        for key in self.menu_items.keys() {
            for list in &self.menu_items.get(key).unwrap().lists {
                for sub_menu in &list.items {
                    if !sub_menu.toggles.is_empty() {
                        let val: u32 = sub_menu
                            .toggles
                            .iter()
                            .filter(|t| t.checked)
                            .map(|t| t.toggle_value)
                            .sum();
                        settings.insert(sub_menu.submenu_id.to_string(), json!(val));
                    } else if sub_menu.slider.is_some() {
                        let s: &Slider = sub_menu.slider.as_ref().unwrap();
                        let val: Vec<u32> = vec![s.selected_min, s.selected_max];
                        settings.insert(sub_menu.submenu_id.to_string(), json!(val));
                    } else {
                        panic!("Could not collect settings for {:?}", sub_menu.submenu_id);
                    }
                }
            }
        }
        serde_json::to_string(&settings).unwrap()
    }


    /// Returns the current menu selections and the default menu selections.
    pub fn get_menu_selections(&self) -> String {
        serde_json::to_string(
            &MenuJsonStruct {
            menu: serde_json::from_str(self.to_json().as_str()).unwrap(),
            defaults_menu: serde_json::from_str(self.default_menu.1.clone().as_str()).unwrap(),
        }).unwrap()
    }

    pub fn submenu_ids(&self) -> Vec<&str> {
        return self.menu_items
        .values()
        .flat_map(|multi_stateful_list| {
            multi_stateful_list
                .lists
                .iter()
                .flat_map(|sub_stateful_list| {
                    sub_stateful_list
                        .items
                        .iter()
                        .map(|submenu| submenu.submenu_id)
                })
        })
        .collect::<Vec<&str>>();
    }
}

fn render_submenu_page<B: Backend>(f: &mut Frame<B>, app: &mut App, list_chunks: Vec<Rect>, help_chunk: Rect) {
    let tab_selected = app.tab_selected();
    let mut item_help = None;
    for (list_section, stateful_list) in app
        .menu_items
        .get(tab_selected)
        .unwrap()
        .lists
        .iter()
        .enumerate()
    {
        let items: Vec<ListItem> = stateful_list
            .items
            .iter()
            .map(|i| {
                let lines = vec![Spans::from(if stateful_list.state.selected().is_some() {
                    i.submenu_title.to_owned()
                } else {
                    "   ".to_owned() + i.submenu_title
                })];
                ListItem::new(lines).style(Style::default().fg(Color::White))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title(if list_section == 0 { "Options" } else { "" })
                    .style(Style::default().fg(Color::LightRed)),
            )
            .highlight_style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        let mut state = stateful_list.state.clone();
        if state.selected().is_some() {
            item_help = Some(stateful_list.items[state.selected().unwrap()].help_text);
        }

        f.render_stateful_widget(list, list_chunks[list_section], &mut state);
    }

    let help_paragraph = Paragraph::new(
        item_help.unwrap_or("").replace('\"', "")
            + "\nZL/ZR: Next tab | X: Save Defaults | R: Reset All Menus",
    )
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(help_paragraph, help_chunk);
}

pub fn render_toggle_page<B: Backend>(f: &mut Frame<B>, app: &mut App, list_chunks: Vec<Rect>, help_chunk: Rect) {
    let (title, help_text, mut sub_menu_str_lists) = app.sub_menu_strs_and_states();
    for list_section in 0..sub_menu_str_lists.len() {
        let sub_menu_str = sub_menu_str_lists[list_section].0.clone();
        let sub_menu_state = &mut sub_menu_str_lists[list_section].1;
        let values_items: Vec<ListItem> = sub_menu_str
            .iter()
            .map(|s| {
                ListItem::new(vec![Spans::from(
                    (if s.0 { "X " } else { "  " }).to_owned() + s.1,
                )])
            })
            .collect();

        let values_list = List::new(values_items)
            .block(Block::default().title(if list_section == 0 { title } else { "" }))
            .start_corner(Corner::TopLeft)
            .highlight_style(
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
        f.render_stateful_widget(values_list, list_chunks[list_section], sub_menu_state);
    }
    let help_paragraph = Paragraph::new(
        help_text.replace('\"', "") + "\nL: Reset Current Menu",
    )
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(help_paragraph, help_chunk);
}


pub fn render_slider_page<B: Backend>(f: &mut Frame<B>, app: &mut App, vertical_chunk: Rect, help_chunk: Rect) {
    let (_title, help_text, gauge_vals) = app.sub_menu_strs_for_slider();
    let abs_min = gauge_vals.abs_min;
    let abs_max = gauge_vals.abs_max;
    let selected_min = gauge_vals.selected_min;
    let selected_max = gauge_vals.selected_max;
    let lbl_ratio = 0.95; // Needed so that the upper limit label is visible
    let constraints = [
        Constraint::Ratio((lbl_ratio * (selected_min-abs_min) as f32) as u32, abs_max-abs_min),
        Constraint::Ratio((lbl_ratio * (selected_max-selected_min) as f32) as u32, abs_max-abs_min),
        Constraint::Ratio((lbl_ratio * (abs_max-selected_max) as f32) as u32, abs_max-abs_min),
        Constraint::Min(3), // For upper limit label
    ];
    let gauge_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(vertical_chunk);

    let slider_lbls = [
        abs_min,
        selected_min,
        selected_max,
        abs_max,
    ];
    for (idx, lbl) in slider_lbls.iter().enumerate() {
        let mut line_set = tui::symbols::line::NORMAL;
        line_set.horizontal = "-";
        let mut gauge = LineGauge::default()
            .ratio(1.0)
            .label(format!("{}", lbl))
            .style(Style::default().fg(Color::White))
            .line_set(line_set)
            .gauge_style(Style::default().fg(Color::White).bg(Color::Black));
        if idx == 1 {
            // Slider between selected_min and selected_max
            match gauge_vals.state {
                GaugeState::MinHover => {
                    gauge = gauge.style(Style::default().fg(Color::Red))
                }
                GaugeState::MinSelected => {
                    gauge = gauge.style(Style::default().fg(Color::Green))
                }
                _ => {}
            }
            gauge = gauge.gauge_style(Style::default().fg(Color::Yellow).bg(Color::Black));
        } else if idx == 2 {
            // Slider between selected_max and abs_max
            match gauge_vals.state {
                GaugeState::MaxHover => {
                    gauge = gauge.style(Style::default().fg(Color::Red))
                }
                GaugeState::MaxSelected => {
                    gauge = gauge.style(Style::default().fg(Color::Green))
                }
                _ => {}
            }
        } else if idx == 3 {
            // Slider for abs_max
            // We only want the label to show, so set the line character to " "
            let mut line_set = tui::symbols::line::NORMAL;
            line_set.horizontal = " ";
            gauge = gauge.line_set(line_set);

            // For some reason, the selected_max slider displays on top
            // So we need to change the abs_max slider styling to match
            // If the selected_max is close enough to the abs_max
            if (selected_max as f32 / abs_max as f32) > 0.95 {
                gauge = gauge.style(match gauge_vals.state {
                    GaugeState::MaxHover => Style::default().fg(Color::Red),
                    GaugeState::MaxSelected => Style::default().fg(Color::Green),
                    _ => Style::default(),
                })
            }
        }
        f.render_widget(gauge, gauge_chunks[idx]);
    }

    let help_paragraph = Paragraph::new(
        help_text.replace('\"', "") + "\nL: Reset Current Menu",
    )
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(help_paragraph, help_chunk);
}

/// Run
pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let app_tabs = &app.tabs;
    let tab_selected = app_tabs.state.selected().unwrap();
    let mut span_selected = Spans::default();

    let titles: Vec<Spans> = app_tabs
        .items
        .iter()
        .cloned()
        .enumerate()
        .map(|(idx, tab)| {
            if idx == tab_selected {
                span_selected = Spans::from("> ".to_owned() + tab);
                Spans::from("> ".to_owned() + tab)
            } else {
                Spans::from("  ".to_owned() + tab)
            }
        })
        .collect();
    // There is only enough room to display 3 tabs of text
    // So lets replace tabs not near the selected with "..."
    let all_windows: Vec<&[Spans]> = titles
        .windows(3)
        .filter(|w| w.contains(&titles[tab_selected]))
        .collect();
    let first_window = all_windows[0];
    let mut titles: Vec<Spans> = titles
        .iter()
        .cloned()
        .map(
            // Converts all tabs not in the window to "..."
            |t| {
                if first_window.contains(&t) {
                    t
                } else {
                    Spans::from("...".to_owned())
                }
            },
        )
        .collect();
    // Don't keep consecutive "..." tabs
    titles.dedup();
    // Now that the size of the titles vector has changed, need to re-locate the selected tab
    let tab_selected_deduped: usize = titles
        .iter()
        .cloned()
        .position(|span| span == span_selected)
        .unwrap_or(0);

    let tabs = Tabs::new(titles)
        .block(Block::default().title(Spans::from(Span::styled(
            "Ultimate Training Modpack Menu",
            Style::default().fg(Color::LightRed),
        ))))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider("|")
        .select(tab_selected_deduped);

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Max(10),
                Constraint::Length(2),
            ]
            .as_ref(),
        )
        .split(f.size());

    // Prevent overflow by adding a length constraint of NX_TUI_WIDTH
    // Need to add a second constraint since the .expand_to_fill() method
    // is not publicly exposed, and the attribute defaults to true.
    // https://github.com/fdehau/tui-rs/blob/v0.19.0/src/layout.rs#L121
    let vertical_chunks: Vec<Rect> = vertical_chunks
    .iter()
    .map(|chunk| {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(NX_TUI_WIDTH), // Width of the TUI terminal
                    Constraint::Min(0), // Fill the remainder margin
                ]
                .as_ref(),
            )
            .split(*chunk)[0]
        }
    )
    .collect();


    let list_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(vertical_chunks[1]);

    f.render_widget(tabs, vertical_chunks[0]);

    match app.page {
        AppPage::SUBMENU => render_submenu_page(f, app, list_chunks, vertical_chunks[2]),
        AppPage::SLIDER => render_slider_page(f, app, vertical_chunks[1], vertical_chunks[2]),
        AppPage::TOGGLE => render_toggle_page(f, app, list_chunks, vertical_chunks[2]),
        AppPage::CONFIRMATION => todo!()
    }
}