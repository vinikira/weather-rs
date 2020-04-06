mod place;
mod weather;

pub use place::*;
pub use weather::*;

#[derive(Debug)]
pub enum WeatherError {
    ParseError,
    AdapterError,
    MissingArgument(&'static str),
}

pub type WeatherResult<T> = Result<T, WeatherError>;
