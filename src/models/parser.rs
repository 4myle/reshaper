
use regex::Regex;
use std::slice::Iter;

#[derive(Default)]
pub struct Parser
{
    variables: Vec<String>,
    source_after: Vec<String>,
    target_after: Vec<String>,
    target_index: Vec<u16>
}

impl Parser
{
    
    pub fn new () -> Self {
        Self { 
            variables: Vec::new(),
            source_after: Vec::new(),
            target_after: Vec::new(),
            target_index: Vec::new()
        }
    }

    pub fn variables (&self) -> Iter<String> {
        self.variables.iter()
    }

    pub fn parse_source (&mut self, text: &str) -> bool {
        let expression = Result::unwrap(Regex::new(r"<([^>]+)>|([^<>]+)"));
        self.variables = Vec::new();
        self.source_after = Vec::new();
        for capture in expression.captures_iter(text) {
            if let Some(tag) = capture.get(1) {
                self.variables.push(tag.as_str().to_string());
            } else if let Some(text) = capture.get(2) {
                self.source_after.push(text.as_str().to_string());
            }
        }
        /*
        Captures({0: 0..6/"<date>", 1: 1..5/"date", 2: None})
        Captures({0: 7..13/"<time>", 1: 8..12/"time", 2: None})
        Captures({0: 15..25/"<systolic>", 1: 16..24/"systolic", 2: None})
        Captures({0: 26..37/"<diastolic>", 1: 27..36/"diastolic", 2: None})
        Captures({0: 38..45/"<pulse>", 1: 39..44/"pulse", 2: None})
        */
        println!("{:?}", self.variables);
        println!("{:?}", self.source_after);
        self.target_after = Vec::new();
        self.target_index = Vec::new();
        true
}

    pub fn parse_target (&mut self, text: &str) -> bool {
        let expression = Result::unwrap(Regex::new(r"<([^>]+)>|([^<>]+)"));
        self.target_after = Vec::new();
        self.target_index = Vec::new();
        true
    }

    // pub fn is_empty (&self) -> bool {
    //     self.rows.is_empty()
    // }

}
