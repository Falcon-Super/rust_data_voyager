use rust_data_voyager::{data_analysis, data_cleaning, data_validation, read_csv};
use std::error::Error;
use std::path::Path;

const INPUT_CSV_PATH: &str = "Datasets/csv-files/organization-100.csv";
const CLEANED_CSV_PATH: &str = "Datasets/csv-files/cleaned_test.csv"; // Define the path for cleaned data
const OUTPUT_ARROW_PATH: &str = "Datasets/arrow-files/output.arrow";

fn main() -> Result<(), Box<dyn Error>> {
    // Read and print the CSV data first
    if Path::new(INPUT_CSV_PATH).exists() {
        println!("Reading CSV data from {}", INPUT_CSV_PATH);
        read_csv::read_csv(INPUT_CSV_PATH)?;
    } else {
        return Err(From::from(format!("CSV file not found: {}", INPUT_CSV_PATH)));
    }
    // Clean the data first and get the path to the cleaned data
    let cleaned_csv_path = data_cleaning::clean_csv_data(INPUT_CSV_PATH, CLEANED_CSV_PATH)?;

    // Now validate and convert the cleaned data
    data_validation::validate_and_convert_data(&cleaned_csv_path, OUTPUT_ARROW_PATH)?;

    // Perform data analysis on the validated Arrow file
    data_analysis::perform_data_analysis(OUTPUT_ARROW_PATH)?;

    Ok(())
}
