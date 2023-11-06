use std::error::Error;
use rust_data_voyager::{data_analysis, data_validation};

fn main() -> Result<(), Box<dyn Error>> {
    // This function will validate and convert the CSV data to Arrow format.
    // If there is an error during this process, it will be propagated with `?` 
    // and the program will return early with that error.
    let input_csv = "Datasets/iris_utf8.csv"; // Specify the path to your input CSV file
    let output_arrow = "output.arrow";
    data_validation::validate_and_convert_data(input_csv, output_arrow)?;

    // Here we call the validate_arrow_output function and immediately handle the result
    // with a match expression. If the result is Ok, we proceed with data analysis.
    // If there is an error (the Err variant), we print a message and return the error.
    match data_validation::validate_arrow_output("iris.arrow") {
        Ok(_) => {
            // Assuming the perform_data_analysis function is implemented correctly,
            // we call it and propagate any errors with `?`.
            data_analysis::perform_data_analysis()?;
        },
        Err(e) => {
            // If the Arrow file validation failed, we print an error message
            // and return the error to exit the program.
            eprintln!("Arrow file validation failed: {}", e);
            return Err(e);
        }
    }

    // If everything went well, we return Ok to indicate the program completed successfully.
    Ok(())
}
