use crate::adapters::WeatherAdapter;
use crate::types::place::{Coordinates, LocationType, Place};
use crate::types::weather::Weather;
use crate::types::WeatherError;

struct MetaWeatherApi {
    endpoint: String,
}

#[derive(Deserialize, Debug)]
struct PlaceResponse {
    title: String,
    location_type: String,
    woeid: u32,
    latt_long: String,
}

impl WeatherAdapter for MetaWeatherApi {
    fn search_place(&self, location: String) -> Result<Vec<Place>, WeatherError> {
        let url: String = format!("{}/location/search/?query={}", self.endpoint, location);

        let response_body: reqwest::Result<Vec<PlaceResponse>> = match reqwest::get(&url) {
            Ok(mut res) => res.json(),

            Err(_) => return Err(WeatherError {}),
        };

        if let Ok(place_result) = response_body {
            let places = place_result
                .iter()
                .map(|place_response| Place {
                    name: place_response.title.to_owned(),
                    location_type: LocationType::from(place_response),
                    coordinates: Coordinates::from(place_response),
                })
                .collect();

            Ok(places)
        } else {
            Err(WeatherError {})
        }
    }

    fn get_weather(&self, woeid: String) -> Result<Weather, WeatherError> {
        unimplemented!()
    }
}

impl From<&PlaceResponse> for LocationType {
    fn from(place_response: &PlaceResponse) -> Self {
        match &place_response.location_type[..] {
            "City" => LocationType::City,
            "Region / State / Province" => LocationType::RegionOrStateOrProvince,
            "Country" => LocationType::Country,
            "Continent" => LocationType::Continent,
            _ => LocationType::RegionOrStateOrProvince,
        }
    }
}

impl From<&PlaceResponse> for Coordinates {
    fn from(place_response: &PlaceResponse) -> Self {
        let mut lat_long: Vec<f32> = place_response
            .latt_long
            .split(",")
            .map(|unit| unit.parse::<f32>().unwrap_or(0f32))
            .collect();

        Coordinates {
            latt: lat_long.pop().unwrap_or(0f32),
            long: lat_long.pop().unwrap_or(0f32),
        }
    }
}
