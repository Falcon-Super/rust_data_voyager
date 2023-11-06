use arrow::array::{ArrayRef, Int64Array};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::error::ArrowError;
use arrow::record_batch::RecordBatch;
use polars::prelude::*;
use std::error::Error;
use std::fs::File;
use csv::Reader;

pub fn perform_data_analysis() -> Result<(), Box<dyn Error>> {
    // Open the CSV file
    let file = File::open("iris.arrow")?;
    let mut reader = Reader::from_reader(file);

    // Read CSV header to get column names
    let header = reader.headers()?.clone();

    let schema = Schema::new(
        header.iter()
            .map(|field| {
                Field::new(&*field, DataType::Utf8, true)
            })
            .collect::<Vec<Field>>(), // Specify the type explicitly here
    );
    

    for result in reader.records() {
        let record = result?;

        // Parse record into Arrow arrays
        let arrays: Result<Vec<ArrayRef>, ArrowError> = record.iter()
            .map(|val| {
                Ok(Arc::new(Int64Array::from(vec![val.parse::<i64>().unwrap()])) as ArrayRef)
            })
            .collect();

            let batch = RecordBatch::try_new(schema.clone().into(), arrays?)?;
        for i in 0..batch.num_columns() {
            let array = batch.column(i);

            // Calculate the sum
            let sum: f64 = array.as_any().downcast_ref::<Int64Array>().unwrap().values().iter().map(|&x| x as f64).sum();

            println!("Column {}: Sum = {}", i, sum);
        }
    }

    Ok(())
}
