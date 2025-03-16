
// #![deny(clippy::pedantic)]
// #![deny(clippy::unwrap_used)]
// #![deny(clippy::expect_used)]
// #![deny(clippy::panic)]
// #![deny(unused_must_use)]

use std::rc::Rc;

#[derive(Default)]
pub struct Row
{
    text: String,
    part: Vec<Rc<str>>
}

impl Row
{
    pub fn new (text: &str, width: usize) -> Self {
        Self { 
            text: text.to_string(), 
            part: Vec::with_capacity(width)
        }
    }

    pub fn add (&mut self, start: usize, end: usize) -> Option<()> {
        if self.text.is_empty() || start > end || end > self.text.len() {
            return None;
        }
        self.part.push(self.text[start..end].into());
        Some(())
    }

    pub fn get (&self, index: usize) -> Option<&str> {
        if self.text.is_empty() || index > self.part.len() {
            return None;
        }
        Some(&self.part[index])
    }

}
