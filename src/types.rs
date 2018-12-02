use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug)]
pub struct ConsolidatedWeather {
    pub id: u64,
    pub weather_state_name: String,
    pub weather_state_abbr: String,
    pub wind_direction_compass: String,
    pub created: DateTime<Utc>,
    pub applicable_date: String,
    pub min_temp: f32,
    pub max_temp: f32,
    pub the_temp: f32,
    pub wind_speed: f32,
    pub wind_direction: f32,
    pub air_pressure: f32,
    pub humidity: i32,
    pub visibility: f32,
    pub predictability: i32,
}

impl ConsolidatedWeather {
    pub fn temp_emoji(&self) -> &'static str {
        let state: &str = &self.weather_state_abbr[..];

        match state {
            "sn" => "❄️",
            "sl" | "h" => "⛆",
            "t" => "⛈️",
            "hr" => "🌧️",
            "lr" | "s" => "🌦",
            "hc" => "☁️",
            "lc" => "🌥️",
            "c" => "☀️",
            _ => "",
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Parent {
    pub title: String,
    pub location_type: String,
    pub woeid: u32,
    pub latt_long: String,
}

#[derive(Deserialize, Debug)]
pub struct ListWeatherInfo {
    pub consolidated_weather: Vec<ConsolidatedWeather>,
    pub title: String,
    pub woeid: u32,
    pub latt_long: String,
    pub timezone: String,
    pub timezone_name: String,
    pub location_type: String,
    pub parent: Parent,
    pub time: DateTime<Utc>,
    pub sun_rise: DateTime<Utc>,
    pub sun_set: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct City {
    pub title: String,
    pub location_type: String,
    pub woeid: u32,
    pub latt_long: String,
}
