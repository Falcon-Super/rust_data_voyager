use arrow::datatypes::{DataType, Field, Schema};
use csv::{ReaderBuilder, StringRecord};
use std::error::Error;
use std::fs::File;
// use std::io::{self, Write};
use std::sync::Arc;
use crate::arrow_converter::convert_csv_to_arrow;

const EXPECTED_COLUMN_COUNT: usize = 5;

pub fn validate_and_convert_data(input_csv: &str, output_arrow: &str) -> Result<(), Box<dyn Error>> {
    let schema = validate_csv_input(input_csv)?;
    convert_csv_to_arrow(input_csv, output_arrow, &schema)?;
    validate_arrow_output(output_arrow)?;
    Ok(())
}

fn validate_csv_input(input_csv: &str) -> Result<Arc<Schema>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(input_csv)?;

        if let Some(headers) = rdr.headers().ok() {
            println!("CSV Headers: {:?}", headers);
            if headers.len() != EXPECTED_COLUMN_COUNT {
                println!("Unexpected column count: {} expected, but found {}", EXPECTED_COLUMN_COUNT, headers.len());
            }
        } else {
            println!("Failed to read headers from CSV");
        }

    for (index, result) in rdr.byte_records().enumerate() {
        let byte_record = match result {
            Ok(record) => record,
            Err(e) => {
                eprintln!("Error reading CSV at line {}: {}", index + 1, e);
                continue; // Skip this record and continue with the next
            }
        };

        // Attempt to convert ByteRecord to StringRecord
        let string_record = match StringRecord::from_byte_record(byte_record) {
            Ok(record) => record,
            Err(utf8_error) => {
                eprintln!("UTF-8 error at line {}: {:?}", index + 1, utf8_error);
                continue; // Skip this record and continue with the next
            }
        };

        // Now `string_record` is a `StringRecord` which you can validate.
        validate_record(&string_record, index)?;
    }

    // Assuming `build_schema` is a function you have defined to build your schema
    let schema = build_schema()?;
    Ok(Arc::new(schema))
}



fn build_schema() -> Result<Schema, Box<dyn Error>> {
    Ok(Schema::new(vec![
        Field::new("sepal.length", DataType::Float64, false),
        Field::new("sepal.width", DataType::Float64, false),
        Field::new("petal.length", DataType::Float64, false),
        Field::new("petal.width", DataType::Float64, false),
        Field::new("variety", DataType::Utf8, false),
    ]))
}

fn validate_record(record: &StringRecord, line_number: usize) -> Result<(), Box<dyn Error>> {
    if record.len() != EXPECTED_COLUMN_COUNT {
        return Err(From::from(format!("Unexpected number of columns at line {}", line_number + 1)));
    }
    // Additional validation for each field can be added here
    Ok(())
}

pub fn validate_arrow_output(arrow_output: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(arrow_output)?;
    let reader = arrow::ipc::reader::FileReader::try_new(file, None)?;

    if reader.schema().fields().len() != EXPECTED_COLUMN_COUNT {
        return Err(From::from("Unexpected column count in Arrow file"));
    }

    Ok(())
}
