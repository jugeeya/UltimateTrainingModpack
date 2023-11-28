use ratatui::widgets::{TableSelection, TableState};
use training_mod_tui_2::StatefulTable;

fn initialize_table(row: usize, col: usize) -> StatefulTable<u8> {
    let mut s = StatefulTable::with_items(2, 3, vec![0, 1, 2, 3, 4]);
    s.select(row, col);
    s
}

fn tablestate_with(row: usize, col: usize) -> TableState {
    TableState::default().with_selected(Some(TableSelection::Cell { row, col }))
}

#[test]
fn stateful_table_next_col_full() {
    let mut t = initialize_table(0, 0);
    assert_eq!(t.get_selected(), Some(&mut 0));
    t.next_col();
    assert_eq!(t.get_selected(), Some(&mut 1));
    t.next_col();
    assert_eq!(t.get_selected(), Some(&mut 2));
    t.next_col();
    assert_eq!(t.get_selected(), Some(&mut 0));
}

#[test]
fn stateful_table_next_col_checked_full() {
    let mut t = initialize_table(0, 0);
    assert_eq!(t.get_selected(), Some(&mut 0));
    t.next_col_checked();
    assert_eq!(t.get_selected(), Some(&mut 1));
    t.next_col_checked();
    assert_eq!(t.get_selected(), Some(&mut 2));
    t.next_col_checked();
    assert_eq!(t.get_selected(), Some(&mut 0));
}

#[test]
fn stateful_table_prev_col_full() {
    let mut t = initialize_table(0, 0);
    assert_eq!(t.get_selected(), Some(&mut 0));
    t.prev_col();
    assert_eq!(t.get_selected(), Some(&mut 2));
    t.prev_col();
    assert_eq!(t.get_selected(), Some(&mut 1));
    t.prev_col();
    assert_eq!(t.get_selected(), Some(&mut 0));
}

#[test]
fn stateful_table_prev_col_checked_full() {
    let mut t = initialize_table(0, 0);
    assert_eq!(t.get_selected(), Some(&mut 0));
    t.prev_col_checked();
    assert_eq!(t.get_selected(), Some(&mut 2));
    t.prev_col_checked();
    assert_eq!(t.get_selected(), Some(&mut 1));
    t.prev_col_checked();
    assert_eq!(t.get_selected(), Some(&mut 0));
}

#[test]
fn stateful_table_next_col_short() {
    let mut t = initialize_table(1, 0);
    assert_eq!(t.get_selected(), Some(&mut 3));
    t.next_col();
    assert_eq!(t.get_selected(), Some(&mut 4));
    t.next_col();
    assert_eq!(t.get_selected(), None);
    t.next_col();
    assert_eq!(t.get_selected(), Some(&mut 3));
}

#[test]
fn stateful_table_next_col_checked_short() {
    let mut t = initialize_table(1, 0);
    assert_eq!(t.get_selected(), Some(&mut 3));
    t.next_col_checked();
    assert_eq!(t.get_selected(), Some(&mut 4));
    t.next_col_checked();
    assert_eq!(t.get_selected(), Some(&mut 3));
}

#[test]
fn stateful_table_prev_col_short() {
    let mut t = initialize_table(1, 0);
    assert_eq!(t.get_selected(), Some(&mut 3));
    t.prev_col();
    assert_eq!(t.get_selected(), None);
    t.prev_col();
    assert_eq!(t.get_selected(), Some(&mut 4));
    t.prev_col();
    assert_eq!(t.get_selected(), Some(&mut 3));
}

#[test]
fn stateful_table_carriage_return_none() {
    let mut t = initialize_table(1, 2);
    t.carriage_return();
    assert_eq!(t.state, tablestate_with(1, 1));
}

#[test]
fn stateful_table_carriage_return_some() {
    let mut t = initialize_table(1, 1);
    t.carriage_return();
    assert_eq!(t.state, tablestate_with(1, 1));
}

#[test]
fn stateful_table_table_with_items() {
    let items: Vec<u8> = vec![0, 1, 2, 3, 4];
    let t: StatefulTable<u8> = StatefulTable::with_items(2, 3, items);
    let u = initialize_table(0, 0);
    assert_eq!(t, u);
}

