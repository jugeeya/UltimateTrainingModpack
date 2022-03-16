use training_mod_consts::{OnOffSelector, Slider, SubMenu, SubMenuType, Toggle};
use tui::{
    backend::{Backend},
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, List, ListItem, ListState},
    Frame,
};

pub use tui::{backend::TestBackend, Terminal};
use tui::widgets::Tabs;
use std::collections::HashMap;

pub struct MultiStatefulList<T> {
    pub lists: Vec<StatefulList<T>>,
    pub state: usize,
    pub total_len: usize
}

impl<T: Clone> MultiStatefulList<T> {
    pub fn selected_list_item(&mut self) -> &mut T {
        let (list_section, list_idx) = self.idx_to_list_idx(self.state);
        &mut self.lists[list_section].items[list_idx]
    }

    fn idx_to_list_idx(&self, idx: usize) -> (usize, usize) {
        let mut list_idx = idx;
        for list_section in 0..self.lists.len() {
            let list_section_min_idx = (self.total_len / self.lists.len()) * list_section;
            let list_section_max_idx = (self.total_len / self.lists.len()) * (list_section + 1);
            if (list_section_min_idx..list_section_max_idx).contains(&idx) {
                return (list_section, idx - list_section_min_idx)
            }
        }
        (0, 0)
    }

    fn list_idx_to_idx(&self, list_idx: (usize, usize)) -> usize {
        let list_section = list_idx.0;
        let mut list_idx = list_idx.1;
        for list_section in 0..list_section {
            list_idx += self.lists[list_section].items.len();
        }
        list_idx
    }

    pub fn with_items(items: Vec<T>, num_lists: usize) -> MultiStatefulList<T> {
        let lists = (0..num_lists).map(|list_section| {
            let list_section_min_idx = (items.len() / num_lists) * list_section;
            let list_section_max_idx = (items.len() / num_lists) * (list_section + 1);
            let mut state = ListState::default();
            if list_section == 0 {
                // Enforce state as first of list
                state.select(Some(0));
            }
            StatefulList {
                state: state,
                items: items[list_section_min_idx..list_section_max_idx].to_vec(),
            }
        }).collect();
        let total_len = items.len();
        MultiStatefulList {
            // Divide evenly into three lists
            lists: lists,
            total_len: total_len,
            state: 0
        }
    }

    pub fn next(&mut self) {
        let (list_section, list_idx) = self.idx_to_list_idx(self.state);
        let (next_list_section, next_list_idx) = self.idx_to_list_idx(self.state+1);

        if list_section != next_list_section {
            self.lists[list_section].unselect();
        }
        let state;
        if self.state + 1 >= self.total_len {
            state = (0, 0);
        } else {
            state = (next_list_section, next_list_idx);
        }

        self.lists[state.0].state.select(Some(state.1));
        self.state = self.list_idx_to_idx(state);
    }

    pub fn previous(&mut self) {
        let (list_section, list_idx) = self.idx_to_list_idx(self.state);
        let (last_list_section, last_list_idx) = (self.lists.len() - 1, self.lists[self.lists.len() - 1].items.len() - 1);

        self.lists[list_section].unselect();
        let state;
        if self.state == 0 {
            state = (last_list_section, last_list_idx);
        } else {
            let (prev_list_section, prev_list_idx) = self.idx_to_list_idx(self.state - 1);
            state = (prev_list_section, prev_list_idx);
        }

        self.lists[state.0].state.select(Some(state.1));
        self.state = self.list_idx_to_idx(state);
    }

    pub fn next_list(&mut self) {
        let (list_section, list_idx) = self.idx_to_list_idx(self.state);
        let next_list_section = (list_section + 1) % self.lists.len();
        let next_list_idx;
        if list_idx > self.lists[next_list_section].items.len() - 1 {
            next_list_idx = self.lists[next_list_section].items.len() - 1;
        } else {
            next_list_idx = list_idx;
        }

        if list_section != next_list_section {
            self.lists[list_section].unselect();
        }
        let state = (next_list_section, next_list_idx);

        self.lists[state.0].state.select(Some(state.1));
        self.state = self.list_idx_to_idx(state);
    }

