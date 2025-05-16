extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::string::ToString;

use csv_core::{ReaderBuilder};
use serde::{Serialize, Deserialize};

// Custom record type to replace csv::StringRecord
#[derive(Debug, Serialize, Deserialize)] // --> manually added
struct Record {
    fields: Vec<String>,
}

impl Record {
    fn new() -> Self {
        Record {
            fields: Vec::new(),
        }
    }

    fn push_field(&mut self, field: &str) {
        self.fields.push(field.to_string());
    }

    fn get(&self, index: usize) -> Option<&str> {
        self.fields.get(index).map(|s| s.as_str())
    }
}

fn parse_csv_record(input: &str) -> Option<Record> {
    let mut reader = ReaderBuilder::new().build();
    
    let mut record = Record::new();
    let mut field = String::new();
    let mut input_pos = 0;
    let mut field_pos = 0;
    let mut buffer = [0u8; 1024];
    
    loop {
        let (result, nin, nout) = reader.read_field(
            &input.as_bytes()[input_pos..],
            &mut buffer[field_pos..],
        );
        
        input_pos += nin;
        field_pos += nout;
        
        match result {
            csv_core::ReadFieldResult::InputEmpty => break,
            csv_core::ReadFieldResult::OutputFull => {
                // Buffer is full, append what we have and continue
                field.push_str(core::str::from_utf8(&buffer[..field_pos]).ok()?);
                field_pos = 0;
            }
            csv_core::ReadFieldResult::Field { .. } => {
                // Field is complete
                field.push_str(core::str::from_utf8(&buffer[..field_pos]).ok()?);
                record.push_field(&field);
                field.clear();
                field_pos = 0;
            }
            csv_core::ReadFieldResult::End => break,
        }
    }
    
    if !field.is_empty() {
        record.push_field(&field);
    }
    
    Some(record)

}
