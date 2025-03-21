
use std::rc::Rc;
use crate::models::row::Row;

#[derive(Default)]
pub struct Table
{
    width: usize,
    rows: Vec<Row>
}

impl Table 
{
    
    pub fn new (columns: usize) -> Self {
        Self { 
            width: columns,
            rows: Vec::new()
        }
    }

    pub fn add (&mut self, text: &str, slices: Vec<(usize,usize)>) -> Option<&mut Row> {
        let mut row = Row::new(text, self.width);
        for range in slices {
            row.add(range.0, range.1);
        }
        self.rows.push(row);
        self.rows.last_mut()
    }

    pub fn get (&self, index: usize, column: usize) -> Option<&str> {
        if index >= self.rows.len() || column >= self.width {
            return None;
        }
        self.rows[index].get(column)
    }

    pub fn get_parts (&self, index: usize) -> Option<&Vec<Rc<str>>> {
        if index >= self.rows.len() {
            return None;
        }
        Some(self.rows[index].get_parts())
    }

    pub fn is_empty (&self) -> bool {
        self.rows.is_empty()
    }

    pub fn row_count (&self) -> usize {
        self.rows.len()

    }
    
}
