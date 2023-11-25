use ratatui::widgets::{TableSelection, TableState};
use training_mod_tui_2::*;

fn make_toggle<'a>(v: u8) -> Toggle<'a> {
    Toggle {
        title: "Title",
        value: v,
        max: 4,
    }
}

fn make_toggle_table_multiple<'a>(
    rows: usize,
    cols: usize,
    num: usize,
) -> StatefulTable<Toggle<'a>> {
    // [ (0)  1  2 ]
    // [  3        ]
    let v: Vec<Toggle> = (0..num).map(|v| make_toggle(v as u8)).collect();
    StatefulTable::with_items(rows, cols, v)
}

fn make_toggle_table_single<'a>(rows: usize, cols: usize, num: usize) -> StatefulTable<Toggle<'a>> {
    // [ (1)  0  0 ]
    // [  0        ]
    let v: Vec<Toggle> = (0..num).map(|_| make_toggle(0)).collect();
    let mut t = StatefulTable::with_items(rows, cols, v);
    t.items[0][0] = Some(make_toggle(1));
    t
}

fn initialize_submenu<'a>(submenu_type: SubMenuType) -> SubMenu<'a> {
    match submenu_type {
        SubMenuType::ToggleSingle => SubMenu {
            title: "Single Option Menu",
            id: "single_option",
            help_text: "A Single Option",
            submenu_type: submenu_type,
            toggles: make_toggle_table_single(2, 3, 4),
            slider: None,
        },
        SubMenuType::ToggleMultiple => SubMenu {
            title: "Multi Option Menu",
            id: "multi_option",
            help_text: "Multiple Options",
            submenu_type: submenu_type,
            toggles: make_toggle_table_multiple(2, 3, 4),
            slider: None,
        },
        SubMenuType::Slider => SubMenu {
            title: "Slider Menu",
            id: "slider",
            help_text: "A Double-ended Slider",
            submenu_type: submenu_type,
            toggles: make_toggle_table_multiple(0, 0, 0),
            slider: Some(StatefulSlider::new()),
        },
        SubMenuType::None => {
            panic!()
        }
    }
}

#[test]
fn submenu_serialize() {
    let submenu = initialize_submenu(SubMenuType::ToggleSingle);
    let json = serde_json::to_string(&submenu).unwrap();
    assert_eq!(&json, "[1,0,0,0]");

    let submenu = initialize_submenu(SubMenuType::ToggleMultiple);
    let json = serde_json::to_string(&submenu).unwrap();
    assert_eq!(&json, "[0,1,2,3]");

    let submenu = initialize_submenu(SubMenuType::Slider);
    let json = serde_json::to_string(&submenu).unwrap();
    assert_eq!(&json, "[0,150]");
}

#[test]
fn submenu_selected_toggle() {
    let mut submenu = initialize_submenu(SubMenuType::ToggleSingle);
    let mut t = make_toggle(1);
    assert_eq!(submenu.selected_toggle(), &mut t);
    t = make_toggle(0);
    submenu.toggles.next_col();
    assert_eq!(submenu.selected_toggle(), &mut t);

    let mut submenu = initialize_submenu(SubMenuType::ToggleMultiple);
    let mut t = make_toggle(0);
    assert_eq!(submenu.selected_toggle(), &mut t);
    t = make_toggle(1);
    submenu.toggles.next_col();
    assert_eq!(submenu.selected_toggle(), &mut t);
    t = make_toggle(2);
    submenu.toggles.next_col();
    assert_eq!(submenu.selected_toggle(), &mut t);
}

