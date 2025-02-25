
#[derive(Default)]
pub struct Row
{
    data: String,
    part: Vec<*const str>
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
        if start > end || end > self.data.len() {
            return None;
        }
        self.part.push(&self.data[start..end]);
        Some(())
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
