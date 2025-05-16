extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::string::ToString;

use serde::{Serialize, Deserialize};

// Custom record type to replace csv::StringRecord
#[derive(Debug, Serialize, Deserialize)] // --> manually added
pub struct Record {
    fields: Vec<String>,
}

impl Record {
    pub fn new() -> Self {
        Record {
            fields: Vec::new(),
        }
    }

    pub fn push_field(&mut self, field: &str) {
        self.fields.push(field.to_string());
    }

    pub fn get(&self, index: usize) -> Option<&str> {
        self.fields.get(index).map(|s| s.as_str())
    }
}
