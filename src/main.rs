extern crate chrono;
extern crate clap;
extern crate colored;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use self::handler::{get, search};
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("weater")
        .version("1.0.0")
        .about("Get weater form https://www.metaweather.com/")
        .author("Vinícius Simões")
        .subcommand(
            SubCommand::with_name("search")
                .about("Search city by name")
                .arg(Arg::with_name("city_name").help("City name")),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get weather by city woid")
                .arg(Arg::with_name("city_woid").help("City woid.")),
        )
        .get_matches();

    if let Some(search_matches) = matches.subcommand_matches("search") {
        search(search_matches);
    }

    if let Some(get_matches) = matches.subcommand_matches("get") {
        get(get_matches);
    }
}
