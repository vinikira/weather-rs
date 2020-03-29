use crate::types::weather::{Temperature, Weather, WeatherForecast, WeatherState};

impl Weather {
    pub fn pretty(&self) -> String {
        let header = format!("{}", self.place.pretty());

        self.weather_forecasts
            .iter()
            .fold(header, |text, weather_forecast| {
                format!(
                    "{}{weather_forecast}\n",
                    text,
                    weather_forecast = weather_forecast.pretty()
                )
            })
    }
}

impl WeatherForecast {
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
            WeatherState::NotDefined => "",
        }
    }

    pub fn state_name(&self) -> &'static str {
        match self.state {
            WeatherState::Snow => "Snow️",
            WeatherState::Sleet => "Sleet",
            WeatherState::Hail => "Hail",
            WeatherState::Thunderstorm => "Thunderstorm",
            WeatherState::HeavyRain => "Heavy Rain",
            WeatherState::LightRain => "Light Rain",
            WeatherState::Showers => "Showers",
            WeatherState::HeavyCloud => "Heavy Cloud",
            WeatherState::LightCloud => "Light Cloud",
            WeatherState::Clear => "Clear️",
            WeatherState::NotDefined => "",
        }
    }

    pub fn pretty(&self) -> String {
        use colored::*;
        format!(
            "{emoji}    {state_name:<12} - {date} - temp. {temp} min. {min} max. {max}\n",
            emoji = self.state_emoji().green().bold(),
            state_name = self.state_name().green().bold(),
            date = self.applicable_date,
            temp = format!("{}", self.temp.pretty()).bold(),
            min = format!("{}", self.min.pretty()).bold(),
            max = format!("{}", self.max.pretty()).bold()
        )
    }
}

impl Temperature {
    pub fn pretty(&self) -> String {
        match self {
            Self::Celsius(val) => format!("{:.2} °C", val),
            Self::Farenheit(val) => format!("{:.2} °F", val),
            Self::Kelvin(val) => format!("{:.2} K", val),
        }
    }
}
