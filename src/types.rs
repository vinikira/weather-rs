pub mod place;
pub mod weather;

#[derive(Debug)]
pub enum WeatherError {
    ParseError,
    AdapterError,
    MissingArgument(&'static str),
}

pub type WeatherResult<T> = Result<T, WeatherError>;
