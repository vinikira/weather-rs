pub mod place;
pub mod weather;

#[derive(Debug)]
pub enum WeatherError {
    ParseError,
    AdapterError,
    MissingArgument,
}

pub type WeatherResult<T> = Result<T, WeatherError>;
