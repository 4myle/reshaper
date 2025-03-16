/*
source template:    <date> <time>: <systolic>/<diastolic> <pulse>
source input:       2024-10-25 M: 131/79 63

target template:    <date>,<pulse>,<systolic>,<diastolic>
target produced:    2024-10-25,63,131,79
*/

use regex::Regex;
// use std::slice::Iter;

// #[derive(Default)]
pub struct Parser
{
    variables: Vec<String>,
    source_expression: Regex,
    target_expression: Regex
}

impl Default for Parser {
    fn default() -> Self {
        Self { 
            variables: Vec::default(), 
            source_expression: Result::unwrap(Regex::new("")),
            target_expression: Result::unwrap(Regex::new(""))
        }
    }
}

impl Parser
{
    
    pub fn new () -> Self {
        Self::default()
    }

    // pub fn variables (&self) -> Iter<String> {
    //     self.variables.iter()
    // }

    pub fn set_source (&mut self, text: &str) -> Result<&mut Self, &str> {
        if text.is_empty() {
            return Err("")
        }
        let extractor  = Result::unwrap(Regex::new(r"<([^>]+)>|([^<>]+)"));
        let mut result = String::new();
        self.variables = Vec::new();
        for capture in extractor.captures_iter(text) {
            if let Some(variable) = capture.get(1) {
                result.push_str("(?P<");
                result.push_str(variable.as_str());
                result.push_str(">\\S+)");
                self.variables.push(variable.as_str().to_string());
            } else if let Some(delimiter) = capture.get(2) {
                result.push_str(delimiter.as_str().replacen(' ', "\\s+", 1).as_str());
            }
        }
        match Regex::new(&result) {
            Ok (r) => self.source_expression = r,
            Err(_) => return Err("Error during conversion")
        }
        if self.variables.is_empty() || self.variables.iter().any(String::is_empty) {
            return Err("Syntax error in template");
        }
        Ok(self)
    }

    pub fn set_target (&mut self, text: &str) -> Result<&mut Self, &str> {
        if text.is_empty() {
            return Err("")
        }
        let extractor  = Result::unwrap(Regex::new(r"<([^>]+)>|([^<>]+)"));
        let mut result = String::new();
        //TODO: expression on the form "$1,$4,$3,$2"?
        //TODO: implement.
        // println!("{:?}", self.target_expression);
        for capture in extractor.captures_iter(text) {
            if let Some(_variable) = capture.get(1) {
                result.push_str("$1");
            } else if let Some(delimiter) = capture.get(2) {
                result.push_str(delimiter.as_str());
            }
        }
        self.target_expression = extractor;
        if false {
            return Err("REMOVE ME");
        }
        Ok(self)
    }

    pub fn split (&self, row: &str) -> Vec<(usize,usize)> {
        let mut result: Vec<(usize,usize)> = Vec::new();
        let mut slices = self.source_expression.capture_locations();
        self.source_expression.captures_read(&mut slices, row);
        for index in 1..slices.len() {
            result.push(slices.get(index).unwrap_or_default());
        }
        result
    }

}
