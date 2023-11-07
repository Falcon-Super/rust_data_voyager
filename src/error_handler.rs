// error_handler.rs

use std::fmt;
use csv::Error as CsvError;
use arrow::error::ArrowError;
use std::io;

#[derive(Debug)]
pub enum DataError {
    Csv(CsvError),
    Io(io::Error),
    Arrow(ArrowError),
    ParseFloat(std::num::ParseFloatError),
    // You can add more error types as needed for your application
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataError::Csv(e) => write!(f, "CSV error: {}", e),
            DataError::Io(e) => write!(f, "IO error: {}", e),
            DataError::Arrow(e) => write!(f, "Arrow error: {}", e),
            DataError::ParseFloat(e) => write!(f, "Parse float error: {}", e),
        }
    }
}

impl std::error::Error for DataError {}

// Implement From trait to convert from underlying errors to our custom error
impl From<CsvError> for DataError {
    fn from(err: CsvError) -> DataError {
        DataError::Csv(err)
    }
}

impl From<io::Error> for DataError {
    fn from(err: io::Error) -> DataError {
        DataError::Io(err)
    }
}

impl From<ArrowError> for DataError {
    fn from(err: ArrowError) -> DataError {
        DataError::Arrow(err)
    }
}

impl From<std::num::ParseFloatError> for DataError {
    fn from(err: std::num::ParseFloatError) -> DataError {
        DataError::ParseFloat(err)
    }
}

// Here you can add more `From` implementations for other error types as needed
