use crate::types::place::Place;
use crate::types::weather::Weather;
use crate::types::WeatherError;

mod metaweather;

pub trait WeatherAdapter {
    fn search_place(&self, searchArgs: String) -> Result<Vec<Place>, WeatherError>;

    fn get_weather(&self, weatherParam: String) -> Result<Weather, WeatherError>;
}