#[test]
fn stateful_table_get_selected() {
    let mut t = initialize_table(1, 1);
    assert_eq!(t.get_selected(), Some(&mut 4));
}

#[test]
fn stateful_table_get() {
    let t = initialize_table(1, 1);
    assert_eq!(t.get(0, 0), Some(&0));
    assert_eq!(t.get(0, 1), Some(&1));
    assert_eq!(t.get(0, 2), Some(&2));
    assert_eq!(t.get(1, 0), Some(&3));
    assert_eq!(t.get(1, 1), Some(&4));
    assert_eq!(t.get(1, 2), None);
    assert_eq!(t.get(10, 0), None);
    assert_eq!(t.get(0, 10), None);
}

#[test]
fn stateful_table_get_by_idx() {
    let t = initialize_table(1, 1);
    assert_eq!(t.get_by_idx(0), Some(&0));
    assert_eq!(t.get_by_idx(1), Some(&1));
    assert_eq!(t.get_by_idx(2), Some(&2));
    assert_eq!(t.get_by_idx(3), Some(&3));
    assert_eq!(t.get_by_idx(4), Some(&4));
    assert_eq!(t.get_by_idx(5), None);
    assert_eq!(t.get_by_idx(50), None);
}

#[test]
fn stateful_table_len() {
    let t = initialize_table(1, 1);
    assert_eq!(t.len(), 5);
}

#[test]
fn stateful_table_full_len() {
    let t = initialize_table(0, 0);
    assert_eq!(t.full_len(), 6);
}

#[test]
fn stateful_table_serialize() {
    let t = initialize_table(1, 1);
    let t_ser = serde_json::to_string(&t).unwrap();
    assert_eq!(&t_ser, "[0,1,2,3,4]");
}

#[test]
fn stateful_table_new() {
    let t: StatefulTable<u8> = StatefulTable::new(2, 3);
    let u: StatefulTable<u8> = StatefulTable::with_items(2, 3, vec![]);
    let v: StatefulTable<u8> = StatefulTable {
        state: tablestate_with(0, 0),
        items: vec![vec![None; 3]; 2],
        rows: 2,
        cols: 3,
    };
    assert_eq!(t, u);
    assert_eq!(t, v);
}

#[test]
fn stateful_table_with_items() {
    let t: StatefulTable<u8> = StatefulTable::with_items(2, 3, vec![1, 2]);
    let u: StatefulTable<u8> = StatefulTable {
        state: tablestate_with(0, 0),
        items: vec![vec![Some(1), Some(2), None], vec![None; 3]],
        rows: 2,
        cols: 3,
    };
    assert_eq!(t, u);
}

#[test]
fn stateful_table_select() {
    let mut t = initialize_table(0, 0);
    assert_eq!(t.get_selected(), Some(&mut 0));
    t.select(0, 1);
    assert_eq!(t.get_selected(), Some(&mut 1));
    t.select(0, 2);
    assert_eq!(t.get_selected(), Some(&mut 2));
    t.select(1, 0);
    assert_eq!(t.get_selected(), Some(&mut 3));
    t.select(1, 1);
    assert_eq!(t.get_selected(), Some(&mut 4));
    t.select(1, 2);
    assert_eq!(t.get_selected(), None);
}

#[test]
fn stateful_table_get_mut() {
    let mut t = initialize_table(1, 1);
    assert_eq!(t.get_mut(0, 0), Some(&mut 0));
    assert_eq!(t.get_mut(0, 1), Some(&mut 1));
    assert_eq!(t.get_mut(0, 2), Some(&mut 2));
    assert_eq!(t.get_mut(1, 0), Some(&mut 3));
    assert_eq!(t.get_mut(1, 1), Some(&mut 4));
    assert_eq!(t.get_mut(1, 2), None);
    assert_eq!(t.get_mut(10, 0), None);
    assert_eq!(t.get_mut(0, 10), None);
}

#[test]
fn stateful_table_get_by_idx_mut() {
    let mut t = initialize_table(1, 1);
    assert_eq!(t.get_by_idx_mut(0), Some(&mut 0));
    assert_eq!(t.get_by_idx_mut(1), Some(&mut 1));
    assert_eq!(t.get_by_idx_mut(2), Some(&mut 2));
    assert_eq!(t.get_by_idx_mut(3), Some(&mut 3));
    assert_eq!(t.get_by_idx_mut(4), Some(&mut 4));
    assert_eq!(t.get_by_idx_mut(5), None);
    assert_eq!(t.get_by_idx_mut(50), None);
}

