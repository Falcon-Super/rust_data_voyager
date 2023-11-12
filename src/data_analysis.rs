//data_analysis.rs
use arrow::ipc::reader::FileReader;
use arrow::array::{StringArray, UInt32Array, UInt16Array};
use std::error::Error;
use std::fs::File;
use std::collections::HashMap;


pub fn perform_data_analysis(input_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Starting data analysis for file: {}", input_file);

    // Open the Arrow file
    let file = File::open(input_file)?;
    println!("File opened successfully.");

    // Create an Arrow file reader
    let reader = FileReader::try_new(file, None)?;

    let mut total_employees = 0;
    let mut total_orgs = 0;
    let mut country_counts: HashMap<String, u32> = HashMap::new();
    let mut total_years = 0;
    let mut total_years_count = 0;

    for maybe_batch in reader {
        let batch = maybe_batch?;
        println!("Processing record batch.");

        let employees_column = batch.column(8).as_any().downcast_ref::<UInt32Array>().unwrap();
        let country_column = batch.column(4).as_any().downcast_ref::<StringArray>().unwrap();
        let founded_column = batch.column(6).as_any().downcast_ref::<UInt16Array>().unwrap();

        // Accumulate totals for employees and years
        total_employees += employees_column.iter().filter_map(|x| x).sum::<u32>();
        total_orgs += employees_column.len();

        // Count organizations per country
        for country in country_column.iter() {
            if let Some(country_name) = country {
                *country_counts.entry(country_name.to_string()).or_insert(0) += 1;
            }
        }

        // Accumulate total founded years
        for year in founded_column.iter() {
            if let Some(year) = year {
                total_years += year as u32;
                total_years_count += 1;
            }
        }
    }

    let average_employees = total_employees as f64 / total_orgs as f64;
    let average_founded_year = total_years as f64 / total_years_count as f64;

    println!("Average number of employees: {}", average_employees);
    println!("Organizations per country:");
    for (country, count) in country_counts {
        println!("{}: {}", country, count);
    }
    println!("Average founding year: {}", average_founded_year);

    Ok(())
}