#[test]
fn submenu_update_from_vec() {
    let mut submenu = initialize_submenu(SubMenuType::ToggleSingle);
    assert_eq!(submenu.toggles.items[0][0], Some(make_toggle(1)));
    assert_eq!(submenu.toggles.items[0][1], Some(make_toggle(0)));
    assert_eq!(submenu.toggles.items[0][2], Some(make_toggle(0)));
    assert_eq!(submenu.toggles.items[1][0], Some(make_toggle(0)));
    assert_eq!(submenu.toggles.items[1][1], None);
    assert_eq!(submenu.toggles.items[1][2], None);
    submenu.update_from_vec(vec![0, 0, 1, 0]);
    assert_eq!(submenu.toggles.items[0][0], Some(make_toggle(0)));
    assert_eq!(submenu.toggles.items[0][1], Some(make_toggle(0)));
    assert_eq!(submenu.toggles.items[0][2], Some(make_toggle(1)));
    assert_eq!(submenu.toggles.items[1][0], Some(make_toggle(0)));
    assert_eq!(submenu.toggles.items[1][1], None);
    assert_eq!(submenu.toggles.items[1][2], None);

    let mut submenu = initialize_submenu(SubMenuType::ToggleMultiple);
    assert_eq!(submenu.toggles.items[0][0], Some(make_toggle(0)));
    assert_eq!(submenu.toggles.items[0][1], Some(make_toggle(1)));
    assert_eq!(submenu.toggles.items[0][2], Some(make_toggle(2)));
    assert_eq!(submenu.toggles.items[1][0], Some(make_toggle(3)));
    assert_eq!(submenu.toggles.items[1][1], None);
    assert_eq!(submenu.toggles.items[1][2], None);
    submenu.update_from_vec(vec![1, 1, 0, 4]);
    assert_eq!(submenu.toggles.items[0][0], Some(make_toggle(1)));
    assert_eq!(submenu.toggles.items[0][1], Some(make_toggle(1)));
    assert_eq!(submenu.toggles.items[0][2], Some(make_toggle(0)));
    assert_eq!(submenu.toggles.items[1][0], Some(make_toggle(4)));
    assert_eq!(submenu.toggles.items[1][1], None);
    assert_eq!(submenu.toggles.items[1][2], None);

    let mut submenu = initialize_submenu(SubMenuType::Slider);
    let mut slider = StatefulSlider::new();
    assert_eq!(submenu.slider, Some(slider));
    slider.lower = 5;
    submenu.update_from_vec(vec![5, 150]);
    assert_eq!(submenu.slider, Some(slider));
    slider.upper = 75;
    submenu.update_from_vec(vec![5, 75]);
    assert_eq!(submenu.slider, Some(slider));
}

#[test]
fn submenu_single_on_a() {
    let mut submenu = initialize_submenu(SubMenuType::ToggleSingle);
    submenu.toggles.select(1, 0);
    submenu.on_a();
    assert_eq!(submenu.toggles.items[0][0], Some(make_toggle(0)));
    assert_eq!(submenu.toggles.items[0][1], Some(make_toggle(0)));
    assert_eq!(submenu.toggles.items[0][2], Some(make_toggle(0)));
    assert_eq!(submenu.toggles.items[1][0], Some(make_toggle(1)));
    assert_eq!(submenu.toggles.items[1][1], None);
    assert_eq!(submenu.toggles.items[1][2], None);
    submenu.on_a();
    assert_eq!(submenu.toggles.items[0][0], Some(make_toggle(0)));
    assert_eq!(submenu.toggles.items[0][1], Some(make_toggle(0)));
    assert_eq!(submenu.toggles.items[0][2], Some(make_toggle(0)));
    assert_eq!(submenu.toggles.items[1][0], Some(make_toggle(1)));
    assert_eq!(submenu.toggles.items[1][1], None);
    assert_eq!(submenu.toggles.items[1][2], None);
    submenu.toggles.select(0, 0);
    submenu.on_a();
    assert_eq!(submenu.toggles.items[0][0], Some(make_toggle(1)));
    assert_eq!(submenu.toggles.items[0][1], Some(make_toggle(0)));
    assert_eq!(submenu.toggles.items[0][2], Some(make_toggle(0)));
    assert_eq!(submenu.toggles.items[1][0], Some(make_toggle(0)));
    assert_eq!(submenu.toggles.items[1][1], None);
    assert_eq!(submenu.toggles.items[1][2], None);
}

