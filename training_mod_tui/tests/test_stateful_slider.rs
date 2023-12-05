use training_mod_tui_2::{SliderState, StatefulSlider};

fn initialize_slider(state: SliderState) -> StatefulSlider {
    StatefulSlider {
        state: state,
        lower: 0,
        upper: 150,
        min: 0,
        max: 150,
        incr_amount_slow: 1,
        incr_amount_fast: 10,
    }
}

#[test]
fn stateful_slider_new() {
    let s = initialize_slider(SliderState::LowerHover);
    assert_eq!(s, StatefulSlider::new());
}

#[test]
fn stateful_slider_increment_selected_slow() {
    let mut s = initialize_slider(SliderState::LowerHover);
    s.upper = 50;

    // Check LowerHover: no increment
    s.increment_selected_slow();
    assert_eq!(s.lower, 0);

    // Check LowerSelected: lower increments
    s.state = SliderState::LowerSelected;
    s.increment_selected_slow();
    assert_eq!(s.lower, 1);

    // Check LowerSelected: lower can't go above upper
    s.lower = s.upper;
    s.increment_selected_slow();
    assert_eq!(s.lower, s.upper);

    // Check UpperHover: no increment
    s.lower = 5;
    s.upper = 50;
    s.state = SliderState::UpperHover;
    s.increment_selected_slow();
    assert_eq!(s.lower, 5);
    assert_eq!(s.upper, 50);

    // Check UpperSelected: upper increments
    s.state = SliderState::UpperSelected;
    s.increment_selected_slow();
    assert_eq!(s.lower, 5);
    assert_eq!(s.upper, 51);

    // Check UpperSelected: upper can't go above max
    s.upper = s.max;
    s.increment_selected_slow();
    assert_eq!(s.lower, 5);
    assert_eq!(s.upper, s.max);
}

#[test]
fn stateful_slider_increment_selected_fast() {
    let mut s = initialize_slider(SliderState::LowerHover);
    s.upper = 50;

    // Check LowerHover: no increment
    s.increment_selected_fast();
    assert_eq!(s.lower, 0);

    // Check LowerSelected: lower increments
    s.state = SliderState::LowerSelected;
    s.increment_selected_fast();
    assert_eq!(s.lower, 10);

    // Check LowerSelected: lower can't go above upper
    s.lower = s.upper;
    s.increment_selected_fast();
    assert_eq!(s.lower, s.upper);

    // Check UpperHover: no increment
    s.lower = 5;
    s.upper = 50;
    s.state = SliderState::UpperHover;
    s.increment_selected_fast();
    assert_eq!(s.lower, 5);
    assert_eq!(s.upper, 50);

    // Check UpperSelected: upper increments
    s.state = SliderState::UpperSelected;
    s.increment_selected_fast();
    assert_eq!(s.lower, 5);
    assert_eq!(s.upper, 60);

    // Check UpperSelected: upper can't go above max
    s.upper = s.max;
    s.increment_selected_fast();
    assert_eq!(s.lower, 5);
    assert_eq!(s.upper, s.max);
}

#[test]
fn stateful_slider_decrement_selected_slow() {
    let mut s = initialize_slider(SliderState::LowerHover);
    s.min = 4;
    s.lower = 5;
    s.upper = 50;

    // Check LowerHover: no decrement
    s.decrement_selected_slow();
    assert_eq!(s.lower, 5);

    // Check LowerSelected: lower decrements
    s.state = SliderState::LowerSelected;
    s.decrement_selected_slow();
    assert_eq!(s.lower, 4);

    // Check LowerSelected: lower can't go below min
    s.lower = s.min;
    s.decrement_selected_slow();
    assert_eq!(s.lower, s.min);

    // Check LowerSelected: lower can't go below 0
    s.min = 0;
    s.lower = 0;
    s.decrement_selected_slow();
    assert_eq!(s.lower, 0);

    // Check UpperHover: no decrement
    s.lower = 5;
    s.upper = 50;
    s.state = SliderState::UpperHover;
    s.decrement_selected_slow();
    assert_eq!(s.lower, 5);
    assert_eq!(s.upper, 50);

    // Check UpperSelected: upper decrements
    s.state = SliderState::UpperSelected;
    s.decrement_selected_slow();
    assert_eq!(s.lower, 5);
    assert_eq!(s.upper, 49);

    // Check UpperSelected: upper can't go below lower
    s.upper = s.lower;
    s.decrement_selected_slow();
    assert_eq!(s.lower, 5);
    assert_eq!(s.upper, s.lower);
}

