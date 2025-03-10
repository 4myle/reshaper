
// #![deny(clippy::pedantic)]
// #![deny(clippy::unwrap_used)]
// #![deny(clippy::expect_used)]
// #![deny(clippy::panic)]
// #![deny(unused_must_use)]

use std::rc::Rc;

#[derive(Default)]
pub struct Row
{
    data: String,
    part: Vec<Rc<str>>
}

impl Row
{
    pub fn new (data: String) -> Self {
        Self { 
            data, 
            part: Vec::new()
        }
    }

    pub fn add (&mut self, start: usize, end: usize) -> Option<()> {
        if self.data.is_empty() || start > end || end > self.data.len() {
            return None;
        }
        self.part.push(self.data[start..end].into());
        Some(())
    }

    pub fn get (&self, index: usize) -> &str { //TODO: cleaner with Option<&str>?
        if self.data.is_empty() || index > self.part.len() {
            return "";
        }
        &self.part[index]
    }

    // pub fn count (&self) -> usize {
    //     if self.data.is_empty() {
    //         return 0;
    //     }
    //     self.part[0].len()
    // }

    // pub fn is_empty (&self) -> bool {
    //     self.data.is_empty()
    // }

}