#[test]
fn submenu_multiple_on_a() {
    let mut submenu = initialize_submenu(SubMenuType::ToggleMultiple);
    submenu.toggles.select(1, 0);
    submenu.on_a();
    assert_eq!(submenu.toggles.items[0][0], Some(make_toggle(0)));
    assert_eq!(submenu.toggles.items[0][1], Some(make_toggle(1)));
    assert_eq!(submenu.toggles.items[0][2], Some(make_toggle(2)));
    assert_eq!(submenu.toggles.items[1][0], Some(make_toggle(4)));
    assert_eq!(submenu.toggles.items[1][1], None);
    assert_eq!(submenu.toggles.items[1][2], None);
    submenu.on_a();
    assert_eq!(submenu.toggles.items[0][0], Some(make_toggle(0)));
    assert_eq!(submenu.toggles.items[0][1], Some(make_toggle(1)));
    assert_eq!(submenu.toggles.items[0][2], Some(make_toggle(2)));
    assert_eq!(submenu.toggles.items[1][0], Some(make_toggle(0)));
    assert_eq!(submenu.toggles.items[1][1], None);
    assert_eq!(submenu.toggles.items[1][2], None);
    submenu.toggles.select(0, 0);
    submenu.on_a();
    assert_eq!(submenu.toggles.items[0][0], Some(make_toggle(1)));
    assert_eq!(submenu.toggles.items[0][1], Some(make_toggle(1)));
    assert_eq!(submenu.toggles.items[0][2], Some(make_toggle(2)));
    assert_eq!(submenu.toggles.items[1][0], Some(make_toggle(0)));
    assert_eq!(submenu.toggles.items[1][1], None);
    assert_eq!(submenu.toggles.items[1][2], None);
}

#[test]
fn submenu_slider_on_a() {
    let mut submenu = initialize_submenu(SubMenuType::Slider);
    assert_eq!(submenu.slider.unwrap().state, SliderState::LowerHover);
    submenu.on_a();
    assert_eq!(submenu.slider.unwrap().state, SliderState::LowerSelected);
    submenu.on_a();
    assert_eq!(submenu.slider.unwrap().state, SliderState::LowerHover);
    submenu.slider = Some(StatefulSlider {
        state: SliderState::UpperHover,
        ..submenu.slider.unwrap()
    });
    assert_eq!(submenu.slider.unwrap().state, SliderState::UpperHover);
    submenu.on_a();
    assert_eq!(submenu.slider.unwrap().state, SliderState::UpperSelected);
    submenu.on_a();
    assert_eq!(submenu.slider.unwrap().state, SliderState::UpperHover);
}

#[test]
fn submenu_slider_on_b_selected() {
    let mut submenu = initialize_submenu(SubMenuType::Slider);
    submenu.slider = Some(StatefulSlider {
        state: SliderState::LowerSelected,
        ..submenu.slider.unwrap()
    });
    submenu.on_b();
    assert_eq!(submenu.slider.unwrap().state, SliderState::LowerHover);
    submenu.on_b();
    assert_eq!(submenu.slider.unwrap().state, SliderState::LowerHover);
    submenu.slider = Some(StatefulSlider {
        state: SliderState::UpperSelected,
        ..submenu.slider.unwrap()
    });
    submenu.on_b();
    assert_eq!(submenu.slider.unwrap().state, SliderState::UpperHover);
    submenu.on_b();
    assert_eq!(submenu.slider.unwrap().state, SliderState::UpperHover);
}

