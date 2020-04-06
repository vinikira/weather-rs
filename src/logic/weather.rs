use crate::types::{Temperature, Weather, WeatherForecast, WeatherState};

impl std::fmt::Display for Weather {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use colored::*;

        let place_name = format!("{}", self.place.to_string());

        let table_header = format!(
            "{: ^14} | {: ^10} | {: ^10} | {: ^10} | {: ^10}",
            "State".bold(),
            "Date".bold(),
            "Temp".bold(),
            "Min".bold(),
            "Max".bold()
        );

        let header = format!(
            "{place}\n\n{table_header}\n{dots}\n",
            place = place_name,
            table_header = table_header,
            dots = "-".repeat(66)
        );

        let weather_str = self
            .weather_forecasts
            .iter()
            .fold(header, |text, weather_forecast| {
                format!(
                    "{}{weather_forecast}\n",
                    text,
                    weather_forecast = weather_forecast.to_string()
                )
            });

        write!(f, "{}", weather_str)
    }
}

impl WeatherForecast {
    pub fn state_name(&self) -> &'static str {
        use WeatherState::*;

        match self.state {
            Snow => "Snow",
            Sleet => "Sleet",
            Hail => "Hail",
            Thunderstorm => "Thunderstorm",
            HeavyRain => "Heavy Rain",
            LightRain => "Light Rain",
            Showers => "Showers",
            HeavyCloud => "Heavy Cloud",
            LightCloud => "Light Cloud",
            Clear => "Clear",
        }
    }
}

impl std::fmt::Display for WeatherForecast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use colored::*;

        write!(
            f,
            "{state_name: <14} | {date: <10} | {temp: <10} | {min: <10} | {max: <10}",
            state_name = self.state_name().green().bold(),
            date = self.applicable_date,
            temp = format!("{}", self.temp.to_string()),
            min = format!("{}", self.min.to_string()),
            max = format!("{}", self.max.to_string())
        )
    }
}

impl Temperature {
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

impl std::fmt::Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Celsius(val) => write!(f, "{:.2} °C", val),
            Self::Farenheit(val) => write!(f, "{:.2} °F", val),
            Self::Kelvin(val) => write!(f, "{:.2} K", val),
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
