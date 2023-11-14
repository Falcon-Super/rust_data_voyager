// data_analysis.rs
use arrow::ipc::reader::FileReader;
use arrow::record_batch::RecordBatch;
use std::error::Error;
use std::fs::File;

pub fn perform_data_analysis(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Starting data analysis for file: {}", input_file);

    let file = File::open(input_file)?;
    println!("File opened successfully.");

    let reader = FileReader::try_new(file, None)?;

    // Process each record batch
    for maybe_batch in reader {
        let batch = maybe_batch?;
        print_batch(&batch)?;
    }

    Ok(())
}

fn print_batch(batch: &RecordBatch) -> Result<(), Box<dyn Error>> {
    // Iterate over each column
    for i in 0..batch.num_columns() {
        let column = batch.column(i);
        println!("Column {}: {:?}", i, column);
    }

    Ok(())
}
