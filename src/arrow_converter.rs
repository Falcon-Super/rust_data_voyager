//arrow_converter.rs
use arrow::array::*;
use arrow::datatypes::{DataType, Field, Schema};
use arrow::ipc::writer::FileWriter;
use arrow::record_batch::RecordBatch;
use csv::{ReaderBuilder, StringRecord};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

// Function to infer schema from CSV header
fn infer_schema(headers: &StringRecord) -> Arc<Schema> {
    let fields: Vec<Field> = headers
        .iter()
        .map(|name| {
            // Infer the type based on the column name
            if name.contains("Year") || name.contains("Founded") {
                Field::new(name, DataType::UInt16, true)
            } else if name.contains("Number of employees") {
                Field::new(name, DataType::UInt32, true)
            } else {
                Field::new(name, DataType::Utf8, true)
            }
        })
        .collect();

    Arc::new(Schema::new(fields))
}

pub fn convert_csv_to_arrow(input_csv: &str, output_arrow: &str) -> Result<(), Box<dyn Error>> {
    // Read and clean the CSV file
    let mut file = File::open(input_csv)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let clean_contents = contents.replace("\u{feff}", ""); // Replace BOM if present

    // Initialize CSV reader
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(clean_contents.as_bytes());

    let headers = csv_reader.headers()?.clone();
    let schema = infer_schema(&headers);

    // Initialize builders for each column
    let mut builders: Vec<Box<dyn ArrayBuilder>> = vec![];
    for field in schema.fields() {
        match field.data_type() {
            DataType::Utf8 => builders.push(Box::new(StringBuilder::new())),
            DataType::UInt16 => builders.push(Box::new(UInt16Builder::new())),
            DataType::UInt32 => builders.push(Box::new(UInt32Builder::new())),
            // Add other data types as needed
            _ => return Err("Unsupported data type".into()),
        }
    }

    // Process each row
    for result in csv_reader.records() {
        let record = result?;
        for (i, field) in schema.fields().iter().enumerate() {
            let value = record.get(i).unwrap_or_default();
            match field.data_type() {
                DataType::Utf8 => {
                    let builder = builders[i]
                        .as_any_mut()
                        .downcast_mut::<StringBuilder>()
                        .unwrap();
                    builder.append_value(value.to_string());
                }
                DataType::UInt16 => {
                    let builder = builders[i]
                        .as_any_mut()
                        .downcast_mut::<UInt16Builder>()
                        .unwrap();
                    builder.append_value(value.parse()?);
                }
                DataType::UInt32 => {
                    let builder = builders[i]
                        .as_any_mut()
                        .downcast_mut::<UInt32Builder>()
                        .unwrap();
                    builder.append_value(value.parse()?);
                }
                // Add other data types as needed
                _ => {}
            }
        }
    }

    // Create a RecordBatch
    let mut column_arrays: Vec<ArrayRef> = Vec::new();
    for mut builder in builders {
        column_arrays.push(builder.finish());
    }
    let batch = RecordBatch::try_new(schema.clone(), column_arrays)?;

    // Create the Arrow file writer
    let file = File::create(output_arrow)?;
    let mut arrow_writer = FileWriter::try_new(file, &schema)?;

    // Write the RecordBatch to the Arrow file
    arrow_writer.write(&batch)?;
    arrow_writer.finish()?;

    println!(
        "Data converted to Arrow format and saved as '{}'",
        output_arrow
    );

    Ok(())
}

// Additional validation functions can be included here or in a separate module
