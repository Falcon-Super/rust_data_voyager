use arrow::array::{ArrayRef, Int64Array};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::error::ArrowError;
use arrow::record_batch::RecordBatch;
use csv::Reader;
use polars::prelude::*;
use std::error::Error;
use std::fs::File;

pub fn perform_data_analysis(input_file: &str) -> Result<(), Box<dyn Error>> {
    // Open the CSV file with `input_file`
    let file = File::open(input_file)?;
    let mut reader = Reader::from_reader(file);

    // Read CSV header to get column names
    let header = reader.headers()?.clone();

    let schema = Schema::new(
        header
            .iter()
            .map(|field| Field::new(&*field, DataType::Utf8, true))
            .collect::<Vec<Field>>(), // Specify the type explicitly here
    );

    for result in reader.records() {
        let record = result?;

        // Parse record into Arrow arrays
        let arrays: Result<Vec<ArrayRef>, ArrowError> = record
            .iter()
            .map(|val| {
                val.parse::<i64>()
                    .map_err(|_e| ArrowError::ParseError("Failed to parse i64".to_string())) // Handle the error properly
                    .map(|parsed_val| Arc::new(Int64Array::from(vec![parsed_val])) as ArrayRef)
            })
            .collect();

        let batch = RecordBatch::try_new(schema.clone().into(), arrays?)?;
        for i in 0..batch.num_columns() {
            let array = batch.column(i);
            // Calculate the sum
            let sum: f64 = array
                .as_any()
                .downcast_ref::<Int64Array>()
                .unwrap()
                .values()
                .iter()
                .map(|&x| x as f64)
                .sum();

            println!("Column {}: Sum = {}", i, sum);
        }
    }

    Ok(())
}
