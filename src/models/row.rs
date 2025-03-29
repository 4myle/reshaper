
// #![deny(clippy::pedantic)]
// #![deny(clippy::unwrap_used)]
// #![deny(clippy::expect_used)]
// #![deny(clippy::panic)]
// #![deny(unused_must_use)]

use std::sync::Arc;

#[derive(Default)]
pub struct Row
{
    text: String,
    parts: Vec<Arc<str>>
}

impl Row
{
    pub fn new (text: &str, width: usize) -> Self {
        Self { 
            text:  text.to_string(), 
            parts: Vec::with_capacity(width)
        }
    }

    pub fn add (&mut self, start: usize, end: usize) -> Option<&mut Row> {
        if self.text.is_empty() || start > end || end > self.text.len() {
            return None;
        }
        self.parts.push(self.text[start..end].into());
        Some(self)
    }

    pub fn get (&self, column: usize) -> Option<&str> {
        if self.text.is_empty() || column >= self.parts.len() {
            return None;
        }
        Some(&self.parts[column])
    }

    pub fn get_parts (&self) -> &Vec<Arc<str>> {
        &self.parts
    }

}
