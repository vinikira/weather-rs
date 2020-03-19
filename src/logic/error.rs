use crate::types::WeatherError;
use std::error;
use std::fmt;

impl fmt::Display for WeatherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid weather")
    }
}

impl error::Error for WeatherError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
