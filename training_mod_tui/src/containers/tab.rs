use serde::ser::{SerializeMap, Serializer};
use serde::Serialize;

use crate::{InputControl, StatefulTable, SubMenu};

#[derive(Clone)]
pub struct Tab<'a> {
    pub title: &'a str,
    pub id: &'a str,
    pub submenus: StatefulTable<SubMenu<'a>>,
}

impl<'a> Tab<'a> {
    pub fn len(&self) -> usize {
        self.submenus.len()
    }
}

impl<'a> Serialize for Tab<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.submenus.len()))?;
        for submenu in self.submenus.as_vec().iter() {
            map.serialize_entry(&submenu.title, &submenu)?;
        }
        map.end()
    }
}

impl<'a> InputControl for Tab<'a> {
    fn on_a(&mut self) {}
    fn on_b(&mut self) {}
    fn on_x(&mut self) {}
    fn on_y(&mut self) {}
    fn on_up(&mut self) {
        self.submenus.prev_row_checked()
    }
    fn on_down(&mut self) {
        self.submenus.next_row_checked()
    }
    fn on_left(&mut self) {
        self.submenus.prev_col_checked()
    }
    fn on_right(&mut self) {
        self.submenus.next_col_checked()
    }
    fn on_start(&mut self) {}
    fn on_l(&mut self) {}
    fn on_r(&mut self) {}
    fn on_zl(&mut self) {}
    fn on_zr(&mut self) {}
}
