use crate::api::{get_weather, search_city};
use colored::*;

pub fn search(city_name: &str) -> String {
    println!("{}", String::from("Search Results:"));
    search_city(city_name)
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
        .unwrap()
}

pub fn get(woeid: &str) -> String {
    get_weather(woeid)
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
        .unwrap()
}
