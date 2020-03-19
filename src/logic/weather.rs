use crate::types::weather::Weather;
use crate::types::weather::WeatherState;

impl Weather {
    pub fn state_emoji(&self) -> &'static str {
        match self.state {
            WeatherState::Snow => "❄️",
            WeatherState::Sleet | WeatherState::Hail => "⛆",
            WeatherState::Thunderstorm => "⛈️",
            WeatherState::HeavyRain => "🌧️",
            WeatherState::LightRain | WeatherState::Showers => "🌦",
            WeatherState::HeavyCloud => "☁️",
            WeatherState::LightCloud => "🌥️",
            WeatherState::Clear => "☀️",
        }
    }
}
