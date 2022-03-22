use training_mod_consts::{OnOffSelector, Slider, SubMenu, SubMenuType, Toggle};
use tui::{
    backend::{Backend},
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Tabs, Paragraph, Block, List, ListItem, ListState},
    Frame,
};

pub use tui::{backend::TestBackend, Terminal};
use std::collections::HashMap;

mod list;

use crate::list::{StatefulList, MultiStatefulList};

/// We should hold a list of SubMenus.
/// The currently selected SubMenu should also have an associated list with necessary information.
/// We can convert the option types (Toggle, OnOff, Slider) to lists
pub struct App<'a> {
    pub tabs: StatefulList<&'a str>,
    pub menu_items: HashMap<&'a str, MultiStatefulList<SubMenu<'a>>>,
    pub selected_sub_menu_toggles: MultiStatefulList<Toggle<'a>>,
    pub selected_sub_menu_onoff_selectors: MultiStatefulList<OnOffSelector<'a>>,
    pub selected_sub_menu_sliders: MultiStatefulList<Slider>,
    pub outer_list: bool
}

impl<'a> App<'a> {
    pub fn new(menu: training_mod_consts::Menu<'a>) -> App<'a> {
        let tab_specifiers = vec![
            ("Mash Settings", vec![
                "Mash Toggles",
                "Followup Toggles",
                "Attack Angle",
                "Ledge Options",
                "Ledge Delay",
                "Tech Options",
                "Miss Tech Options",
                "Defensive Options",
                "Aerial Delay",
                "OoS Offset",
                "Reaction Time",
            ]),
            ("Defensive Settings", vec![
                "Fast Fall",
                "Fast Fall Delay",
                "Falling Aerials",
                "Full Hop",
                "Shield Tilt",
                "DI Direction",
                "SDI Direction",
                "Airdodge Direction",
                "SDI Strength",
                "Shield Toggles",
                "Mirroring",
                "Throw Options",
                "Throw Delay",
                "Pummel Delay",
                "Buff Options",
            ]),
            ("Other Settings", vec![
                "Input Delay",
                "Save States",
                "Save Damage",
                "Hitbox Visualization",
                "Stage Hazards",
                "Frame Advantage",
                "Mash In Neutral",
                "Quick Menu"
            ])
        ];
        let mut tabs: std::collections::HashMap<&str, Vec<SubMenu>> = std::collections::HashMap::new();
        tabs.insert("Mash Settings", vec![]);
        tabs.insert("Defensive Settings", vec![]);
        tabs.insert("Other Settings", vec![]);

        for sub_menu in menu.sub_menus.iter() {
            for tab_spec in tab_specifiers.iter() {
                if tab_spec.1.contains(&sub_menu.title) {
                    tabs.get_mut(tab_spec.0).unwrap().push(sub_menu.clone());
                }
            }
        };
        let num_lists = 3;

        let mut menu_items_stateful = HashMap::new();
        tabs.keys().for_each(|k| {
            menu_items_stateful.insert(
                k.clone(),
                MultiStatefulList::with_items(tabs.get(k).unwrap().clone(), num_lists)
            );
        });
        let mut app = App {
            tabs: StatefulList::with_items(tab_specifiers.iter().map(|(tab_title, _)| *tab_title).collect()),
            menu_items: menu_items_stateful,
            selected_sub_menu_toggles: MultiStatefulList::with_items(vec![], 0),
            selected_sub_menu_onoff_selectors: MultiStatefulList::with_items(vec![], 0),
            selected_sub_menu_sliders: MultiStatefulList::with_items(vec![], 0),
            outer_list: true
        };
        app.set_sub_menu_items();
        app
    }

    pub fn set_sub_menu_items(&mut self) {
        let (list_section, list_idx) = self.menu_items.get(self.tab_selected()).unwrap().idx_to_list_idx(self.menu_items.get(self.tab_selected()).unwrap().state);
        let selected_sub_menu = &self.menu_items.get(self.tab_selected()).unwrap().lists[list_section].items.get(list_idx).unwrap();

        let toggles = selected_sub_menu.toggles.clone();
        let sliders = selected_sub_menu.sliders.clone();
        let onoffs = selected_sub_menu.onoffselector.clone();
        match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::TOGGLE => {
                self.selected_sub_menu_toggles = MultiStatefulList::with_items(
                    toggles,
                    if selected_sub_menu.toggles.len() >= 3 { 3 } else { selected_sub_menu.toggles.len()} )
            },
            SubMenuType::SLIDER => {
                self.selected_sub_menu_sliders = MultiStatefulList::with_items(
                    sliders,
                    if selected_sub_menu.sliders.len() >= 3 { 3 } else { selected_sub_menu.sliders.len()} )
            },
            SubMenuType::ONOFF => {
                self.selected_sub_menu_onoff_selectors = MultiStatefulList::with_items(
                    onoffs,
                    if selected_sub_menu.onoffselector.len() >= 3 { 3 } else { selected_sub_menu.onoffselector.len()} )
            },
        };
    }

