use crate::api::{get_weather, search_city};
use clap::ArgMatches;
use colored::*;

pub fn search(search_args: &ArgMatches) -> () {
    let city_name: &str = search_args.value_of("city_name").unwrap_or("");

    println!("{}", String::from("Search Results:"));

    let search_result = search_city(city_name)
        .map(|results| {
            results.iter().fold("".to_string(), |text, result| {
                format!(
                    "{}\n{id} - {name} ({latt_long})",
                    text,
                    id = result.woeid.to_string().green().bold(),
                    name = result.title.bold(),
                    latt_long = result.latt_long
                )
            })
        })
        .map_err(|err| format!("Error - {}", err))
        .unwrap();

    println!("{}", search_result);
}

pub fn get(get_matches: &ArgMatches) -> () {
    let woeid = get_matches.value_of("city_woid").unwrap_or("");

    let get_result = get_weather(woeid)
        .map(|result| {
            let header = format!(
                "{id} - {city_name}\n",
                id = result.woeid,
                city_name = result.title.bold()
            );

            result
                .consolidated_weather
                .iter()
                .fold(header, |text, weather| {
                    format!(
                        "{}{emoji} {state_name:<12} - {date} - min. {min:.2} max. {max:.2}\n",
                        text,
                        emoji = weather.temp_emoji().green().bold(),
                        state_name = weather.weather_state_name.green().bold(),
                        date = weather.applicable_date,
                        min = format!("{:.2}", weather.min_temp).bold(),
                        max = format!("{:.2}", weather.max_temp).bold()
                    )
                })
        })
        .map_err(|err| format!("Error - {}", err))
        .unwrap();

    println!("{}", get_result);
}
