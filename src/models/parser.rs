/*
source template:    <date> <time>: <systolic>/<diastolic> <pulse>
source input:       2024-10-25 M: 131/79 63

target template:    <date>,<pulse>,<systolic>,<diastolic>
target produced:    2024-10-25,63,131,79
*/

use regex::Regex;
use std::rc::Rc;
use std::slice::Iter;

const TAGMATCHER: &str = r"<([^>]+)>|([^<>]+)";

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
            return Err("No text to process.")
        }
        if template.chars().filter(|c| *c == '<').count() != template.chars().filter(|c| *c == '>').count() {
            return Err("Brackets unclosed.")
        }
        let  extractor = Result::unwrap(Regex::new(TAGMATCHER));
        let mut result = String::new();
        self.source = Descriptor::default();
        self.target = Descriptor::default();
        for capture in extractor.captures_iter(template) {
            if let Some(variable) = capture.get(1) {
                result.push_str("(?P<");
                result.push_str(variable.as_str());
                result.push_str(">\\S+)");
                self.source.variables.push(variable.as_str().to_string());
            } else if let Some(delimiter) = capture.get(2) {
                result.push_str(delimiter.as_str().replacen(' ', "\\s+", 1).as_str());
            }
        }
        self.source.positions = (0..self.source.variables.len()).collect();
        match Regex::new(&result) {
            Ok (r) => self.extractor = r,
            Err(_) => return Err("Error during template transformation.")
        }
        if self.source.variables.is_empty() || self.source.variables.iter().any(String::is_empty) {
            return Err("No variables found or wrong variable syntax.");
        }
        Ok(self)
    }

    pub fn set_target(&mut self, template: &str) -> Result<&mut Self, &str> {
        if template.is_empty() {
            return Err("No text to process.");
        }
        if template.chars().filter(|c| *c == '<').count() != template.chars().filter(|c| *c == '>').count() {
            return Err("Brackets unclosed.");
        }
        self.target.positions = Vec::new();
        let  extractor = Result::unwrap(Regex::new(TAGMATCHER));
        let mut result = String::new();
        for capture in extractor.captures_iter(template) {
            if let Some(name) = capture.get(1) {
                if let Some(index) = self.source.variables.iter().position(|v| v == name.as_str()) {
                    result.push_str(&format!("${}", index + 1));
                    self.target.variables.push(self.source.variables[index].clone());
                    self.target.positions.push(index);
                } else {
                    return Err("Unknown variable in target template.");
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
            return Err("Nothing to slice.");
        }
        let mut result: Vec<(usize,usize)> = Vec::new();
        let mut slices = self.extractor.capture_locations();
        self.extractor.captures_read(&mut slices, row);
        for index in 1..slices.len() {
            result.push(slices.get(index).unwrap_or_default());
        }
        Ok(result)
    }

    pub fn transform (&self, parts: &[Rc<str>]) -> Result<String, &str> {
        if self.replacer.is_empty()  || self.target.positions.is_empty() {
            return Err("Nothing to transform.");
        }
        if parts.len() < self.target.positions.len() {
            return Err("Source variables fewer than target variables.");
        }
        let mut result = self.replacer.clone();
        for position in &self.target.positions {
            result = result.replace(&format!("${}", position+1), &parts[*position]);
        }
        Ok(result)
    }

}
