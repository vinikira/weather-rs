use crate::types::weather::Weather;
use crate::types::WeatherError;

pub trait SearchWeather {
    type SearchArgs;

    fn search(searchArgs: Self::SearchArgs) -> Result<Vec<Weather>, WeatherError>;
}
