// data_cleaning.rs
use csv::{ReaderBuilder, WriterBuilder, StringRecord};
use std::error::Error;
use std::string::FromUtf8Error;

pub fn clean_csv_data(input_csv: &str, output_csv: &str) -> Result<String, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .from_path(input_csv)?;

    let mut wtr = WriterBuilder::new()
        .from_path(output_csv)?;

    for result in rdr.byte_records() {
        let byte_record = result?;
        let record = byte_record_to_string_record(byte_record)?;
        let cleaned_record = clean_record(&record)?;

        // Print the cleaned record
        println!("{:?}", cleaned_record);

        wtr.write_record(&cleaned_record)?;
    }

    wtr.flush()?;
    Ok(output_csv.to_string())
}

// Convert ByteRecord to StringRecord, replacing invalid UTF-8 sequences
fn byte_record_to_string_record(byte_record: csv::ByteRecord) -> Result<StringRecord, FromUtf8Error> {
    let mut fields = Vec::new();
    for field in byte_record.iter() {
        let field_str = String::from_utf8_lossy(field);
        fields.push(field_str.into_owned());
    }
    Ok(StringRecord::from(fields))
}

// Example function to perform some cleaning operations on a record
fn clean_record(record: &StringRecord) -> Result<StringRecord, Box<dyn Error>> {
    // Here, add any specific cleaning operations you need.
    // For example, trimming whitespace, handling missing values, etc.
    Ok(record.clone())
}
