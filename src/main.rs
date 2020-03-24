use clap::ArgMatches;
use clap::{App, Arg, SubCommand};
use std::collections::HashMap;
use weather_rs::adapters::{metaweather, WeatherAdapter};
use weather_rs::types::{WeatherError, WeatherResult};

fn main() {
    let providers = init_providers();
    let default_provider = providers
        .get("meta-weather")
        .expect("Error to get default provider.");

    let matches = App::new("weather")
        .version("2.0.0")
        .about("Get weather prevision")
        .author("Vinícius Simões")
        .arg(
            Arg::with_name("provider")
                .short("p")
                .long("provider")
                .value_name("PROVIDER_NAME")
                .help("Choose weather forecast provider.")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("search")
                .about("Search place by name.")
                .arg(Arg::with_name("place_name").help("Place name.")),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get weather by place id.")
                .arg(Arg::with_name("place_id").help("Place id.")),
        )
        .get_matches();

    let provider = match matches.value_of("provider") {
        Some(provider_name) => providers.get(provider_name).unwrap_or(default_provider),
        None => default_provider,
    };

    if let Some(search_matches) = matches.subcommand_matches("search") {
        match search_location(search_matches, provider) {
            Ok(status) => std::process::exit(status),
            Err(error) => {
                handle_error(error);
                std::process::exit(1);
            }
        }
    }

    if let Some(get_matches) = matches.subcommand_matches("get") {
        match fetch_weather(get_matches, provider) {
            Ok(status) => std::process::exit(status),
            Err(error) => {
                handle_error(error);
                std::process::exit(1);
            }
        }
    }
}

fn init_providers() -> HashMap<&'static str, impl WeatherAdapter> {
    let mut providers = HashMap::new();

    providers.insert(
        "meta-weather",
        metaweather::MetaWeatherApi {
            endpoint: "https://www.metaweather.com".to_string(),
        },
    );

    providers
}

fn handle_error(e: WeatherError) {
    println!("{}", e);
}

fn search_location(search_args: &ArgMatches, adapter: &impl WeatherAdapter) -> WeatherResult<i32> {
    let city_name = match search_args.value_of("place_name") {
        Some(name) => name,
        None => return Err(WeatherError::MissingArgument),
    };

    let search_result: String = adapter
        .search_place(city_name.to_string())?
        .iter()
        .map(|place| place.pretty())
        .collect::<Vec<String>>()
        .join("");

    println!("Search Results:\n\n{}", search_result);

    Ok(0)
}

fn fetch_weather(get_matches: &ArgMatches, adapter: &impl WeatherAdapter) -> WeatherResult<i32> {
    let id = match get_matches.value_of("place_id") {
        Some(id) => id,
        None => return Err(WeatherError::MissingArgument),
    };

    let weather = adapter.get_weather(id.to_string())?;

    println!("{}", weather.pretty());

    Ok(0)
}