#[test]
fn submenu_single_on_up() {
    let mut submenu = initialize_submenu(SubMenuType::ToggleSingle);
    let mut state = TableState::default();
    state.select(Some(TableSelection::Cell { row: 0, col: 0 }));
    assert_eq!(submenu.toggles.state, state);
    submenu.on_up();
    state.select(Some(TableSelection::Cell { row: 1, col: 0 }));
    assert_eq!(submenu.toggles.state, state);
    submenu.on_up();
    state.select(Some(TableSelection::Cell { row: 0, col: 0 }));
    assert_eq!(submenu.toggles.state, state);

    submenu.toggles.select(0, 2);
    state.select(Some(TableSelection::Cell { row: 0, col: 2 }));
    assert_eq!(submenu.toggles.state, state);
    submenu.on_up();
    assert_eq!(submenu.toggles.state, state);
}

#[test]
fn submenu_multiple_on_up() {
    let mut submenu = initialize_submenu(SubMenuType::ToggleMultiple);
    let mut state = TableState::default();
    state.select(Some(TableSelection::Cell { row: 0, col: 0 }));
    assert_eq!(submenu.toggles.state, state);
    submenu.on_up();
    state.select(Some(TableSelection::Cell { row: 1, col: 0 }));
    assert_eq!(submenu.toggles.state, state);
    submenu.on_up();
    state.select(Some(TableSelection::Cell { row: 0, col: 0 }));
    assert_eq!(submenu.toggles.state, state);

    submenu.toggles.select(0, 2);
    state.select(Some(TableSelection::Cell { row: 0, col: 2 }));
    assert_eq!(submenu.toggles.state, state);
    submenu.on_up();
    assert_eq!(submenu.toggles.state, state);
}
#[test]
fn submenu_single_on_down() {
    let mut submenu = initialize_submenu(SubMenuType::ToggleSingle);
    let mut state = TableState::default();
    state.select(Some(TableSelection::Cell { row: 0, col: 0 }));
    assert_eq!(submenu.toggles.state, state);
    submenu.on_down();
    state.select(Some(TableSelection::Cell { row: 1, col: 0 }));
    assert_eq!(submenu.toggles.state, state);
    submenu.on_down();
    state.select(Some(TableSelection::Cell { row: 0, col: 0 }));
    assert_eq!(submenu.toggles.state, state);

    submenu.toggles.select(0, 2);
    state.select(Some(TableSelection::Cell { row: 0, col: 2 }));
    assert_eq!(submenu.toggles.state, state);
    submenu.on_down();
    assert_eq!(submenu.toggles.state, state);
}

#[test]
fn submenu_multiple_on_down() {
    let mut submenu = initialize_submenu(SubMenuType::ToggleMultiple);
    let mut state = TableState::default();
    state.select(Some(TableSelection::Cell { row: 0, col: 0 }));
    assert_eq!(submenu.toggles.state, state);
    submenu.on_down();
    state.select(Some(TableSelection::Cell { row: 1, col: 0 }));
    assert_eq!(submenu.toggles.state, state);
    submenu.on_down();
    state.select(Some(TableSelection::Cell { row: 0, col: 0 }));
    assert_eq!(submenu.toggles.state, state);

    submenu.toggles.select(0, 2);
    state.select(Some(TableSelection::Cell { row: 0, col: 2 }));
    assert_eq!(submenu.toggles.state, state);
    submenu.on_down();
    assert_eq!(submenu.toggles.state, state);
}

#[test]
fn submenu_single_on_left() {
    let mut submenu = initialize_submenu(SubMenuType::ToggleSingle);
    let mut state = TableState::default();
    state.select(Some(TableSelection::Cell { row: 0, col: 0 }));
    assert_eq!(submenu.toggles.state, state);
    assert_eq!(submenu.toggles.get_selected(), Some(&mut make_toggle(1)));
    submenu.on_left();
    state.select(Some(TableSelection::Cell { row: 0, col: 2 }));
    assert_eq!(submenu.toggles.state, state);
    assert_eq!(submenu.toggles.get_selected(), Some(&mut make_toggle(0)));
    submenu.on_left();
    state.select(Some(TableSelection::Cell { row: 0, col: 1 }));
    assert_eq!(submenu.toggles.state, state);
    assert_eq!(submenu.toggles.get_selected(), Some(&mut make_toggle(0)));

    submenu.toggles.select(1, 0);
    submenu.on_left();
    state.select(Some(TableSelection::Cell { row: 1, col: 0 }));
    assert_eq!(submenu.toggles.state, state);
    assert_eq!(submenu.toggles.get_selected(), Some(&mut make_toggle(0)));
    submenu.on_left();
    assert_eq!(submenu.toggles.state, state);
    assert_eq!(submenu.toggles.get_selected(), Some(&mut make_toggle(0)));
}

