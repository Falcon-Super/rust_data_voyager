//read_csv.rs
use csv::Reader;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn read_csvs_in_directory(directory: &str) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("csv") {
            println!("Reading file: {:?}", path.display());
            read_csv(&path)?;
            println!("Finished reading file: {:?}", path.display());
        }
    }

    Ok(())
}

fn read_csv<P: AsRef<Path>>(file_path: P) -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path(file_path)?;

    // Optionally, print headers
    if let Some(headers) = rdr.headers().ok() {
        println!("Headers: {:?}", headers);
    }

    // Print only the first 10 records for preview
    for (i, result) in rdr.records().enumerate() {
        let record = result?;
        if i < 10 {
            println!("Record {}: {:?}", i + 1, record);
        } else {
            break;
        }
    }

    Ok(())
}

// Add more helper functions if needed
