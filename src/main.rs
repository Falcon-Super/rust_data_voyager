use std::error::Error;
use rust_data_voyager::{data_analysis, data_validation, data_cleaning};

const INPUT_CSV_PATH: &str = "Datasets/csv-files/test.csv";
const CLEANED_CSV_PATH: &str = "Datasets/csv-files/cleaned_test.csv"; // Define the path for cleaned data
const OUTPUT_ARROW_PATH: &str = "Datasets/arrow-files/output.arrow";
// const EXPECTED_COLUMN_COUNT: usize = 9;

fn main() -> Result<(), Box<dyn Error>> {
    // Clean the data first and get the path to the cleaned data
    let cleaned_csv_path = data_cleaning::clean_csv_data(INPUT_CSV_PATH, CLEANED_CSV_PATH)?;

    // Now validate and convert the cleaned data
    data_validation::validate_and_convert_data(&cleaned_csv_path, OUTPUT_ARROW_PATH)?;

    // Validate the output Arrow file
    match data_validation::validate_arrow_output(OUTPUT_ARROW_PATH) {
        Ok(_) => {
            // Perform data analysis on the validated Arrow file
            data_analysis::perform_data_analysis(OUTPUT_ARROW_PATH)?;
        },
        Err(e) => {
            eprintln!("Arrow file validation failed: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