#[test]
fn stateful_table_next_row_full() {
    let mut t = initialize_table(0, 0);
    assert_eq!(t.get_selected(), Some(&mut 0));
    t.next_row();
    assert_eq!(t.get_selected(), Some(&mut 3));
    t.next_row();
    assert_eq!(t.get_selected(), Some(&mut 0));
}

#[test]
fn stateful_table_next_row_short() {
    let mut t = initialize_table(0, 2);
    assert_eq!(t.get_selected(), Some(&mut 2));
    t.next_row();
    assert_eq!(t.get_selected(), None);
    t.next_row();
    assert_eq!(t.get_selected(), Some(&mut 2));
}

#[test]
fn stateful_table_next_row_checked_full() {
    let mut t = initialize_table(0, 0);
    assert_eq!(t.get_selected(), Some(&mut 0));
    t.next_row_checked();
    assert_eq!(t.get_selected(), Some(&mut 3));
    t.next_row_checked();
    assert_eq!(t.get_selected(), Some(&mut 0));
}

#[test]
fn stateful_table_next_row_checked_short() {
    let mut t = initialize_table(0, 2);
    assert_eq!(t.get_selected(), Some(&mut 2));
    t.next_row_checked();
    assert_eq!(t.get_selected(), Some(&mut 2));
    t.next_row_checked();
    assert_eq!(t.get_selected(), Some(&mut 2));
}

#[test]
fn stateful_table_prev_row_full() {
    let mut t = initialize_table(0, 0);
    assert_eq!(t.get_selected(), Some(&mut 0));
    t.prev_row();
    assert_eq!(t.get_selected(), Some(&mut 3));
    t.prev_row();
    assert_eq!(t.get_selected(), Some(&mut 0));
}

#[test]
fn stateful_table_prev_row_short() {
    let mut t = initialize_table(0, 2);
    assert_eq!(t.get_selected(), Some(&mut 2));
    t.prev_row();
    assert_eq!(t.get_selected(), None);
    t.prev_row();
    assert_eq!(t.get_selected(), Some(&mut 2));
}

#[test]
fn stateful_table_prev_row_checked_full() {
    let mut t = initialize_table(0, 0);
    assert_eq!(t.get_selected(), Some(&mut 0));
    t.prev_row_checked();
    assert_eq!(t.get_selected(), Some(&mut 3));
    t.prev_row_checked();
    assert_eq!(t.get_selected(), Some(&mut 0));
}

#[test]
fn stateful_table_prev_row_checked_short() {
    let mut t = initialize_table(0, 2);
    assert_eq!(t.get_selected(), Some(&mut 2));
    t.prev_row_checked();
    assert_eq!(t.get_selected(), Some(&mut 2));
    t.prev_row_checked();
    assert_eq!(t.get_selected(), Some(&mut 2));
}

#[test]
fn stateful_table_iter() {
    let t = initialize_table(0, 0);
    let mut t_iter = t.iter();
    assert_eq!(t_iter.next(), Some(&0));
    assert_eq!(t_iter.next(), Some(&1));
    assert_eq!(t_iter.next(), Some(&2));
    assert_eq!(t_iter.next(), Some(&3));
    assert_eq!(t_iter.next(), Some(&4));
    assert_eq!(t_iter.next(), None);
    assert_eq!(t_iter.next(), None);
}

#[test]
fn stateful_table_iter_mut() {
    let mut t = initialize_table(0, 0);
    for item in t.iter_mut().filter(|item| item.is_some()) {
        *item = Some(item.unwrap() + 10);
    }
    let mut t_iter = t.iter();
    assert_eq!(t_iter.next(), Some(&10));
    assert_eq!(t_iter.next(), Some(&11));
    assert_eq!(t_iter.next(), Some(&12));
    assert_eq!(t_iter.next(), Some(&13));
    assert_eq!(t_iter.next(), Some(&14));
    assert_eq!(t_iter.next(), None);
    assert_eq!(t_iter.next(), None);
}
