use crate::adapters::WeatherAdapter;
use crate::types::{Coordinates, LocationType, Place};
use crate::types::{
    Temperature, Weather, WeatherForecast, WeatherState, WindDirection, WindDirectionCompass,
    WindSpeed,
};
use crate::types::{WeatherError, WeatherResult};
use chrono::{DateTime, NaiveDate, Utc};
use std::convert::TryFrom;

pub struct MetaWeatherApi {
    pub endpoint: String,
}

#[derive(Deserialize, Debug, Clone)]
struct PlaceResponse {
    title: String,
    location_type: String,
    woeid: u32,
    latt_long: String,
}

#[derive(Deserialize, Debug, Clone)]
struct WeatherResponse {
    consolidated_weather: Vec<ConsolidatedWeather>,
    title: String,
    woeid: u32,
    latt_long: String,
    timezone: String,
    timezone_name: String,
    location_type: String,
    parent: PlaceResponse,
    time: DateTime<Utc>,
    sun_rise: DateTime<Utc>,
    sun_set: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Clone)]
struct ConsolidatedWeather {
    id: u64,
    weather_state_name: String,
    weather_state_abbr: String,
    wind_direction_compass: String,
    created: DateTime<Utc>,
    applicable_date: NaiveDate,
    min_temp: f32,
    max_temp: f32,
    the_temp: f32,
    wind_speed: f32,
    wind_direction: f32,
    air_pressure: f32,
    humidity: f32,
    visibility: f32,
    predictability: i32,
}

impl WeatherAdapter for MetaWeatherApi {
    fn search_place(&self, location: String) -> WeatherResult<Vec<Place>> {
        let url = format!("{}/api/location/search/?query={}", self.endpoint, location);

        let response_body: Vec<PlaceResponse> = reqwest::get(&url)?.json()?;

        let places: WeatherResult<Vec<Place>> = response_body
            .iter()
            .map(|place_response| Place::try_from(place_response))
            .collect();

        places
    }

    fn get_weather(&self, woeid: String) -> WeatherResult<Weather> {
        let url = format!("{}/api/location/{}/", self.endpoint, woeid);

        let weather_result: WeatherResponse = reqwest::get(&url)?.json()?;

        Weather::try_from(&weather_result)
    }
}

impl TryFrom<&PlaceResponse> for Place {
    type Error = WeatherError;

    fn try_from(place_response: &PlaceResponse) -> Result<Self, Self::Error> {
        let location_type = match &place_response.location_type[..] {
            "City" => LocationType::City,
            "Region / State / Province" => LocationType::RegionOrStateOrProvince,
            "Country" => LocationType::Country,
            "Continent" => LocationType::Continent,
            _ => LocationType::NotDefined,
        };

        let latt_long: Vec<&str> = place_response.latt_long.split(",").collect();

        let coordinates = Coordinates {
            latt: latt_long[0].trim().parse::<f32>()?,
            long: latt_long[1].trim().parse::<f32>()?,
        };

        let place = Place {
            id: place_response.woeid.to_string(),
            name: place_response.title.to_owned(),
            location_type,
            coordinates,
        };

        Ok(place)
    }
}

fn consolidated_weather_to_weather_prevision(
    consolidated_weather: &ConsolidatedWeather,
) -> WeatherResult<WeatherForecast> {
    let state = match consolidated_weather.weather_state_abbr.as_ref() {
        "sn" => WeatherState::Snow,
        "sl" => WeatherState::Sleet,
        "h" => WeatherState::Hail,
        "t" => WeatherState::Thunderstorm,
        "hr" => WeatherState::HeavyRain,
        "lr" => WeatherState::LightRain,
        "s" => WeatherState::Showers,
        "hc" => WeatherState::HeavyCloud,
        "lc" => WeatherState::LightCloud,
        "c" => WeatherState::Clear,
        _ => return Err(WeatherError::ParseError),
    };

    let min = Temperature::Celsius(consolidated_weather.min_temp);
    let max = Temperature::Celsius(consolidated_weather.max_temp);
    let temp = Temperature::Celsius(consolidated_weather.the_temp);

    let wind_speed = WindSpeed::KPH(consolidated_weather.wind_speed);

    let wind_direction = WindDirection::Degrees(consolidated_weather.wind_direction);

    let wind_direction_compass = match consolidated_weather.wind_direction_compass.chars().next() {
        Some('W') => WindDirectionCompass::W,
        Some('E') => WindDirectionCompass::E,
        Some('S') => WindDirectionCompass::S,
        Some('N') => WindDirectionCompass::N,
        _ => return Err(WeatherError::ParseError),
    };

    let weather_prevision = WeatherForecast {
        state,
        created_date: consolidated_weather.created,
        applicable_date: consolidated_weather.applicable_date,
        min,
        max,
        temp,
        wind_speed,
        wind_direction_compass,
        wind_direction,
        humidity: consolidated_weather.humidity,
    };

    Ok(weather_prevision)
}

impl TryFrom<&WeatherResponse> for Weather {
    type Error = WeatherError;

    fn try_from(weather_response: &WeatherResponse) -> Result<Self, WeatherError> {
        let weather_forecasts: WeatherResult<Vec<WeatherForecast>> = weather_response
            .consolidated_weather
            .iter()
            .map(consolidated_weather_to_weather_prevision)
            .collect();

        let place_response = PlaceResponse {
            title: weather_response.title.clone(),
            location_type: weather_response.location_type.clone(),
            woeid: weather_response.woeid,
            latt_long: weather_response.latt_long.clone(),
        };

        let weather = Weather {
            place: Place::try_from(&place_response)?,
            weather_forecasts: weather_forecasts?,
            place_parent: Place::try_from(&weather_response.parent)?,
        };

        Ok(weather)
    }
}
