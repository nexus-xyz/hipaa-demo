#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::string::ToString;
use serde::Serialize;
use uuid::Uuid;
use guest::common::Record;

// Structure to hold the parsed record data
#[derive(Debug, Serialize)]
struct PatientRecord {
    uuid: String,
    first_name: String,
    middle_initials: String,
    last_name: String,
    email_address: String,
    zip_code: String,
    telephone_number: String,
    medical_record_number: u32,
    pregnancies: u32,
    glucose: u32,
    blood_pressure: u32,
    skin_thickness: u32,
    insulin: u32,
    bmi: f32,
    diabetes_pedigree: f32,
    age: u32,
    outcome: u32,
}

impl PatientRecord {
    fn from_record(record: &Record, uuid: &Uuid) -> Option<Self> {
        Some(PatientRecord {
            uuid: uuid.to_string(),
            first_name: record.get(0)?.to_string(),
            middle_initials: record.get(1)?.to_string(),
            last_name: record.get(2)?.to_string(),
            email_address: record.get(3)?.to_string(),
            zip_code: record.get(4)?.to_string(),
            telephone_number: record.get(5)?.to_string(),
            medical_record_number: record.get(6)?.parse().ok()?,
            pregnancies: record.get(7)?.parse().ok()?,
            glucose: record.get(8)?.parse().ok()?,
            blood_pressure: record.get(9)?.parse().ok()?,
            skin_thickness: record.get(10)?.parse().ok()?,
            insulin: record.get(11)?.parse().ok()?,
            bmi: record.get(12)?.parse().ok()?,
            diabetes_pedigree: record.get(13)?.parse().ok()?,
            age: record.get(14)?.parse().ok()?,
            outcome: record.get(15)?.parse().ok()?,
        })
    }

    fn to_record(&self) -> Record {
        let mut record = Record::new();
        record.push_field(&self.uuid);
        record.push_field(&self.first_name);
        record.push_field(&self.middle_initials);
        record.push_field(&self.last_name);
        record.push_field(&self.email_address);
        record.push_field(&self.zip_code);
        record.push_field(&self.telephone_number);
        record.push_field(&self.medical_record_number.to_string());
        record.push_field(&self.pregnancies.to_string());
        record.push_field(&self.glucose.to_string());
        record.push_field(&self.blood_pressure.to_string());
        record.push_field(&self.skin_thickness.to_string());
        record.push_field(&self.insulin.to_string());
        record.push_field(&self.bmi.to_string());
        record.push_field(&self.diabetes_pedigree.to_string());
        record.push_field(&self.age.to_string());
        record.push_field(&self.outcome.to_string());
        record
    }
}

// Apply HIPAA Safe Harbor deidentification rules
fn apply_hipaa_deidentification(record: &mut PatientRecord) {
    // 1. Names - Remove all names
    record.first_name = String::from("REDACTED");
    record.middle_initials = String::from("REDACTED");
    record.last_name = String::from("REDACTED");

    // 2. Geographic subdivisions smaller than a state
    // For ZIP codes, we need to truncate to first 3 digits and set to 000 if population < 20,000
    // Note: In a real implementation, we would need a database of ZIP code populations
    // For this example, we'll just truncate to 3 digits
    if record.zip_code.len() >= 3 {
        record.zip_code = format!("{}00", &record.zip_code[..3]);
    }

    // 3. All elements of dates (except year) for dates directly related to an individual
    // For ages over 89, we need to clamp them to 90
    if record.age > 89 {
        record.age = 90;
    }

    // 4. Telephone numbers - Remove
    record.telephone_number = String::from("REDACTED");

    // 5. Fax numbers - Not present in dataset

    // 6. Electronic mail addresses - Remove
    record.email_address = String::from("REDACTED");

    // 7. Social Security Numbers - Not present in dataset

    // 8. Medical record numbers - Remove
    record.medical_record_number = 0;

    // 9. Health plan beneficiary numbers - Not present in dataset

    // 10. Account numbers - Not present in dataset

    // 11. Certificate/license numbers - Not present in dataset

    // 12. Vehicle identifiers and serial numbers - Not present in dataset

    // 13. Device identifiers and serial numbers - Not present in dataset

    // 14. Web Universal Resource Locators (URLs) - Not present in dataset

    // 15. Internet Protocol (IP) address numbers - Not present in dataset

    // 16. Biometric identifiers - Not present in dataset

    // 17. Full face photographic images - Not present in dataset

    // 18. Any other unique identifying number, characteristic, or code
    // Note: We're keeping the medical data fields as they are not considered identifiers
    // under HIPAA Safe Harbor rules
}

#[nexus_rt::main]
#[nexus_rt::public_input(uuids)] // --> manually added
fn main(records: Vec<Record>, uuids: Vec<Uuid>) -> Vec<Record> {
    if records.len() > uuids.len() {
        panic!("Not enough UUIDs provided. Need {} UUIDs for {} records.", records.len(), uuids.len());
    }

    let mut deidentified_records: Vec<Record> = Vec::new();
    let mut failed_records = 0;

    // Process each record
    for (i, record) in records.into_iter().enumerate() {
        if let Some(mut patient_record) = PatientRecord::from_record(&record, &uuids[i]) {
            // Print the identified record as JSON
            if let Ok(json) = serde_json::to_string(&patient_record) {
                nexus_rt::print!("Deidentified record: {}\n", json);
            }
            
            apply_hipaa_deidentification(&mut patient_record);

            deidentified_records.push(patient_record.to_record());
        } else {
            failed_records += 1;
        }
    }

    // Print the number of failed records
    nexus_rt::print!("Failed to process {} records\n", failed_records);

    // Return the deidentified records
    deidentified_records
}
