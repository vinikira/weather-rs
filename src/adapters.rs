use crate::types::place::Place;
use crate::types::weather::Weather;
use crate::types::WeatherError;

pub mod metaweather;

pub trait WeatherAdapter {
    fn search_place(&self, search_args: String) -> Result<Vec<Place>, WeatherError>;

    fn get_weather(&self, weather_param: String) -> Result<Weather, WeatherError>;
}
