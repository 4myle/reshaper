
use crate::models::row::Row;

#[derive(Default)]
pub struct Table
{
    rows: Vec<Row>
}

impl Table 
{
    
    pub fn new () -> Self {
        Self { 
            rows: Vec::new()
        }
    }

    pub fn add (&mut self, string: String) -> Option<&mut Row> {
        self.rows.push(Row::new(string)); // Move occurs here.
        self.rows.last_mut()
    }

    pub fn get (&self, row: usize, column: usize) -> Option<&str> {
        if row >= self.rows.len() {
            return None;
        }
        Some(self.rows[row].get(column))
    }

    pub fn is_empty (&self) -> bool {
        self.rows.is_empty()
    }

    pub fn rows_total (&self) -> usize {
        self.rows.len()

    }

}
