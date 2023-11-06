use arrow::array::*;
use arrow::datatypes::*;
use arrow::ipc::writer::FileWriter;
use arrow::record_batch::RecordBatch;
use csv::ReaderBuilder;
use std::fs::File;
use std::error::Error;
use std::sync::Arc;

pub fn convert_csv_to_arrow(input_csv: &str, output_arrow: &str, schema: &Arc<Schema>) -> Result<(), Box<dyn Error>> {
    // Read the CSV file using the `input_csv` path provided as an argument
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(input_csv)?;

    // Create an Arrow writer for the `output_arrow` file provided as an argument
    let file = File::create(output_arrow)?;
    let mut arrow_writer = FileWriter::try_new(&file, &schema)?;

    // Initialize vectors for Arrow arrays
    let mut sepal_length_values = Vec::new();
    let mut sepal_width_values = Vec::new();
    let mut petal_length_values = Vec::new();
    let mut petal_width_values = Vec::new();
    let mut species_values = Vec::new();

    for result in csv_reader.records() {
        let record = result?;

        // Parse the CSV record
        let sepal_length: f64 = record[0].parse()?;
        let sepal_width: f64 = record[1].parse()?;
        let petal_length: f64 = record[2].parse()?;
        let petal_width: f64 = record[3].parse()?;
        let species = record[4].to_string();

        // Append values to Arrow arrays
        sepal_length_values.push(sepal_length);
        sepal_width_values.push(sepal_width);
        petal_length_values.push(petal_length);
        petal_width_values.push(petal_width);
        species_values.push(species);
    }

    // Create Arrow arrays from the vectors
    let sepal_length_array = Arc::new(Float64Array::from(sepal_length_values));
    let sepal_width_array = Arc::new(Float64Array::from(sepal_width_values));
    let petal_length_array = Arc::new(Float64Array::from(petal_length_values));
    let petal_width_array = Arc::new(Float64Array::from(petal_width_values));
    let species_array = Arc::new(StringArray::from(species_values));

    // Create a RecordBatch using the schema provided as an argument
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            sepal_length_array,
            sepal_width_array,
            petal_length_array,
            petal_width_array,
            species_array,
        ],
    )?;

    // Write the RecordBatch to the Arrow file
    arrow_writer.write(&batch)?;
    arrow_writer.finish()?;

    println!("Data converted to Arrow format and saved as '{}'", output_arrow);

    Ok(())
}
