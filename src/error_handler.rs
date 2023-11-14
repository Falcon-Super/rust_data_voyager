//error_handler.rs
use arrow::error::ArrowError;
use csv::Error as CsvError;
use std::error::Error;
use std::fmt;
use std::io;
use std::num::{ParseFloatError, ParseIntError};
use std::str::Utf8Error;

#[derive(Debug)]
pub enum DataError {
    Csv(CsvError),
    Io(io::Error),
    Arrow(ArrowError),
    ParseFloat(ParseFloatError),
    ParseInt(ParseIntError),
    Utf8(Utf8Error),
    Other(String), // Catch-all for other error types
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataError::Csv(e) => write!(f, "CSV error: {}", e),
            DataError::Io(e) => write!(f, "IO error: {}", e),
            DataError::Arrow(e) => write!(f, "Arrow error: {}", e),
            DataError::ParseFloat(e) => write!(f, "Parse float error: {}", e),
            DataError::ParseInt(e) => write!(f, "Parse integer error: {}", e),
            DataError::Utf8(e) => write!(f, "UTF-8 decoding error: {}", e),
            DataError::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

impl Error for DataError {}

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

impl From<ParseFloatError> for DataError {
    fn from(err: ParseFloatError) -> DataError {
        DataError::ParseFloat(err)
    }
}

impl From<ParseIntError> for DataError {
    fn from(err: ParseIntError) -> DataError {
        DataError::ParseInt(err)
    }
}

impl From<Utf8Error> for DataError {
    fn from(err: Utf8Error) -> DataError {
        DataError::Utf8(err)
    }
}

impl From<Box<dyn Error>> for DataError {
    fn from(err: Box<dyn Error>) -> DataError {
        DataError::Other(format!("{}", err))
    }
}

impl From<&'static str> for DataError {
    fn from(err: &'static str) -> DataError {
        DataError::Other(err.to_string())
    }
}

// Here you can add more `From` implementations for other error types as needed