    pub fn previous_list(&mut self) {
        let (list_section, list_idx) = self.idx_to_list_idx(self.state);
        let prev_list_section;
        if list_section == 0 {
            prev_list_section = self.lists.len() - 1;
        } else {
            prev_list_section = list_section - 1;
        }

        let prev_list_idx;
        if list_idx > self.lists[prev_list_section].items.len() - 1 {
            prev_list_idx = self.lists[prev_list_section].items.len() - 1;
        } else {
            prev_list_idx = list_idx;
        }

        if list_section != prev_list_section {
            self.lists[list_section].unselect();
        }
        let state = (prev_list_section, prev_list_idx);

        self.lists[state.0].state.select(Some(state.1));
        self.state = self.list_idx_to_idx(state);
    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        let mut state = ListState::default();
        // Enforce state as first of list
        state.select(Some(0));
        StatefulList {
            state: state,
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

/// We should hold a list of SubMenus.
/// The currently selected SubMenu should also have an associated list with necessary information.
/// We can convert the option types (Toggle, OnOff, Slider) to lists
pub struct App<'a> {
    pub tabs: StatefulList<&'a str>,
    all_menu_items: Vec<SubMenu<'a>>,
    pub menu_items: HashMap<&'a str, MultiStatefulList<SubMenu<'a>>>,
    pub selected_sub_menu_toggles: MultiStatefulList<Toggle<'a>>,
    pub selected_sub_menu_onoff_selectors: MultiStatefulList<OnOffSelector<'a>>,
    pub selected_sub_menu_sliders: MultiStatefulList<Slider>,
    pub outer_list: bool
}

impl<'a> App<'a> {
    pub fn new(tabs: HashMap<&'a str, Vec<SubMenu<'a>>>, menu_items: Vec<SubMenu<'a>>, num_lists: usize) -> App<'a> {
        let mut menu_items_stateful = HashMap::new();
        tabs.keys().for_each(|k| {
            menu_items_stateful.insert(
                k.clone(),
                MultiStatefulList::with_items(tabs.get(k).unwrap().clone(), num_lists)
            );
        });
        App {
            tabs: StatefulList::with_items(tabs.keys().cloned().collect()),
            all_menu_items: menu_items.clone(),
            menu_items: menu_items_stateful,
            selected_sub_menu_toggles: MultiStatefulList::with_items(vec![], 0),
            selected_sub_menu_onoff_selectors: MultiStatefulList::with_items(vec![], 0),
            selected_sub_menu_sliders: MultiStatefulList::with_items(vec![], 0),
            outer_list: true
        }
    }

    pub fn set_sub_menu_items(&mut self) {
        let (list_section, list_idx) = self.menu_items.get(self.tab_selected()).unwrap().idx_to_list_idx(self.menu_items.get(self.tab_selected()).unwrap().state);
        let selected_sub_menu = &self.menu_items.get(self.tab_selected()).unwrap().lists[list_section].items.get(list_idx).unwrap();

        let toggles = selected_sub_menu.toggles.clone();
        let sliders = selected_sub_menu.sliders.clone();
        let onoffs = selected_sub_menu.onoffselector.clone();
        match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::TOGGLE => {
                self.selected_sub_menu_toggles = MultiStatefulList::with_items(toggles, 3)
            },
            SubMenuType::SLIDER => {
                self.selected_sub_menu_sliders = MultiStatefulList::with_items(sliders, 3)
            },
            SubMenuType::ONOFF => {
                self.selected_sub_menu_onoff_selectors = MultiStatefulList::with_items(onoffs.clone(), 3)
            },
        };
    }

    fn tab_selected(&self) -> &str {
        self.tabs.items.get(self.tabs.state.selected().unwrap()).unwrap()
    }

    fn tab_selected_mut(&mut self) -> &str {
        self.tabs.items.get(self.tabs.state.selected().unwrap()).unwrap()
    }

    fn sub_menu_selected(&self) -> &SubMenu {
        let (list_section, list_idx) = self.menu_items.get(self.tab_selected()).unwrap().idx_to_list_idx(self.menu_items.get(self.tab_selected()).unwrap().state);
        &self.menu_items.get(self.tab_selected()).unwrap().lists[list_section].items.get(list_idx).unwrap()
    }

    pub fn sub_menu_next(&mut self) {
        match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::TOGGLE => {
                self.selected_sub_menu_toggles.next();
            },
            SubMenuType::SLIDER => {
                self.selected_sub_menu_sliders.next();
            },
            SubMenuType::ONOFF => {
                self.selected_sub_menu_onoff_selectors.next()
            },
        }
    }

    pub fn sub_menu_next_list(&mut self) {
        match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::TOGGLE => {
                self.selected_sub_menu_toggles.next_list();
            },
            SubMenuType::SLIDER => {
                self.selected_sub_menu_sliders.next_list();
            },
            SubMenuType::ONOFF => {
                self.selected_sub_menu_onoff_selectors.next_list()
            },
        }
    }

