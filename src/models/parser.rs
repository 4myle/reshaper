/*
source template:    <date> <time>: <systolic>/<diastolic> <pulse>
source input:       2024-10-25 M: 131/79 63

target template:    <date>,<pulse>,<systolic>,<diastolic>
target produced:    2024-10-25,63,131,79
*/

use regex::Regex;
use std::slice::Iter;

const TAGMATCHER: &str = r"<([^>]*)>|([^<>]*)";

#[derive(Clone, Copy)]
pub enum Origin {
    Source,
    Target
}

#[derive(Default)]
struct Descriptor {
    variables: Vec<String>, // List of parsed variable names, ie. ["date", "pulse", "systolic", "diastolic"].
    positions: Vec<usize>   // List of positions into variables list above, ie. [0,3,2,1].
}

pub struct Parser
{
    source: Descriptor, // Source variable names and positions in source variables list.
    target: Descriptor, // Ditto for target.
    extractor: Regex,
    replacer: String
}

impl Default for Parser {
    fn default() -> Self {
        Self { 
            source: Descriptor::default(),
            target: Descriptor::default(),
            extractor: Result::unwrap(Regex::new("")),
            replacer: String::new()
        }
    }
}

impl Parser
{
    pub fn new () -> Self {
        Self::default()
    }

    pub fn variables (&self, origin: Origin) -> Iter<String> {
        match origin {
            Origin::Source => self.source.variables.iter(),
            Origin::Target => self.target.variables.iter()
        }
    }

    pub fn positions (&self, origin: Origin) -> Iter<usize> {
        match origin {
            Origin::Source => self.source.positions.iter(),
            Origin::Target => self.target.positions.iter()
        }
    }

    pub fn set_source (&mut self, template: &str) -> Result<&mut Self, &str> {
        if template.is_empty() {
            return Err("Template must contain at least one variable.");
        }
        if template.chars().filter(|c| *c == '<').count() != template.chars().filter(|c| *c == '>').count() {
            return Err("Brackets does not match.");
        }
        let  extractor = Result::unwrap(Regex::new(TAGMATCHER));
        let mut result = String::new();
        self.source = Descriptor::default();
        for capture in extractor.captures_iter(template) {
            if let Some(variable) = capture.get(1) {
                if variable.is_empty() {
                    return Err("Variable name must be at least one character long")
                }
                // Since the template contains variables in brackets, these are reused 
                // to form named groups in regex (eg. <date> => (?P<date>"[^"]+"|[^,]+)).
                result.push_str("(?P<");
                result.push_str(variable.as_str());
                result.push_str(r#">"[^"]*"|[^,]*)"#);
                self.source.variables.push(variable.as_str().to_string());
            } else if let Some(delimiter) = capture.get(2) {
                result.push_str(delimiter.as_str());
            }
        }
        self.source.positions = (0..self.source.variables.len()).collect();
        match Regex::new(&result) {
            Ok (r) => self.extractor = r,
            Err(_) => return Err("Error during template transformation.")
        }
        if self.source.variables.is_empty() {
            return Err("No variables found.");
        }
        Ok(self)
    }

    pub fn set_target(&mut self, template: &str) -> Result<&mut Self, &str> {
        if template.is_empty() {
            return Err("Template must contain at least one variable.");
        }
        if template.chars().filter(|c| *c == '<').count() != template.chars().filter(|c| *c == '>').count() {
            return Err("Brackets does not match.");
        }
        self.target = Descriptor::default();
        let  extractor = Result::unwrap(Regex::new(TAGMATCHER));
        let mut result = String::new();
        for capture in extractor.captures_iter(template) {
            if let Some(variable) = capture.get(1) {
                if variable.is_empty() {
                    return Err("Variable name must be at least one character long")
                }
                if let Some(index) = self.source.variables.iter().position(|v| v == variable.as_str()) {
                    result.push_str(&format!("${}", index + 1));
                    self.target.variables.push(self.source.variables[index].clone());
                    self.target.positions.push(index);
                } else {
                    return Err("Variable not found in source template.");
                }
            } else if let Some(delimiter) = capture.get(2) {
                result.push_str(delimiter.as_str());
            }
        }
        self.replacer = result;
        Ok(self)
    }

    pub fn split (&self, row: &str) -> Result<Vec<(usize,usize)>, &str> {
        if row.is_empty() || self.source.variables.is_empty() {
            return Err("Nothing to split.");
        }
        let mut result: Vec<(usize,usize)> = Vec::new();
        let mut slices = self.extractor.capture_locations();
        let bytes = row.as_bytes();
        self.extractor.captures_read(&mut slices, row);
        for index in 1..slices.len() {
            if let Some(mut slice) = slices.get(index) {
                if  slice.0 < bytes.len() && bytes[slice.0] == b'"' && bytes[slice.1-1] == b'"'{ 
                    slice.0 += 1;
                    slice.1 -= 1;
                }
                result.push(slice);
            }
        }
        Ok(result)
    }

    pub fn transform (&self, parts: &[Box<str>], do_quotes: bool) -> Result<String, &str> {
        if self.replacer.is_empty()  || self.target.positions.is_empty() {
            return Err("Nothing to transform.");
        }
        if parts.len() < self.target.positions.len() {
            return Err("Source variables fewer than target variables.");
        }
        let mut result = self.replacer.clone();
        for position in &self.target.positions { 
            let part: String = if do_quotes { 
                format!("\"{}\"", parts[*position])
            } else {
                parts[*position].to_string()
            };
            // Make sure "$11" is not replaced together with "$1" (hence "replacen").
            result = result.replacen(&format!("${}", position+1), &part, 1);
        }
        Ok(result)
    }

}