    fn tab_selected(&self) -> &str {
        self.tabs.items.get(self.tabs.state.selected().unwrap()).unwrap()
    }

    fn sub_menu_selected(&self) -> &SubMenu {
        let (list_section, list_idx) = self.menu_items.get(self.tab_selected()).unwrap().idx_to_list_idx(self.menu_items.get(self.tab_selected()).unwrap().state);
        &self.menu_items.get(self.tab_selected()).unwrap().lists[list_section].items.get(list_idx).unwrap()
    }

    pub fn sub_menu_next(&mut self) {
        match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::TOGGLE => self.selected_sub_menu_toggles.next(),
            SubMenuType::SLIDER => self.selected_sub_menu_sliders.next(),
            SubMenuType::ONOFF => self.selected_sub_menu_onoff_selectors.next(),
        }
    }

    pub fn sub_menu_next_list(&mut self) {
        match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::TOGGLE => self.selected_sub_menu_toggles.next_list(),
            SubMenuType::SLIDER => self.selected_sub_menu_sliders.next_list(),
            SubMenuType::ONOFF => self.selected_sub_menu_onoff_selectors.next_list(),
        }
    }

    pub fn sub_menu_previous(&mut self) {
        match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::TOGGLE => self.selected_sub_menu_toggles.previous(),
            SubMenuType::SLIDER => self.selected_sub_menu_sliders.previous(),
            SubMenuType::ONOFF => self.selected_sub_menu_onoff_selectors.previous(),
        }
    }

    pub fn sub_menu_previous_list(&mut self) {
        match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::TOGGLE => self.selected_sub_menu_toggles.previous_list(),
            SubMenuType::SLIDER => self.selected_sub_menu_sliders.previous_list(),
            SubMenuType::ONOFF => self.selected_sub_menu_onoff_selectors.previous_list(),
        }
    }

    pub fn sub_menu_strs_and_states(&mut self) -> (&str, &str, Vec<(Vec<(&str, &str)>, ListState)>) {
        (self.sub_menu_selected().title, self.sub_menu_selected().help_text,
         match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::TOGGLE => {
                self.selected_sub_menu_toggles.lists.iter().map(|toggle_list| {
                    (toggle_list.items.iter().map(
                        |toggle| (toggle.checked, toggle.title)
                    ).collect(), toggle_list.state.clone())
                }).collect()
            },
            SubMenuType::SLIDER => {
                vec![(vec![], ListState::default())]
            },
            SubMenuType::ONOFF => {
                self.selected_sub_menu_onoff_selectors.lists.iter().map(|onoff_list| {
                    (onoff_list.items.iter().map(
                        |onoff| (onoff.checked, onoff.title)
                    ).collect(), onoff_list.state.clone())
                }).collect()
            },
        })
    }

    pub fn on_a(&mut self) {
        if self.outer_list {
            self.outer_list = false;
        } else {
            let tab_selected = self.tabs.items.get(self.tabs.state.selected().unwrap()).unwrap();
            let (list_section, list_idx) = self.menu_items.get(tab_selected)
                .unwrap()
                .idx_to_list_idx(self.menu_items.get(tab_selected).unwrap().state);
            let selected_sub_menu = self.menu_items.get_mut(tab_selected)
                .unwrap()
                .lists[list_section]
                .items.get_mut(list_idx).unwrap();
            match SubMenuType::from_str(selected_sub_menu._type) {
                SubMenuType::TOGGLE => {
                    let is_single_option = selected_sub_menu.is_single_option.is_some();
                    let state = self.selected_sub_menu_toggles.state;
                    self.selected_sub_menu_toggles.lists.iter_mut()
                        .map(|list| (list.state.selected(), &mut list.items))
                        .for_each(|(state, toggle_list)| toggle_list.iter_mut()
                            .enumerate()
                            .for_each(|(i, o)|
                            if state.is_some() && i == state.unwrap() {
                               if o.checked != "is-appear" {
                                   o.checked = "is-appear";
                               } else {
                                   o.checked = "is-hidden";
                               }
                            } else if is_single_option {
                                o.checked = "is-hidden";
                            }
                        ));
                    selected_sub_menu.toggles.iter_mut()
                        .enumerate()
                        .for_each(|(i, o)| {
                            if i == state  {
                                if o.checked != "is-appear" {
                                    o.checked = "is-appear";
                                } else {
                                    o.checked = "is-hidden";
                                }
                            } else if is_single_option {
                                o.checked = "is-hidden";
                            }
                        });
                },
                SubMenuType::ONOFF => {
                    let onoff = self.selected_sub_menu_onoff_selectors.selected_list_item();
                    if onoff.checked != "is-appear" {
                        onoff.checked = "is-appear";
                    } else {
                        onoff.checked = "is-hidden";
                    }
                    selected_sub_menu.onoffselector.iter_mut()
                        .filter(|o| o.title == onoff.title)
                        .for_each(|o| o.checked = onoff.checked);
                },
                SubMenuType::SLIDER => {
                    // self.selected_sub_menu_sliders.selected_list_item().checked = "is-appear";
                }
            }
        }
    }

    pub fn on_b(&mut self) {
        self.outer_list = true;
    }

    pub fn on_l(&mut self) {
        if self.outer_list {
            self.tabs.previous();
            self.set_sub_menu_items();
        }
    }

    pub fn on_r(&mut self) {
        if self.outer_list {
            self.tabs.next();
            self.set_sub_menu_items();
        }
    }

    pub fn on_up(&mut self) {
        if self.outer_list {
            self.menu_items.get_mut(self.tabs.items.get(self.tabs.state.selected().unwrap()).unwrap()).unwrap().previous();
            self.set_sub_menu_items();
        } else {
            self.sub_menu_previous();
        }
    }

    pub fn on_down(&mut self) {
        if self.outer_list {
            self.menu_items.get_mut(self.tabs.items.get(self.tabs.state.selected().unwrap()).unwrap()).unwrap().next();
            self.set_sub_menu_items();
        } else {
            self.sub_menu_next();
        }
    }

    pub fn on_left(&mut self) {
        if self.outer_list {
            self.menu_items.get_mut(self.tabs.items.get(self.tabs.state.selected().unwrap()).unwrap()).unwrap().previous_list();
            self.set_sub_menu_items();
        } else {
            self.sub_menu_previous_list();
        }
    }

    pub fn on_right(&mut self) {
        if self.outer_list {
            self.menu_items.get_mut(self.tabs.items.get(self.tabs.state.selected().unwrap()).unwrap()).unwrap().next_list();
            self.set_sub_menu_items();
        } else {
            self.sub_menu_next_list();
        }
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) -> String {
    let app_tabs = &app.tabs;
    let tab_selected = app_tabs.state.selected().unwrap();
    let titles = app_tabs.items.iter().cloned().enumerate().map(|(idx, tab)|{
        if idx == tab_selected {
            Spans::from(">> ".to_owned() + tab)
        } else {
            Spans::from("  ".to_owned() + tab)
        }
    }).collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().title("Ultimate Training Modpack Menu"))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider("|")
        .select(tab_selected);

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Max(10),
            Constraint::Length(2)].as_ref())
        .split(f.size());

    let list_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(32), Constraint::Percentage(33)].as_ref())
        .split(vertical_chunks[1]);

    f.render_widget(tabs, vertical_chunks[0]);

    if app.outer_list {
        let tab_selected = app.tab_selected();
        let mut item_help = None;
        for list_section in 0..app.menu_items.get(tab_selected).unwrap().lists.len() {
            let stateful_list = &app.menu_items.get(tab_selected).unwrap().lists[list_section];
            let items: Vec<ListItem> = stateful_list
                .items
                .iter()
                .map(|i| {
                    let lines = vec![Spans::from(
                        if stateful_list.state.selected().is_some() {
                            i.title.to_owned()
                        } else {
                            "   ".to_owned() + i.title
                        })];
                    ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().title(if list_section == 0 { "Options" } else { "" }))
                .highlight_style(
                    Style::default()
                        .bg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            let mut state = stateful_list.state.clone();
            if state.selected().is_some() {
                item_help = Some(stateful_list.items[state.selected().unwrap()].help_text);
            }

            f.render_stateful_widget(list, list_chunks[list_section], &mut state);
        }

        // TODO: Add Save Defaults
        let help_paragraph = Paragraph::new(
            item_help.unwrap_or("").replace("\"", "") +
            "\nA: Enter sub-menu | B: Exit menu | ZL/ZR: Next tab"
        );
        f.render_widget(help_paragraph, vertical_chunks[2]);
    } else {
        let (title, help_text, mut sub_menu_str_lists) = app.sub_menu_strs_and_states();
        for list_section in 0..sub_menu_str_lists.len() {
            let sub_menu_str = sub_menu_str_lists[list_section].0.clone();
            let sub_menu_state = &mut sub_menu_str_lists[list_section].1;
            let values_items: Vec<ListItem> = sub_menu_str.iter().map(|s| {
                ListItem::new(
                    vec![
                        Spans::from((if s.0 == "is-appear" { "X " } else { "  " }).to_owned() + s.1)
                    ]
                )
            }).collect();

            let values_list = List::new(values_items)
                .block(Block::default().title(if list_section == 0 { title } else { "" }))
                .start_corner(Corner::TopLeft)
                .highlight_style(
                    Style::default()
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");
            f.render_stateful_widget(values_list, list_chunks[list_section], sub_menu_state);
        }

        let help_paragraph = Paragraph::new(
            help_text.replace("\"", "") +
                "\nA: Select toggle | B: Exit submenu"
        );
        f.render_widget(help_paragraph, vertical_chunks[2]);
    }


    let mut url = "http://localhost/".to_owned();
    let mut settings = HashMap::new();

    // Collect settings for toggles
    for key in app.menu_items.keys() {
        for list in &app.menu_items.get(key).unwrap().lists {
            for sub_menu in &list.items {
                let mut val = String::new();
                sub_menu.toggles.iter()
                    .filter(|t| t.checked == "is-appear")
                    .for_each(|t| val.push_str(format!("{},", t.value).as_str()));

                sub_menu.onoffselector.iter()
                    .for_each(|o| {
                        val.push_str(
                            format!("{}", if o.checked == "is-appear" { 1 } else { 0 }).as_str())
                    });
                settings.insert(sub_menu.id, val);
            }
        }
    }

    url.push_str("?");
    settings.iter()
        .for_each(|(section, val)| url.push_str(format!("{}={}&", section, val).as_str()));
    url

    // TODO: Add saveDefaults
    // if (document.getElementById("saveDefaults").checked) {
    //     url += "save_defaults=1";
    // } else {
    //     url = url.slice(0, -1);
    // }
}