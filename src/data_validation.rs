//data_validation.rs
use crate::arrow_converter::convert_csv_to_arrow;
use crate::error_handler::DataError;
use crate::EXPECTED_COLUMN_COUNT;
use arrow::datatypes::{DataType, Field, Schema};
use csv::{ReaderBuilder, StringRecord};
use std::fs;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

pub fn validate_csv_input(
    input_csv: &str,
    expected_column_count: usize,
) -> Result<Arc<Schema>, DataError> {
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

    // CSV reader setup
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(input_csv)
        .map_err(DataError::Csv)?;

    let headers = rdr.headers().map_err(DataError::Csv)?;
    if headers.len() != expected_column_count {
        return Err(DataError::Other(format!(
            "Unexpected number of columns in header. Expected {}, found {}",
            expected_column_count,
            headers.len()
        )));
    }

    for (index, result) in rdr.records().enumerate() {
        let record = result.map_err(DataError::Csv)?;
        if record.len() != expected_column_count {
            return Err(DataError::Other(format!(
                "Unexpected number of columns at line {}. Expected {}, found {}",
                index + 2,
                expected_column_count,
                record.len()
            )));
        }
    }

    // CSV reader with UTF-8 validation
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(input_csv)
        .map_err(DataError::Csv)?;

    // Check headers
    let headers = rdr.headers().map_err(DataError::Csv)?;
    if headers.len() != expected_column_count {
        return Err(DataError::Other(format!(
            "Unexpected number of columns in header. Expected {}, found {}",
            expected_column_count,
            headers.len()
        )));
    }

    // Validate each record
    for (index, result) in rdr.records().enumerate() {
        let record = result.map_err(DataError::Csv)?;
        validate_record_utf8(&record, index + 2)?;
        if record.len() != expected_column_count {
            return Err(DataError::Other(format!(
                "Unexpected number of columns at line {}. Expected {}, found {}",
                index + 2,
                expected_column_count,
                record.len()
            )));
        }
    }

    // Build schema
    let schema = build_schema()?;
    Ok(Arc::new(schema))
}

// Validate the UTF-8 encoding of each field in a record
fn validate_record_utf8(record: &StringRecord, line_number: usize) -> Result<(), DataError> {
    for (i, field) in record.iter().enumerate() {
        // Use std::str::from_utf8 instead of str::from_utf8
        if !std::str::from_utf8(field.as_bytes()).is_ok() {
            return Err(DataError::Other(format!(
                "Invalid UTF-8 sequence at line {}, field {}",
                line_number, i + 1
            )));
        }
    }
    Ok(())
}

pub fn validate_and_convert_data(input_csv: &str, output_arrow: &str) -> Result<(), DataError> {
    let schema_arc = validate_csv_input(input_csv, EXPECTED_COLUMN_COUNT)?;
    convert_csv_to_arrow(input_csv, output_arrow, &schema_arc)?;
    validate_arrow_output(output_arrow)
}

pub fn validate_output_directory(output_path: &str) -> Result<(), DataError> {
    let output_dir = Path::new(output_path)
        .parent()
        .ok_or(DataError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Output path does not have a parent directory",
        )))?;

    if !output_dir.exists() {
        fs::create_dir_all(output_dir).map_err(DataError::Io)?;
    }

    if !output_dir.is_dir() {
        return Err(DataError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Output path is not a directory: {}", output_dir.display()),
        )));
    }

    let metadata = fs::metadata(output_dir).map_err(DataError::Io)?;
    if metadata.permissions().readonly() {
        return Err(DataError::Io(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            format!(
                "Output directory is not writable (permissions issue): {}",
                output_dir.display()
            ),
        )));
    }

    Ok(())
}

fn build_schema() -> Result<Schema, DataError> {
    Ok(Schema::new(vec![
        Field::new("Index", DataType::UInt64, false), // Assuming Index is always present and is an unsigned integer
        Field::new("Organization Id", DataType::Utf8, false), // Strings are represented as Utf8
        Field::new("Name", DataType::Utf8, false),
        Field::new("Website", DataType::Utf8, false),
        Field::new("Country", DataType::Utf8, false),
        Field::new("Description", DataType::Utf8, false),
        Field::new("Founded", DataType::UInt16, false), // Assuming Founded is a year, which fits within a UInt16
        Field::new("Industry", DataType::Utf8, false),
        Field::new("Number of employees", DataType::UInt32, false), // Assuming the number of employees fits within a UInt32
    ]))
}

pub fn validate_arrow_output(arrow_output: &str) -> Result<(), DataError> {
    let file = File::open(arrow_output).map_err(DataError::Io)?;
    let reader = arrow::ipc::reader::FileReader::try_new(file, None).map_err(DataError::Arrow)?;

    if reader.schema().fields().len() != EXPECTED_COLUMN_COUNT {
        return Err(DataError::Arrow(arrow::error::ArrowError::SchemaError(
            "Unexpected column count in Arrow file".to_string(),
        )));
    }

    Ok(())
}