#[test]
fn submenu_multiple_on_left() {
    let mut submenu = initialize_submenu(SubMenuType::ToggleMultiple);
    let mut state = TableState::default();
    state.select(Some(TableSelection::Cell { row: 0, col: 0 }));
    assert_eq!(submenu.toggles.state, state);
    assert_eq!(submenu.toggles.get_selected(), Some(&mut make_toggle(0)));
    submenu.on_left();
    state.select(Some(TableSelection::Cell { row: 0, col: 2 }));
    assert_eq!(submenu.toggles.state, state);
    assert_eq!(submenu.toggles.get_selected(), Some(&mut make_toggle(2)));
    submenu.on_left();
    state.select(Some(TableSelection::Cell { row: 0, col: 1 }));
    assert_eq!(submenu.toggles.state, state);
    assert_eq!(submenu.toggles.get_selected(), Some(&mut make_toggle(1)));

    submenu.toggles.select(1, 0);
    submenu.on_left();
    state.select(Some(TableSelection::Cell { row: 1, col: 0 }));
    assert_eq!(submenu.toggles.state, state);
    assert_eq!(submenu.toggles.get_selected(), Some(&mut make_toggle(3)));
    submenu.on_left();
    assert_eq!(submenu.toggles.state, state);
    assert_eq!(submenu.toggles.get_selected(), Some(&mut make_toggle(3)));
}

#[test]
fn submenu_slider_on_left() {
    let mut submenu = initialize_submenu(SubMenuType::Slider);
    let mut state = SliderState::LowerHover;
    assert_eq!(submenu.slider.unwrap().state, state);
    state = SliderState::UpperHover;
    submenu.on_left();
    assert_eq!(submenu.slider.unwrap().state, state);

    submenu.slider = Some(StatefulSlider {
        state: SliderState::LowerSelected,
        lower: 1,
        ..submenu.slider.unwrap()
    });
    state = SliderState::LowerSelected;
    submenu.on_left();
    assert_eq!(submenu.slider.unwrap().state, state);
    assert_eq!(submenu.slider.unwrap().lower, 0);
    assert_eq!(submenu.slider.unwrap().upper, 150);
    submenu.on_left();
    assert_eq!(submenu.slider.unwrap().state, state);
    assert_eq!(submenu.slider.unwrap().lower, 0);
    assert_eq!(submenu.slider.unwrap().upper, 150);

    submenu.slider = Some(StatefulSlider {
        state: SliderState::UpperSelected,
        lower: 99,
        upper: 100,
        ..submenu.slider.unwrap()
    });
    state = SliderState::UpperSelected;
    submenu.on_left();
    assert_eq!(submenu.slider.unwrap().state, state);
    assert_eq!(submenu.slider.unwrap().lower, 99);
    assert_eq!(submenu.slider.unwrap().upper, 99);
    submenu.on_left();
    assert_eq!(submenu.slider.unwrap().state, state);
    assert_eq!(submenu.slider.unwrap().lower, 99);
    assert_eq!(submenu.slider.unwrap().upper, 99);
}

