//main.rs
use rust_data_voyager::{data_analysis, data_cleaning, data_validation, read_csv};
use std::error::Error;
use std::fs;

const INPUT_CSV_DIR: &str = "Datasets/csv-files";
const CLEANED_CSV_DIR: &str = "Datasets/csv-files/cleaned";
const OUTPUT_ARROW_DIR: &str = "Datasets/arrow-files";

fn main() -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(CLEANED_CSV_DIR)?;
    fs::create_dir_all(OUTPUT_ARROW_DIR)?;

    for entry in fs::read_dir(INPUT_CSV_DIR)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                if extension == "csv" {
                    let file_stem = path.file_stem().unwrap().to_str().unwrap();
                    let input_csv_path = path.to_str().unwrap();

                    println!("Processing CSV file: {}", file_stem);

                    // Reading CSV
                    println!("Reading CSV data from {}", input_csv_path);
                    read_csv::read_csvs_in_directory(INPUT_CSV_DIR)?;

                    // Cleaning CSV
                    // let cleaned_csv_path = format!("{}/{}_cleaned.csv", CLEANED_CSV_DIR, file_stem);
                    // data_cleaning::clean_csv_data(input_csv_path, &cleaned_csv_path)?;
                    // let cleaned_csv_dir = CLEANED_CSV_DIR;
                    data_cleaning::clean_csv_data(INPUT_CSV_DIR, CLEANED_CSV_DIR)?;

                    // Converting CSV to Arrow
                    let output_arrow_path = format!("{}/{}.arrow", OUTPUT_ARROW_DIR, file_stem);
                    data_validation::validate_csv_input(&CLEANED_CSV_DIR)?;

                    // Data Analysis
                    data_analysis::perform_data_analysis(&output_arrow_path)?;
                }
            }
        }
    }

    Ok(())
}
