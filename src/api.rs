use crate::types::{City, ListWeatherInfo};

const URL_API: &str = "https://www.metaweather.com/api";

pub fn get_weather(woeid: &str) -> Result<ListWeatherInfo, reqwest::Error> {
    let path: String = format!("{}/location/{id}/", URL_API, id = woeid);
    let list_weather: ListWeatherInfo = reqwest::get(&path)?.json()?;

    Ok(list_weather)
}

pub fn search_city(city_name: &str) -> Result<Vec<City>, (reqwest::Error)> {
    let path: String = format!(
        "{}/location/search/?query={search}",
        URL_API,
        search = city_name
    );
    let search_result: Vec<City> = reqwest::get(&path)?.json()?;

    Ok(search_result)
}
