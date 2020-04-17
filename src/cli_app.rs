use crate::adapters::{metaweather, WeatherAdapter};
use crate::domain::WeatherError;
use clap::ArgMatches;

pub struct CLIApp<'a> {
    provider: &'a str,
    providers: Providers,
    matches: ArgMatches<'a>,
}

struct Providers {
    meta_weather: metaweather::MetaWeatherApi,
}

impl<'a> CLIApp<'a> {
    pub fn new(provider: &'a str, matches: ArgMatches<'a>) -> Self {
        let providers = Providers {
            meta_weather: metaweather::MetaWeatherApi {
                endpoint: "https://www.metaweather.com".to_string(),
            },
        };

        Self {
            provider,
            matches,
            providers,
        }
    }

    pub fn perform(&self) {
        if let Some(search_matches) = self.matches.subcommand_matches("search") {
            self.search_location(search_matches);
        }

        if let Some(get_matches) = self.matches.subcommand_matches("get") {
            self.fetch_weather(get_matches)
        }
    }

    fn get_provider(&self) -> &impl WeatherAdapter {
        match self.provider {
            _ => &self.providers.meta_weather,
        }
    }

    fn search_location(&self, search_args: &ArgMatches) {
        let print_json = search_args.is_present("json");
        let city_name = match search_args.value_of("place_name") {
            Some(name) => name,
            None => {
                return Self::handle_error(WeatherError::MissingArgument("place name"));
            }
        };

        let search_results = match self.get_provider().search_place(city_name.to_string()) {
            Ok(results) => results,
            Err(error) => return Self::handle_error(error),
        };

        let search_message = if print_json {
            serde_json::to_string_pretty(&search_results).unwrap_or("".to_string())
        } else {
            search_results
                .iter()
                .map(|place| place.to_string())
                .collect::<Vec<String>>()
                .join("")
        };

        println!("{}", search_message);
    }

    fn fetch_weather(&self, get_matches: &ArgMatches) {
        let print_json = get_matches.is_present("json");
        let id = match get_matches.value_of("place_id") {
            Some(id) => id,
            None => {
                return Self::handle_error(WeatherError::MissingArgument("place id"));
            }
        };

        let weather_forecast_result = match self.get_provider().get_weather(id.to_string()) {
            Ok(weather_forecast) => weather_forecast,
            Err(error) => return Self::handle_error(error),
        };

        let weather_forecast_message = if print_json {
            serde_json::to_string_pretty(&weather_forecast_result).unwrap_or("".to_string())
        } else {
            weather_forecast_result.to_string()
        };

        println!("{}", weather_forecast_message);
    }

    fn handle_error(e: WeatherError) {
        println!("{}", e);
    }
}
