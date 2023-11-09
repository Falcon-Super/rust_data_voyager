use arrow::array::*;
use arrow::datatypes::*;
use arrow::ipc::writer::FileWriter;
use arrow::record_batch::RecordBatch;
use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::sync::Arc;

fn clean_and_read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.replace("\u{feff}", "")) // Replace BOM if present
}

pub fn convert_csv_to_arrow(
    input_csv: &str,
    output_arrow: &str,
    schema: &Arc<Schema>,
) -> Result<(), Box<dyn Error>> {
    let file_contents = clean_and_read_file(input_csv)?;
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file_contents.as_bytes()); // Use the cleaned string here

    let file = File::create(output_arrow)?;
    let mut arrow_writer = FileWriter::try_new(file, schema)?;

    // Initialize vectors for each column
    let mut index_values = Vec::new();
    let mut org_id_values = Vec::new();
    let mut name_values = Vec::new();
    let mut website_values = Vec::new();
    let mut country_values = Vec::new();
    let mut description_values = Vec::new();
    let mut founded_values = Vec::new();
    let mut industry_values = Vec::new();
    let mut number_of_employees_values = Vec::new();

    for result in csv_reader.records() {
        let record = result?;

        // Parse each column and handle potential errors
        index_values.push(
            record[0]
                .parse::<u64>()
                .map_err(|e| format!("Index parsing error: {}", e))?,
        );
        org_id_values.push(record[1].to_string());
        name_values.push(record[2].to_string());
        website_values.push(record[3].to_string());
        country_values.push(record[4].to_string());
        description_values.push(record[5].to_string());
        founded_values.push(
            record[6]
                .parse::<u16>()
                .map_err(|e| format!("Founded year parsing error: {}", e))?,
        );
        industry_values.push(record[7].to_string());
        number_of_employees_values.push(
            record[8]
                .parse::<u32>()
                .map_err(|e| format!("Number of employees parsing error: {}", e))?,
        );
    }

    // Create Arrow arrays from the vectors
    let index_array = Arc::new(UInt64Array::from(index_values)) as ArrayRef;
    let org_id_array = Arc::new(StringArray::from(org_id_values)) as ArrayRef;
    let name_array = Arc::new(StringArray::from(name_values)) as ArrayRef;
    let website_array = Arc::new(StringArray::from(website_values)) as ArrayRef;
    let country_array = Arc::new(StringArray::from(country_values)) as ArrayRef;
    let description_array = Arc::new(StringArray::from(description_values)) as ArrayRef;
    let founded_array = Arc::new(UInt16Array::from(founded_values)) as ArrayRef;
    let industry_array = Arc::new(StringArray::from(industry_values)) as ArrayRef;
    let number_of_employees_array =
        Arc::new(UInt32Array::from(number_of_employees_values)) as ArrayRef;

    // Create a RecordBatch using the schema provided as an argument
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            index_array,
            org_id_array,
            name_array,
            website_array,
            country_array,
            description_array,
            founded_array,
            industry_array,
            number_of_employees_array,
        ],
    )?;

    // Write the RecordBatch to the Arrow file
    arrow_writer.write(&batch)?;
    arrow_writer.finish()?;

    println!(
        "Data converted to Arrow format and saved as '{}'",
        output_arrow
    );

    Ok(())
}
