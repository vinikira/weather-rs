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
    fn from(error: ParseFloatError) -> Self {
        dbg!(error);
        Self::ParseError
    }
}

impl From<reqwest::Error> for WeatherError {
    fn from(_error: reqwest::Error) -> Self {
        WeatherError::AdapterError
    }
}
