use crate::types::WeatherError;
use std::error;
use std::fmt;
use std::num::ParseFloatError;

impl fmt::Display for WeatherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Self::ParseError => "Parser Error",
            Self::AdapterError => "Adapter Error",
            Self::MissingArgument => "Missing Argument",
        };

        write!(f, "Error: {}", msg)
    }
}

impl error::Error for WeatherError {}

impl From<ParseFloatError> for WeatherError {
    fn from(_error: ParseFloatError) -> Self {
        Self::ParseError
    }
}

impl From<reqwest::Error> for WeatherError {
    fn from(error: reqwest::Error) -> Self {
        println!("{}", error);
        WeatherError::AdapterError
    }
}
