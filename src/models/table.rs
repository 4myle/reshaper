
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

    pub fn get (&self, row: usize, column: usize) -> Option<&str> {
        if row >= self.rows.len() {
            return None;
        }
        self.rows[row].get(column)
    }

    pub fn width (&self) -> usize {
        self.width
    }

    pub fn is_empty (&self) -> bool {
        self.rows.is_empty()
    }

    pub fn rows_total (&self) -> usize {
        self.rows.len()

    }

}
