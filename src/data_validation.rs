//data_validation.rs
use crate::error_handler::DataError;
use arrow::datatypes::{DataType, Field, Schema};
use csv::{ReaderBuilder, StringRecord};
use std::fs;
use std::path::Path;
use std::sync::Arc;

// Validate the input CSV and create a schema from its headers
pub fn validate_csv_input(input_csv: &str) -> Result<(Arc<Schema>, Vec<String>), DataError> {
    // Check if the file exists and is accessible
    if !Path::new(input_csv).exists() {
        return Err(DataError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Input CSV file does not exist: {}", input_csv),
        )));
    }
    let metadata = fs::metadata(input_csv).map_err(DataError::Io)?;
    if !metadata.is_file() || metadata.permissions().readonly() {
        return Err(DataError::Io(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            format!("Input CSV file is not accessible: {}", input_csv),
        )));
    }

    // Read headers from the CSV file
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(input_csv)
        .map_err(DataError::Csv)?;
    let headers: Vec<String> = rdr.headers()?.iter().map(|s| s.to_string()).collect();

    // Validate each record for correct number of columns and UTF-8 encoding
    for (index, result) in rdr.records().enumerate() {
        let record = result.map_err(DataError::Csv)?;
        if record.len() != headers.len() {
            return Err(DataError::Other(format!(
                "Unexpected number of columns at line {}. Expected {}, found {}",
                index + 2,
                headers.len(),
                record.len()
            )));
        }
        validate_record_utf8(&record, index + 2)?;
    }

    // Create a schema from the headers
    let schema = build_schema_from_headers(&headers)?;
    Ok((Arc::new(schema), headers))
}

// Validate UTF-8 encoding for each field in a record
fn validate_record_utf8(record: &StringRecord, line_number: usize) -> Result<(), DataError> {
    for (i, field) in record.iter().enumerate() {
        if !std::str::from_utf8(field.as_bytes()).is_ok() {
            return Err(DataError::Other(format!(
                "Invalid UTF-8 sequence at line {}, field {}",
                line_number,
                i + 1
            )));
        }
    }
    Ok(())
}

// Build schema from CSV headers
fn build_schema_from_headers(headers: &[String]) -> Result<Schema, DataError> {
    let fields: Vec<Field> = headers
        .iter()
        .map(|header| {
            // Dynamically determine the data type
            // Here we use some basic rules, but this can be expanded
            if header.contains("Year") || header.contains("Founded") {
                Field::new(header, DataType::UInt16, true)
            } else if header.contains("Number of employees") {
                Field::new(header, DataType::UInt32, true)
            } else {
                Field::new(header, DataType::Utf8, true)
            }
        })
        .collect();

    Ok(Schema::new(fields))
}
