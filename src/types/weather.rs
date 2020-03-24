use crate::types::place::Place;
use chrono::{DateTime, NaiveDate, Utc};

pub struct Weather {
    pub place: Place,
    pub place_parent: Place,
    pub weather_previsions: Vec<WeatherPrevision>,
}

pub struct WeatherPrevision {
    pub state: WeatherState,
    pub created_date: DateTime<Utc>,
    pub applicable_date: NaiveDate,
    pub min: Temperature,
    pub max: Temperature,
    pub wind_speed: WindSpeed,
    pub wind_direction: WindDirection,
    pub wind_direction_compass: WindDirectionCompass,
    pub humidity: Humidity,
}

pub type Humidity = f32;

pub enum Temperature {
    Celsius(f32),
    Farenheit(f32),
    Kelvin(f32),
}

pub enum WindSpeed {
    MPH(f32),
    KPH(f32),
}

pub enum WindDirection {
    Degrees(f32),
}

pub enum WindDirectionCompass {
    N,
    E,
    S,
    W,
    NotDefined,
}

pub enum WeatherState {
    Clear,
    Hail,
    HeavyCloud,
    HeavyRain,
    LightCloud,
    LightRain,
    Showers,
    Sleet,
    Snow,
    Thunderstorm,
    NotDefined,
}
