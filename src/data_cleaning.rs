// data_cleaning.rs
use csv::{ErrorKind, ReaderBuilder, StringRecord, WriterBuilder};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::string::FromUtf8Error;

pub fn clean_csv_data(input_csv_dir: &str, output_csv_dir: &str) -> Result<String, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().flexible(true).from_path(input_csv_dir)?;

    let mut wtr = WriterBuilder::new().from_path(output_csv_dir)?;

    for result in rdr.byte_records() {
        match result {
            Ok(byte_record) => {
                let record = byte_record_to_string_record(byte_record)?;
                let cleaned_record = clean_record(&record)?;
                wtr.write_record(&cleaned_record)?;
            }
            Err(e) => {
                if let ErrorKind::UnequalLengths { .. } = e.kind() {
                    // Pass both directories to read_csv_skip_lines
                    return read_csv_skip_lines(input_csv_dir, output_csv_dir);
                } else {
                    // For other types of errors, return the error
                    return Err(Box::new(e));
                }
            }
        }
    }

    wtr.flush()?;
    Ok(output_csv_dir.to_string())
}

// fn process_csv(input_csv: &str, output_csv: &str) -> Result<String, Box<dyn Error>> {
//     let mut rdr = ReaderBuilder::new().flexible(true).from_path(input_csv)?;
//     let mut wtr = WriterBuilder::new().from_path(output_csv)?;

//     for result in rdr.byte_records() {
//         let byte_record = result?;
//         let record = byte_record_to_string_record(byte_record)?;
//         let cleaned_record = clean_record(&record)?;

//         wtr.write_record(&cleaned_record)?;
//     }

//     wtr.flush()?;
//     Ok(output_csv.to_string())
// }

fn byte_record_to_string_record(
    byte_record: csv::ByteRecord,
) -> Result<StringRecord, FromUtf8Error> {
    let mut fields = Vec::new();
    for field in byte_record.iter() {
        let field_str = String::from_utf8_lossy(field);
        fields.push(field_str.into_owned());
    }
    Ok(StringRecord::from(fields))
}

fn clean_record(record: &StringRecord) -> Result<StringRecord, Box<dyn Error>> {
    // Implement specific cleaning logic here
    // For example, trimming whitespace, replacing invalid characters, etc.
    Ok(record.clone())
}

fn read_csv_skip_lines(input_csv: &str, output_csv_dir: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(input_csv)?;
    let reader = BufReader::new(file);

    // Read all lines into a vector
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    // Skip the first 3 lines and the last 6 lines
    let data_lines = &lines[3..lines.len().saturating_sub(6)];

    // Generate output file path
    let output_file_path =
        Path::new(output_csv_dir).join(Path::new(input_csv).file_name().unwrap());
    let output_csv_path = output_file_path
        .to_str()
        .ok_or("Error converting path to string")?;

    // Create writer for the cleaned CSV
    let mut wtr = WriterBuilder::new().from_path(output_csv_path)?;

    // Now read the actual data using the CSV reader
    let joined_data = data_lines.join("\n");
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(joined_data.as_bytes());

    // Process each record and write to the new file
    for result in csv_reader.records() {
        let record = result?;
        wtr.write_record(&record)?;
    }

    wtr.flush()?;
    Ok(output_csv_path.to_string())
}
