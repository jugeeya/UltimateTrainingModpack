use tui::widgets::ListState;

pub struct MultiStatefulList<T> {
    pub lists: Vec<StatefulList<T>>,
    pub state: usize,
    pub total_len: usize
}

impl<T: Clone> MultiStatefulList<T> {
    pub fn selected_list_item(&mut self) -> &mut T {
        let (list_section, list_idx) = self.idx_to_list_idx(self.state);
        &mut self
            .lists[list_section]
            .items[list_idx]
    }

    pub fn idx_to_list_idx(&self, idx: usize) -> (usize, usize) {
        self.idx_to_list_idx_opt(idx).unwrap_or((0, 0))
    }

    pub fn idx_to_list_idx_opt(&self, idx: usize) -> Option<(usize, usize)> {
        for list_section in 0..self.lists.len() {
            let list_section_min_idx = (self.total_len as f32 / self.lists.len() as f32).ceil() as usize * list_section;
            let list_section_max_idx = std::cmp::min(
                (self.total_len as f32 / self.lists.len() as f32).ceil() as usize * (list_section + 1),
                self.total_len);
            if (list_section_min_idx..list_section_max_idx).contains(&idx) {
                return Some((list_section, idx - list_section_min_idx));
            }
        }
        
        None
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
            let list_section_min_idx = (items.len() as f32 / num_lists as f32).ceil() as usize * list_section;
            let list_section_max_idx = std::cmp::min(
                (items.len() as f32 / num_lists as f32).ceil() as usize * (list_section + 1),
                items.len());
            let mut state = ListState::default();
            if list_section == 0 {
                // Enforce state as first of list
                state.select(Some(0));
            }
            StatefulList {
                state,
                items: items[list_section_min_idx..list_section_max_idx].to_vec(),
            }
        }).collect();
        let total_len = items.len();
        MultiStatefulList {
            lists,
            total_len,
            state: 0
        }
    }

    pub fn next(&mut self) {
        let (list_section, _) = self.idx_to_list_idx(self.state);
        let (next_list_section, next_list_idx) = self.idx_to_list_idx(self.state+1);

        if list_section != next_list_section {
            self.lists[list_section].unselect();
        }
        let state= if self.state + 1 >= self.total_len {
            (0, 0)
        } else {
            (next_list_section, next_list_idx)
        };

        self.lists[state.0].state.select(Some(state.1));
        self.state = self.list_idx_to_idx(state);
    }

    pub fn previous(&mut self) {
        let (list_section, _) = self.idx_to_list_idx(self.state);
        let (last_list_section, last_list_idx) = (self.lists.len() - 1, self.lists[self.lists.len() - 1].items.len() - 1);

        self.lists[list_section].unselect();
        let state= if self.state == 0 {
            (last_list_section, last_list_idx)
        } else {
            let (prev_list_section, prev_list_idx) = self.idx_to_list_idx(self.state - 1);
            (prev_list_section, prev_list_idx)
        };

        self.lists[state.0].state.select(Some(state.1));
        self.state = self.list_idx_to_idx(state);
    }

    pub fn next_list(&mut self) {
        let (list_section, list_idx) = self.idx_to_list_idx(self.state);
        let next_list_section = (list_section + 1) % self.lists.len();
        let next_list_idx = if list_idx > self.lists[next_list_section].items.len() - 1 {
            self.lists[next_list_section].items.len() - 1
        } else {
            list_idx
        };

        if list_section != next_list_section {
            self.lists[list_section].unselect();
        }
        let state = (next_list_section, next_list_idx);

        self.lists[state.0].state.select(Some(state.1));
        self.state = self.list_idx_to_idx(state);
    }

    pub fn previous_list(&mut self) {
        let (list_section, list_idx) = self.idx_to_list_idx(self.state);
        let prev_list_section = if list_section == 0 {
            self.lists.len() - 1
        } else {
            list_section - 1
        };

        let prev_list_idx= if list_idx > self.lists[prev_list_section].items.len() - 1 {
            self.lists[prev_list_section].items.len() - 1
        } else {
            list_idx
        };

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
            state,
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