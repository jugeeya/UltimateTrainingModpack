use serde::ser::Serializer;
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Toggle<'a> {
    pub title: &'a str,
    pub value: u8,
    pub max: u8,
}

impl<'a> Serialize for Toggle<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.value)
    }
}

impl<'a> Toggle<'a> {
    pub fn increment(&mut self) {
        if self.value == self.max {
            self.value = 0;
        } else {
            self.value += 1;
        }
    }

    pub fn decrement(&mut self) {
        if self.value == 0 {
            self.value = self.max;
        } else {
            self.value -= 1;
        }
    }
}
