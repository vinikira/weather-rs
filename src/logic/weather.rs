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
        use WeatherState::*;

        match self.state {
            Some(Snow) => "❄️",
            Some(Sleet) | Some(Hail) => "⛆",
            Some(Thunderstorm) => "⛈️",
            Some(HeavyRain) => "🌧️",
            Some(LightRain) | Some(Showers) => "🌦",
            Some(HeavyCloud) => "☁️",
            Some(LightCloud) => "🌥️",
            Some(Clear) => "☀️",
            None => "",
        }
    }

    pub fn state_name(&self) -> &'static str {
        use WeatherState::*;

        match self.state {
            Some(Snow) => "Snow️",
            Some(Sleet) => "Sleet",
            Some(Hail) => "Hail",
            Some(Thunderstorm) => "Thunderstorm",
            Some(HeavyRain) => "Heavy Rain",
            Some(LightRain) => "Light Rain",
            Some(Showers) => "Showers",
            Some(HeavyCloud) => "Heavy Cloud",
            Some(LightCloud) => "Light Cloud",
            Some(Clear) => "Clear️",
            None => "",
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

    pub fn to_celsius(&self) -> Self {
        match *self {
            Self::Farenheit(temp) => Self::Celsius((temp - 32f32) * (5f32 / 9f32)),
            Self::Kelvin(temp) => Self::Celsius(temp - 273.15f32),
            Self::Celsius(temp) => Self::Celsius(temp),
        }
    }

    pub fn to_farenheit(&self) -> Self {
        match *self {
            Self::Celsius(temp) => Self::Farenheit((temp * (9f32 / 5f32)) + 32f32),
            Self::Kelvin(temp) => Self::Farenheit((temp - 273.15f32) * (9f32 / 5f32) + 32f32),
            Self::Farenheit(temp) => Self::Farenheit(temp),
        }
    }

    pub fn to_kelvin(&self) -> Self {
        match *self {
            Self::Celsius(temp) => Self::Kelvin(temp + 273.15f32),
            Self::Farenheit(temp) => Self::Kelvin((temp - 32f32) * (5f32 / 9f32) + 273.15f32),
            Self::Kelvin(temp) => Self::Kelvin(temp),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_c_to_others() {
        let celsius = Temperature::Celsius(27f32);

        assert_eq!(celsius.to_farenheit(), Temperature::Farenheit(80.6f32));
        assert_eq!(celsius.to_kelvin(), Temperature::Kelvin(300.15f32));
        assert_eq!(celsius.to_celsius(), celsius);
    }

    #[test]
    fn should_convert_f_to_others() {
        let farenheit = Temperature::Farenheit(80.6f32);

        assert_eq!(farenheit.to_farenheit(), farenheit);
        assert_eq!(farenheit.to_kelvin(), Temperature::Kelvin(300.15f32));
        assert_eq!(farenheit.to_celsius(), Temperature::Celsius(27f32));
    }

    #[test]
    fn should_convert_k_to_others() {
        let kelvin = Temperature::Kelvin(300.15f32);

        assert_eq!(kelvin.to_kelvin(), kelvin);
        assert_eq!(kelvin.to_farenheit(), Temperature::Farenheit(80.6f32));
        assert_eq!(kelvin.to_celsius(), Temperature::Celsius(27f32));
    }
}