    pub fn sub_menu_previous(&mut self) {
        match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::TOGGLE => {
                self.selected_sub_menu_toggles.previous();
            },
            SubMenuType::SLIDER => {
                self.selected_sub_menu_sliders.previous();
            },
            SubMenuType::ONOFF => {
                self.selected_sub_menu_onoff_selectors.previous()
            },
        }
    }

    pub fn sub_menu_previous_list(&mut self) {
        match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::TOGGLE => {
                self.selected_sub_menu_toggles.previous_list();
            },
            SubMenuType::SLIDER => {
                self.selected_sub_menu_sliders.previous_list();
            },
            SubMenuType::ONOFF => {
                self.selected_sub_menu_onoff_selectors.previous_list()
            },
        }
    }

    pub fn sub_menu_strs_and_states(&mut self) -> (&str, Vec<(Vec<(&str, &str)>, ListState)>) {
        (self.sub_menu_selected().title,
        match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::TOGGLE => {
                self.selected_sub_menu_toggles.lists.iter().map(|toggle_list| {
                    (toggle_list.items.iter().map(
                        |toggle| (toggle.checked, toggle.title)
                    ).collect(), toggle_list.state.clone())
                }).collect()
            },
            SubMenuType::SLIDER => {
                // self.selected_sub_menu_sliders.items.iter().map(
                //     |_slider| ("TODO", "TODO")
                // ).collect()
                vec![(vec![], ListState::default())]
            },
            SubMenuType::ONOFF => {
                // self.selected_sub_menu_onoff_selectors.items.iter().map(
                //     |onoff_selector| (onoff_selector.checked, onoff_selector.title)
                // ).collect()
                vec![(vec![], ListState::default())]
            },
        })
    }

    pub fn sub_menu_state(&mut self) -> &ListState {
        match SubMenuType::from_str(self.sub_menu_selected()._type) {
            SubMenuType::TOGGLE => {
                let (list_section, list_idx) =
                    self.selected_sub_menu_toggles.idx_to_list_idx(self.selected_sub_menu_toggles.state);
                &self.selected_sub_menu_toggles.lists[list_section].state
            },
            SubMenuType::SLIDER => {
                let (list_section, list_idx) =
                    self.selected_sub_menu_sliders.idx_to_list_idx(self.selected_sub_menu_sliders.state);
                &self.selected_sub_menu_sliders.lists[list_section].state
            },
            SubMenuType::ONOFF => {
                let (list_section, list_idx) =
                    self.selected_sub_menu_onoff_selectors.idx_to_list_idx(self.selected_sub_menu_onoff_selectors.state);
                &self.selected_sub_menu_onoff_selectors.lists[list_section].state
            },
        }
    }

    pub fn on_l(&mut self) {
        if self.outer_list {
            self.tabs.previous();
        }
    }

    pub fn on_r(&mut self) {
        if self.outer_list {
            self.tabs.next();
        }
    }

    pub fn on_left(&mut self) {
        if self.outer_list {
            self.menu_items.get_mut(self.tabs.items.get(self.tabs.state.selected().unwrap()).unwrap()).unwrap().previous_list();
        } else {
            self.sub_menu_previous_list();
        }
    }

    pub fn on_right(&mut self) {
        if self.outer_list {
            self.menu_items.get_mut(self.tabs.items.get(self.tabs.state.selected().unwrap()).unwrap()).unwrap().next_list();
        } else {
            self.sub_menu_next_list();
        }
    }

    pub fn on_a(&mut self) {
        if self.outer_list {
            self.outer_list = false;
        } else {
            let selected_sub_menu = self.sub_menu_selected();
            match SubMenuType::from_str(selected_sub_menu._type) {
                SubMenuType::TOGGLE => {
                    let toggle = self.selected_sub_menu_toggles.selected_list_item();
                    if toggle.checked != "is-appear" {
                        toggle.checked = "is-appear";
                    } else {
                        toggle.checked = "is-hidden";
                    }
                },
                SubMenuType::ONOFF => {
                    let onoff = self.selected_sub_menu_onoff_selectors.selected_list_item();
                    if onoff.checked != "is-appear" {
                        onoff.checked = "is-appear";
                    } else {
                        onoff.checked = "is-hidden";
                    }
                },
                SubMenuType::SLIDER => {
                    // self.selected_sub_menu_sliders.selected_list_item().checked = "is-appear";
                }
                _ => {}
            }
        }
    }

    pub fn on_b(&mut self) {
        self.outer_list = true;
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
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let app_tabs = &app.tabs;
    let titles = app_tabs.items.iter().cloned().map(Spans::from).collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().title("Ultimate Training Modpack Menu"))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider("|")
        .select(app_tabs.state.selected().unwrap());

    let tab_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(f.size());

    let list_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(32), Constraint::Percentage(33)].as_ref())
        .split(tab_chunks[1]);

    f.render_widget(tabs, tab_chunks[0]);

    if app.outer_list {
        let tab_selected = app.tab_selected();
        for list_section in 0..app.menu_items.get(tab_selected).unwrap().lists.len() {
            let mut stateful_list = &app.menu_items.get(tab_selected).unwrap().lists[list_section];
            let items: Vec<ListItem> = stateful_list
                .items
                .iter()
                .map(|i| {
                    let lines = vec![Spans::from(i.title)];
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

            f.render_stateful_widget(list, list_chunks[list_section], &mut state);
        }
    } else {
        let (title, mut sub_menu_str_lists) = app.sub_menu_strs_and_states();
        for list_section in 0..sub_menu_str_lists.len() {
            let sub_menu_str = sub_menu_str_lists[list_section].0.clone();
            let mut sub_menu_state = &mut sub_menu_str_lists[list_section].1;
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
    }


    let url = "http://localhost/";
        // var settings = new Map();
        //
        // // Collect settings for toggles
        //
        // [].forEach.call(document.querySelectorAll("ul.l-grid"), function (toggle) {
        //     var section = toggle.id;
        //     var val = "";
        //
        //     [].forEach.call(toggle.childNodes, function (child) {
        //         if (!isTextNode(child) && child.querySelectorAll(".is-appear").length) {
        //             val += child.getAttribute("val") + ",";
        //         };
        //     });
        //
        //     settings.set(section,val);
        // });
        //
        // // Collect settings for OnOffs
        // [].forEach.call(document.querySelectorAll("div.onoff"), function (onoff) {
        //     var section = onoff.id;
        //     var val = "";
        //     if (onoff.querySelectorAll(".is-appear").length) {
        //         val = "1";
        //     } else {
        //         val = "0";
        //     }
        //     settings.set(section,val);
        // });
        //
        // url += "?";
        // settings.forEach((val, section) => { url += section + "=" + val + "&" } );
        //
        // if (document.getElementById("saveDefaults").checked) {
        //     url += "save_defaults=1";
        // } else {
        //     url = url.slice(0, -1);
        // }
}