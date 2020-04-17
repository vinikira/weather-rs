mod error;
mod place;
mod weather;

pub use error::*;
pub use place::*;
pub use weather::*;

pub type WeatherResult<T> = Result<T, WeatherError>;
