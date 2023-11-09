// read_csv.rs
use csv::Reader;
use std::error::Error;

pub fn read_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path(file_path)?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
