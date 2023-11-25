use ratatui::widgets::ListState;
use training_mod_tui_2::StatefulList;

fn initialize_list(selected: Option<usize>) -> StatefulList<u8> {
    StatefulList {
        state: initialize_state(selected),
        items: vec![10, 20, 30, 40],
    }
}

fn initialize_state(selected: Option<usize>) -> ListState {
    let mut state = ListState::default();
    state.select(selected);
    state
}

#[test]
fn stateful_list_test_new() {
    let l = initialize_list(None);
    assert_eq!(
        l,
        StatefulList {
            state: ListState::default(),
            items: vec![10, 20, 30, 40],
        }
    );
}

#[test]
fn stateful_list_with_items() {
    let l = initialize_list(Some(0));
    let m = StatefulList::<u8>::with_items(vec![10, 20, 30, 40]);
    assert_eq!(l, m);
}

#[test]
fn stateful_list_next() {
    let mut l = initialize_list(Some(0));
    let mut state = ListState::default();
    state.select(Some(0));
    assert_eq!(l.state, state);
    assert_eq!(l.get_selected(), Some(&mut 10));
    l.next();
    state.select(Some(1));
    assert_eq!(l.state, state);
    assert_eq!(l.get_selected(), Some(&mut 20));
    l.next();
    state.select(Some(2));
    assert_eq!(l.state, state);
    assert_eq!(l.get_selected(), Some(&mut 30));
    l.next();
    state.select(Some(3));
    assert_eq!(l.state, state);
    assert_eq!(l.get_selected(), Some(&mut 40));
    l.next();
    state.select(Some(0));
    assert_eq!(l.state, state);
    assert_eq!(l.get_selected(), Some(&mut 10));
}

#[test]
fn stateful_list_prev() {
    let mut l = initialize_list(Some(0));
    let mut state = ListState::default();
    state.select(Some(0));
    assert_eq!(l.state, state);
    assert_eq!(l.get_selected(), Some(&mut 10));
    l.previous();
    state.select(Some(3));
    assert_eq!(l.state, state);
    assert_eq!(l.get_selected(), Some(&mut 40));
    l.previous();
    state.select(Some(2));
    assert_eq!(l.state, state);
    assert_eq!(l.get_selected(), Some(&mut 30));
    l.previous();
    state.select(Some(1));
    assert_eq!(l.state, state);
    assert_eq!(l.get_selected(), Some(&mut 20));
    l.previous();
    state.select(Some(0));
    assert_eq!(l.state, state);
    assert_eq!(l.get_selected(), Some(&mut 10));
}

#[test]
fn stateful_list_unselect() {
    let mut l = initialize_list(Some(0));
    let state = ListState::default();
    l.unselect();
    assert_eq!(l.state, state);
    l.unselect();
    assert_eq!(l.state, state);
}

#[test]
fn stateful_list_get_selected() {
    let mut l = initialize_list(None);
    assert_eq!(l.get_selected(), None);
    l.state.select(Some(0));
    assert_eq!(l.get_selected(), Some(&mut 10));
    l.state.select(Some(1));
    assert_eq!(l.get_selected(), Some(&mut 20));
    l.state.select(Some(2));
    assert_eq!(l.get_selected(), Some(&mut 30));
    l.state.select(Some(3));
    assert_eq!(l.get_selected(), Some(&mut 40));
}

#[test]
fn stateful_list_get_before_selected() {
    let mut l = initialize_list(None);
    assert_eq!(l.get_before_selected(), None);
    l.state.select(Some(0));
    assert_eq!(l.get_before_selected(), Some(&mut 40));
    l.state.select(Some(1));
    assert_eq!(l.get_before_selected(), Some(&mut 10));
    l.state.select(Some(2));
    assert_eq!(l.get_before_selected(), Some(&mut 20));
    l.state.select(Some(3));
    assert_eq!(l.get_before_selected(), Some(&mut 30));
}

#[test]
fn stateful_list_get_after_selected() {
    let mut l = initialize_list(None);
    assert_eq!(l.get_after_selected(), None);
    l.state.select(Some(0));
    assert_eq!(l.get_after_selected(), Some(&mut 20));
    l.state.select(Some(1));
    assert_eq!(l.get_after_selected(), Some(&mut 30));
    l.state.select(Some(2));
    assert_eq!(l.get_after_selected(), Some(&mut 40));
    l.state.select(Some(3));
    assert_eq!(l.get_after_selected(), Some(&mut 10));
}

#[test]
fn stateful_list_serialize() {
    let l = initialize_list(Some(2));
    let l_json = serde_json::to_string(&l).unwrap();
    assert_eq!(&l_json, "[10,20,30,40]");
}

#[test]
fn stateful_list_iter() {
    let l = initialize_list(Some(2));
    let mut l_iter = l.iter();
    assert_eq!(l_iter.next(), Some(&10));
    assert_eq!(l_iter.next(), Some(&20));
    assert_eq!(l_iter.next(), Some(&30));
    assert_eq!(l_iter.next(), Some(&40));
    assert_eq!(l_iter.next(), None);
    assert_eq!(l_iter.next(), None);
}

#[test]
fn stateful_list_iter_mut() {
    let mut l = initialize_list(Some(2));
    let mut l_iter_mut = l.iter_mut();
    assert_eq!(l_iter_mut.next(), Some(&mut 10));
    assert_eq!(l_iter_mut.next(), Some(&mut 20));
    assert_eq!(l_iter_mut.next(), Some(&mut 30));
    assert_eq!(l_iter_mut.next(), Some(&mut 40));
    assert_eq!(l_iter_mut.next(), None);
    assert_eq!(l_iter_mut.next(), None);
}
