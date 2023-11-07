use arrow::datatypes::{DataType, Field, Schema};
use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::sync::Arc;
use std::path::Path;
use std::fs;
use crate::arrow_converter::convert_csv_to_arrow;
use crate::EXPECTED_COLUMN_COUNT;
use crate::data_cleaning::clean_csv_data;

pub fn validate_csv_input(input_csv: &str, output_csv: &str, expected_column_count: usize) -> Result<Arc<Schema>, Box<dyn Error>> {
    // Check if the input CSV file exists and is readable
    if !Path::new(input_csv).exists() {
        return Err(From::from(format!("Input CSV file does not exist: {}", input_csv)));
    }

    let metadata = fs::metadata(input_csv)?;
    if !metadata.is_file() || metadata.permissions().readonly() {
        return Err(From::from(format!("Input CSV file is not accessible: {}", input_csv)));
    }

    // Clean the CSV data first
    clean_csv_data(input_csv, output_csv)?;

    // Open the cleaned CSV to validate
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(output_csv)?;

    // Read headers and validate them
    let headers = rdr.headers()?;
    if headers.len() != expected_column_count {
        return Err(From::from(format!("Unexpected number of columns in header. Expected {}, found {}", expected_column_count, headers.len())));
    }

    // Validate that each row has the correct number of columns
    for (index, result) in rdr.records().enumerate() {
        let record = result?;
        if record.len() != expected_column_count {
            return Err(From::from(format!("Unexpected number of columns at line {}. Expected {}, found {}", index + 2, expected_column_count, record.len())));
        }
    }

    // If all validations pass, build and return the schema
    let schema = build_schema()?;
    Ok(Arc::new(schema))
}


pub fn validate_and_convert_data(input_csv: &str, output_arrow: &str) -> Result<(), Box<dyn Error>> {
    let cleaned_csv_path = "Datasets/csv-files//cleaned_data.csv";
    let schema_arc = validate_csv_input(input_csv, cleaned_csv_path, EXPECTED_COLUMN_COUNT)?;
    convert_csv_to_arrow(input_csv, output_arrow, &schema_arc)?;
    validate_arrow_output(output_arrow)
}


pub fn validate_output_directory(output_path: &str) -> Result<(), Box<dyn Error>> {
    let output_dir = Path::new(output_path).parent().ok_or("Output path does not have a parent directory")?;

    if !output_dir.exists() {
        // Attempt to create the directory if it does not exist
        fs::create_dir_all(output_dir)?;
    }

    if !output_dir.is_dir() {
        return Err(From::from(format!("Output path is not a directory: {}", output_dir.display())));
    }

    // Check if the directory is writable
    let metadata = fs::metadata(output_dir)?;
    if metadata.permissions().readonly() {
        return Err(From::from(format!("Output directory is not writable (permissions issue): {}", output_dir.display())));
    }

    Ok(())
}




fn build_schema() -> Result<Schema, Box<dyn Error>> {
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


// fn validate_record(record: &StringRecord, line_number: usize) -> Result<(), Box<dyn Error>> {
//     if record.len() != EXPECTED_COLUMN_COUNT {
//         return Err(From::from(format!("Unexpected number of columns at line {}", line_number + 1)));
//     }
//     // Additional validation for each field can be added here
//     Ok(())
// }

pub fn validate_arrow_output(arrow_output: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(arrow_output)?;
    let reader = arrow::ipc::reader::FileReader::try_new(file, None)?;

    if reader.schema().fields().len() != EXPECTED_COLUMN_COUNT {
        return Err(From::from("Unexpected column count in Arrow file"));
    }

    Ok(())
}
