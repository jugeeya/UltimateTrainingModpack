use training_mod_consts::{Slider, SubMenu, SubMenuType, Toggle, UiMenu};
use tui::{
    backend::{Backend},
    layout::{Constraint, Corner, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Tabs, Paragraph, Block, List, ListItem, ListState},
    Frame,
};

pub use tui::{backend::TestBackend, Terminal, style::Color};
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
    pub selected_sub_menu_sliders: MultiStatefulList<Slider>,
    pub outer_list: bool
}

impl<'a> App<'a> {
    pub fn new(menu: UiMenu<'a>) -> App<'a> {
        let num_lists = 3;

        let mut menu_items_stateful = HashMap::new();
        menu.tabs.iter().for_each(|tab| {
            menu_items_stateful.insert(
                tab.tab_title,
                MultiStatefulList::with_items(tab.tab_submenus.clone(), num_lists)
            );
        });

        let mut app = App {
            tabs: StatefulList::with_items(menu.tabs.iter().map(|tab| tab.tab_title).collect()),
            menu_items: menu_items_stateful,
            selected_sub_menu_toggles: MultiStatefulList::with_items(vec![], 0),
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
        // let sliders = selected_sub_menu.sliders.clone();
        match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::TOGGLE => {
                self.selected_sub_menu_toggles = MultiStatefulList::with_items(
                    toggles,
                    if selected_sub_menu.toggles.len() >= 3 { 3 } else { selected_sub_menu.toggles.len()} )
            },
            SubMenuType::SLIDER => {
                // self.selected_sub_menu_sliders = MultiStatefulList::with_items(
                //     sliders,
                //     if selected_sub_menu.sliders.len() >= 3 { 3 } else { selected_sub_menu.sliders.len()} )
            },
        };
    }

    fn tab_selected(&self) -> &str {
        self.tabs.items.get(self.tabs.state.selected().unwrap()).unwrap()
    }

    fn sub_menu_selected(&self) -> &SubMenu {
        let (list_section, list_idx) = self.menu_items.get(self.tab_selected()).unwrap().idx_to_list_idx(self.menu_items.get(self.tab_selected()).unwrap().state);
        self.menu_items.get(self.tab_selected()).unwrap().lists[list_section].items.get(list_idx).unwrap()
    }

    pub fn sub_menu_next(&mut self) {
        match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::TOGGLE => self.selected_sub_menu_toggles.next(),
            SubMenuType::SLIDER => self.selected_sub_menu_sliders.next(),
        }
    }

    pub fn sub_menu_next_list(&mut self) {
        match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::TOGGLE => self.selected_sub_menu_toggles.next_list(),
            SubMenuType::SLIDER => self.selected_sub_menu_sliders.next_list(),
        }
    }

    pub fn sub_menu_previous(&mut self) {
        match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::TOGGLE => self.selected_sub_menu_toggles.previous(),
            SubMenuType::SLIDER => self.selected_sub_menu_sliders.previous(),
        }
    }

    pub fn sub_menu_previous_list(&mut self) {
        match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::TOGGLE => self.selected_sub_menu_toggles.previous_list(),
            SubMenuType::SLIDER => self.selected_sub_menu_sliders.previous_list(),
        }
    }

    pub fn sub_menu_strs_and_states(&mut self) -> (&str, &str, Vec<(Vec<(bool, &str)>, ListState)>) {
        (self.sub_menu_selected().submenu_title, self.sub_menu_selected().help_text,
         match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::TOGGLE => {
                self.selected_sub_menu_toggles.lists.iter().map(|toggle_list| {
                    (toggle_list.items.iter().map(
                        |toggle| (toggle.checked, toggle.toggle_title)
                    ).collect(), toggle_list.state.clone())
                }).collect()
            },
            SubMenuType::SLIDER => {
                vec![(vec![], ListState::default())]
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
                    let is_single_option = selected_sub_menu.is_single_option;
                    let state = self.selected_sub_menu_toggles.state;
                    self.selected_sub_menu_toggles.lists.iter_mut()
                        .map(|list| (list.state.selected(), &mut list.items))
                        .for_each(|(state, toggle_list)| toggle_list.iter_mut()
                            .enumerate()
                            .for_each(|(i, o)|
                            if state.is_some() && i == state.unwrap() {
                               if !o.checked {
                                   o.checked = true;
                               } else {
                                   o.checked = false;
                               }
                            } else if is_single_option {
                                o.checked = false;
                            }
                        ));
                    selected_sub_menu.toggles.iter_mut()
                        .enumerate()
                        .for_each(|(i, o)| {
                            if i == state  {
                                if !o.checked {
                                    o.checked = true;
                                } else {
                                    o.checked = false;
                                }
                            } else if is_single_option {
                                o.checked = false;
                            }
                        });
                },
                SubMenuType::SLIDER => {
                    // self.selected_sub_menu_sliders.selected_list_item().checked = true;
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
        .block(Block::default()
            .title(
                Spans::from(
                    Span::styled("Ultimate Training Modpack Menu",
                                Style::default().fg(Color::LightRed)))))
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
        for (list_section, stateful_list) in app.menu_items.get(tab_selected).unwrap().lists.iter().enumerate() {
            let items: Vec<ListItem> = stateful_list
                .items
                .iter()
                .map(|i| {
                    let lines = vec![Spans::from(
                        if stateful_list.state.selected().is_some() {
                            i.submenu_title.to_owned()
                        } else {
                            "   ".to_owned() + i.submenu_title
                        })];
                    ListItem::new(lines).style(Style::default().fg(Color::White))
                })
                .collect();

            let list = List::new(items)
                .block(Block::default()
                    .title(if list_section == 0 { "Options" } else { "" })
                    .style(Style::default().fg(Color::LightRed)))
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

        // TODO: Add Save Defaults
        let help_paragraph = Paragraph::new(
            item_help.unwrap_or("").replace('\"', "") +
            "\nA: Enter sub-menu | B: Exit menu | ZL/ZR: Next tab"
        ).style(Style::default().fg(Color::Cyan));
        f.render_widget(help_paragraph, vertical_chunks[2]);
    } else {
        let (title, help_text, mut sub_menu_str_lists) = app.sub_menu_strs_and_states();
        for list_section in 0..sub_menu_str_lists.len() {
            let sub_menu_str = sub_menu_str_lists[list_section].0.clone();
            let sub_menu_state = &mut sub_menu_str_lists[list_section].1;
            let values_items: Vec<ListItem> = sub_menu_str.iter().map(|s| {
                ListItem::new(
                    vec![
                        Spans::from((if s.0 { "X " } else { "  " }).to_owned() + s.1)
                    ]
                )
            }).collect();

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
            help_text.replace('\"', "") +
                "\nA: Select toggle | B: Exit submenu"
        ).style(Style::default().fg(Color::Cyan));
        f.render_widget(help_paragraph, vertical_chunks[2]);
    }


    let mut url = "http://localhost/".to_owned();
    let mut settings = HashMap::new();

    // Collect settings for toggles
    for key in app.menu_items.keys() {
        for list in &app.menu_items.get(key).unwrap().lists {
            for sub_menu in &list.items {
                let val : usize = sub_menu.toggles.iter()
                    .filter(|t| t.checked)
                    .map(|t| t.toggle_value)
                    .sum();

                settings.insert(sub_menu.submenu_id, val);
            }
        }
    }

    url.push('?');
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