#[test]
fn submenu_single_on_right() {
    let mut submenu = initialize_submenu(SubMenuType::ToggleSingle);
    let mut state = TableState::default();
    state.select(Some(TableSelection::Cell { row: 0, col: 0 }));
    assert_eq!(submenu.toggles.state, state);
    assert_eq!(submenu.toggles.get_selected(), Some(&mut make_toggle(1)));
    submenu.on_right();
    state.select(Some(TableSelection::Cell { row: 0, col: 1 }));
    assert_eq!(submenu.toggles.state, state);
    assert_eq!(submenu.toggles.get_selected(), Some(&mut make_toggle(0)));
    submenu.on_right();
    state.select(Some(TableSelection::Cell { row: 0, col: 2 }));
    assert_eq!(submenu.toggles.state, state);
    assert_eq!(submenu.toggles.get_selected(), Some(&mut make_toggle(0)));

    submenu.toggles.select(1, 0);
    submenu.on_right();
    state.select(Some(TableSelection::Cell { row: 1, col: 0 }));
    assert_eq!(submenu.toggles.state, state);
    assert_eq!(submenu.toggles.get_selected(), Some(&mut make_toggle(0)));
    submenu.on_right();
    assert_eq!(submenu.toggles.state, state);
    assert_eq!(submenu.toggles.get_selected(), Some(&mut make_toggle(0)));
}

#[test]
fn submenu_multiple_on_right() {
    let mut submenu = initialize_submenu(SubMenuType::ToggleMultiple);
    let mut state = TableState::default();
    state.select(Some(TableSelection::Cell { row: 0, col: 0 }));
    assert_eq!(submenu.toggles.state, state);
    assert_eq!(submenu.toggles.get_selected(), Some(&mut make_toggle(0)));
    submenu.on_right();
    state.select(Some(TableSelection::Cell { row: 0, col: 1 }));
    assert_eq!(submenu.toggles.state, state);
    assert_eq!(submenu.toggles.get_selected(), Some(&mut make_toggle(1)));
    submenu.on_right();
    state.select(Some(TableSelection::Cell { row: 0, col: 2 }));
    assert_eq!(submenu.toggles.state, state);
    assert_eq!(submenu.toggles.get_selected(), Some(&mut make_toggle(2)));

    submenu.toggles.select(1, 0);
    submenu.on_right();
    state.select(Some(TableSelection::Cell { row: 1, col: 0 }));
    assert_eq!(submenu.toggles.state, state);
    assert_eq!(submenu.toggles.get_selected(), Some(&mut make_toggle(3)));
    submenu.on_right();
    assert_eq!(submenu.toggles.state, state);
    assert_eq!(submenu.toggles.get_selected(), Some(&mut make_toggle(3)));
}

#[test]
fn submenu_slider_on_right() {
    let mut submenu = initialize_submenu(SubMenuType::Slider);
    let mut state = SliderState::LowerHover;
    assert_eq!(submenu.slider.unwrap().state, state);
    state = SliderState::UpperHover;
    submenu.on_right();
    assert_eq!(submenu.slider.unwrap().state, state);

    submenu.slider = Some(StatefulSlider {
        state: SliderState::LowerSelected,
        lower: 10,
        upper: 11,
        ..submenu.slider.unwrap()
    });
    state = SliderState::LowerSelected;
    submenu.on_right();
    assert_eq!(submenu.slider.unwrap().state, state);
    assert_eq!(submenu.slider.unwrap().lower, 11);
    assert_eq!(submenu.slider.unwrap().upper, 11);
    submenu.on_right();
    assert_eq!(submenu.slider.unwrap().state, state);
    assert_eq!(submenu.slider.unwrap().lower, 11);
    assert_eq!(submenu.slider.unwrap().upper, 11);

    submenu.slider = Some(StatefulSlider {
        state: SliderState::UpperSelected,
        lower: 100,
        upper: 149,
        ..submenu.slider.unwrap()
    });
    state = SliderState::UpperSelected;
    submenu.on_right();
    assert_eq!(submenu.slider.unwrap().state, state);
    assert_eq!(submenu.slider.unwrap().lower, 100);
    assert_eq!(submenu.slider.unwrap().upper, 150);
    submenu.on_right();
    assert_eq!(submenu.slider.unwrap().state, state);
    assert_eq!(submenu.slider.unwrap().lower, 100);
    assert_eq!(submenu.slider.unwrap().upper, 150);
}
