use ratatui::widgets::*;
use serde::{Serialize, Serializer};

/// Allows a snake-filled table of arbitrary size
/// The final row does not need to be filled
/// [ a , b , c , d ]
/// [ e, f, g, h, i ]
/// [ j, k          ]

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StatefulTable<T: Clone + Serialize> {
    pub state: TableState,
    pub items: Vec<Vec<Option<T>>>,
    pub rows: usize,
    pub cols: usize,
}

// Size-related functions
impl<T: Clone + Serialize> StatefulTable<T> {
    pub fn len(&self) -> usize {
        self.items
            .iter()
            .map(|row| {
                row.iter()
                    .map(|item| if item.is_some() { 1 } else { 0 })
                    .sum::<usize>()
            })
            .sum()
    }
    pub fn full_len(&self) -> usize {
        self.rows * self.cols
    }
    pub fn as_vec(&self) -> Vec<T> {
        let mut v = Vec::with_capacity(self.len());
        for row in self.items.iter() {
            for item in row.iter() {
                if let Some(i) = item {
                    v.push(i.clone());
                }
            }
        }
        v
    }
}

// Initalization Functions
impl<T: Clone + Serialize> StatefulTable<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            state: TableState::default().with_selected(Some(TableSelection::default())),
            items: vec![vec![None; cols]; rows],
            rows: rows,
            cols: cols,
        }
    }
    pub fn with_items(rows: usize, cols: usize, v: Vec<T>) -> Self {
        let mut table: Self = Self::new(rows, cols);
        if v.len() > rows * cols {
            panic!(
                "Cannot create StatefulTable; too many items for size {}x{}: {}",
                rows,
                cols,
                v.len()
            );
        } else {
            for (i, item) in v.iter().enumerate() {
                table.items[i.div_euclid(cols)][i.rem_euclid(cols)] = Some(item.clone());
            }
            table
        }
    }
}

// State Functions
impl<T: Clone + Serialize> StatefulTable<T> {
    pub fn select(&mut self, row: usize, col: usize) {
        assert!(col < self.cols);
        assert!(row < self.rows);
        self.state.select(Some(TableSelection::Cell { row, col }));
    }

    pub fn get_selected(&mut self) -> Option<&mut T> {
        self.items[self.state.selected_row().unwrap()][self.state.selected_col().unwrap()].as_mut()
    }

    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        if row >= self.rows || column >= self.cols {
            None
        } else {
            self.items[row][column].as_ref()
        }
    }

    pub fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
        if row >= self.rows || column >= self.cols {
            None
        } else {
            self.items[row][column].as_mut()
        }
    }

    pub fn get_by_idx(&self, idx: usize) -> Option<&T> {
        let row = idx.div_euclid(self.cols);
        let col = idx.rem_euclid(self.cols);
        self.get(row, col)
    }
    pub fn get_by_idx_mut(&mut self, idx: usize) -> Option<&mut T> {
        let row = idx.div_euclid(self.cols);
        let col = idx.rem_euclid(self.cols);
        self.get_mut(row, col)
    }

    pub fn next_row(&mut self) {
        let next_row = match self.state.selected_row() {
            Some(row) => {
                if row == self.items.len() - 1 {
                    0
                } else {
                    row + 1
                }
            }

            None => 0,
        };
        self.state.select_row(Some(next_row));
    }

    pub fn next_row_checked(&mut self) {
        self.next_row();
        if self.get_selected().is_none() {
            self.next_row_checked();
        }
    }

    pub fn prev_row(&mut self) {
        let prev_row = match self.state.selected_row() {
            Some(row) => {
                if row == 0 {
                    self.items.len() - 1
                } else {
                    row - 1
                }
            }
            None => 0,
        };

        self.state.select_row(Some(prev_row));
    }

    pub fn prev_row_checked(&mut self) {
        self.prev_row();
        if self.get_selected().is_none() {
            self.prev_row_checked();
        }
    }

    pub fn next_col(&mut self) {
        // Assumes that all rows are the same width
        let next_col = match self.state.selected_col() {
            Some(col) => {
                if col == self.items[0].len() - 1 {
                    0
                } else {
                    col + 1
                }
            }
            None => 0,
        };
        self.state.select_col(Some(next_col));
    }

    pub fn next_col_checked(&mut self) {
        self.next_col();
        if self.get_selected().is_none() {
            self.next_col_checked();
        }
    }

    pub fn prev_col(&mut self) {
        let prev_col = match self.state.selected_col() {
            Some(col) => {
                if col == 0 {
                    self.items[0].len() - 1
                } else {
                    col - 1
                }
            }
            None => 0,
        };
        self.state.select_col(Some(prev_col));
    }

    pub fn prev_col_checked(&mut self) {
        self.prev_col();
        self.carriage_return();
    }

    /// If the selected cell is None, move selection to the left until you get Some.
    /// No-op if the selected cell is Some.
    /// For example, a 2x3 table with 4 elements would shift the selection from 1,2 to 1,0
    ///
    /// [ a ,  b ,  c ]
    /// [ d ,  e , [ ]]
    ///
    ///        |
    ///        V
    ///
    /// [ a ,  b ,  c ]
    /// [[d],    ,    ]
    pub fn carriage_return(&mut self) {
        assert!(
            self.items[self.state.selected_row().unwrap()]
                .iter()
                .any(|x| x.is_some()),
            "Carriage return called on an empty row!"
        );
        if self.get_selected().is_none() {
            self.prev_col();
            self.carriage_return();
        }
    }
}

impl<T: Clone + Serialize> Serialize for StatefulTable<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let flat: Vec<T> = self.as_vec();
        flat.serialize(serializer)
    }
}

// Implement .iter() for StatefulTable
pub struct StatefulTableIterator<'a, T: Clone + Serialize> {
    stateful_table: &'a StatefulTable<T>,
    index: usize,
}

impl<'a, T: Clone + Serialize> Iterator for StatefulTableIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.stateful_table.get_by_idx(self.index - 1)
    }
}

impl<T: Clone + Serialize> StatefulTable<T> {
    pub fn iter(&self) -> StatefulTableIterator<T> {
        StatefulTableIterator {
            stateful_table: self,
            index: 0,
        }
    }
}

pub struct StatefulTableIteratorMut<'a, T: Clone + Serialize> {
    inner: std::iter::Flatten<std::slice::IterMut<'a, Vec<Option<T>>>>,
}

impl<'a, T: Clone + Serialize> Iterator for StatefulTableIteratorMut<'a, T> {
    type Item = &'a mut Option<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, T: Clone + Serialize + 'a> StatefulTable<T> {
    pub fn iter_mut(&'a mut self) -> StatefulTableIteratorMut<T> {
        StatefulTableIteratorMut {
            inner: self.items.iter_mut().flatten(),
        }
    }
}
