use std::error;
use std::fmt;
use std::num::ParseFloatError;

#[derive(Debug)]
pub enum WeatherError {
    ParseError,
    AdapterError,
    MissingArgument(&'static str),
}

impl fmt::Display for WeatherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Self::ParseError => "Parser Error".to_string(),
            Self::AdapterError => "Adapter Error".to_string(),
            Self::MissingArgument(argument_name) => format!("Missing argument {}", argument_name),
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
    fn from(_error: reqwest::Error) -> Self {
        WeatherError::AdapterError
    }
}