#[test]
fn stateful_slider_decrement_selected_fast() {
    let mut s = initialize_slider(SliderState::LowerHover);
    s.min = 20;
    s.lower = 35;
    s.upper = 50;

    // Check LowerHover: no decrement
    s.decrement_selected_fast();
    assert_eq!(s.lower, 35);

    // Check LowerSelected: lower decrements
    s.state = SliderState::LowerSelected;
    s.decrement_selected_fast();
    assert_eq!(s.lower, 25);

    // Check LowerSelected: lower can't go below min
    s.lower = s.min;
    s.decrement_selected_fast();
    assert_eq!(s.lower, s.min);

    // Check LowerSelected: lower can't go below 0
    s.min = 0;
    s.lower = 0;
    s.decrement_selected_fast();
    assert_eq!(s.lower, 0);

    // Check UpperHover: no decrement
    s.lower = 5;
    s.upper = 50;
    s.state = SliderState::UpperHover;
    s.decrement_selected_fast();
    assert_eq!(s.lower, 5);
    assert_eq!(s.upper, 50);

    // Check UpperSelected: upper decrements
    s.state = SliderState::UpperSelected;
    s.decrement_selected_fast();
    assert_eq!(s.lower, 5);
    assert_eq!(s.upper, 40);

    // Check UpperSelected: upper can't go below lower
    s.upper = s.lower;
    s.decrement_selected_fast();
    assert_eq!(s.lower, 5);
    assert_eq!(s.upper, s.lower);
}

#[test]
fn stateful_slider_select_deselect() {
    let mut s = initialize_slider(SliderState::LowerHover);
    assert_eq!(s.state, SliderState::LowerHover);
    s.select_deselect();
    assert_eq!(s.state, SliderState::LowerSelected);
    s.select_deselect();
    assert_eq!(s.state, SliderState::LowerHover);
    s.state = SliderState::UpperHover;
    s.select_deselect();
    assert_eq!(s.state, SliderState::UpperSelected);
    s.select_deselect();
    assert_eq!(s.state, SliderState::UpperHover);
    s.state = SliderState::None;
    s.select_deselect();
    assert_eq!(s.state, SliderState::None);
}

#[test]
fn stateful_slider_deselect() {
    let mut s = initialize_slider(SliderState::LowerHover);
    s.deselect();
    assert_eq!(s.state, SliderState::LowerHover);

    s.state = SliderState::LowerSelected;
    s.deselect();
    assert_eq!(s.state, SliderState::LowerHover);

    s.state = SliderState::UpperHover;
    s.deselect();
    assert_eq!(s.state, SliderState::UpperHover);

    s.state = SliderState::UpperSelected;
    s.deselect();
    assert_eq!(s.state, SliderState::UpperHover);

    s.state = SliderState::None;
    s.deselect();
    assert_eq!(s.state, SliderState::None);
}

#[test]
fn stateful_slider_switch_hover() {
    let mut s = initialize_slider(SliderState::LowerHover);
    s.switch_hover();
    assert_eq!(s.state, SliderState::UpperHover);

    s.state = SliderState::LowerSelected;
    s.switch_hover();
    assert_eq!(s.state, SliderState::LowerSelected);

    s.state = SliderState::UpperHover;
    s.switch_hover();
    assert_eq!(s.state, SliderState::LowerHover);

    s.state = SliderState::UpperSelected;
    s.switch_hover();
    assert_eq!(s.state, SliderState::UpperSelected);

    s.state = SliderState::None;
    s.switch_hover();
    assert_eq!(s.state, SliderState::None);
}

#[test]
fn stateful_slider_is_handle_selected() {
    let mut s = initialize_slider(SliderState::LowerHover);
    assert_eq!(s.is_handle_selected(), false);

    s.state = SliderState::LowerSelected;
    assert_eq!(s.is_handle_selected(), true);

    s.state = SliderState::UpperHover;
    assert_eq!(s.is_handle_selected(), false);

    s.state = SliderState::UpperSelected;
    assert_eq!(s.is_handle_selected(), true);

    s.state = SliderState::None;
    assert_eq!(s.is_handle_selected(), false);
}

#[test]
fn stateful_slider_serialize() {
    let mut s = initialize_slider(SliderState::LowerHover);
    let s_json = serde_json::to_string(&s).unwrap();
    assert_eq!(&s_json, "[0,150]");
    s.lower = 25;
    s.upper = 75;
    let s_json = serde_json::to_string(&s).unwrap();
    assert_eq!(&s_json, "[25,75]");
}
