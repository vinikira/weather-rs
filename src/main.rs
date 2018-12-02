extern crate chrono;
extern crate clap;
extern crate colored;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod api;
mod handler;
mod types;

use clap::{App, Arg};
use self::handler::{get, search};

fn main() {
    let matches = App::new("weater")
        .version("1.0.0")
        .about("Get weater form https://www.metaweather.com/")
        .author("Vinícius Simões")
        .arg(
            Arg::with_name("COMMAND")
                .takes_value(true)
                .index(1)
                .help("command to execute."),
        )
        .arg(
            Arg::with_name("ARGUMENT")
                .takes_value(true)
                .index(2)
                .require_delimiter(true)
                .help("argument of command"),
        )
        .get_matches();

    let command = matches.value_of("COMMAND").unwrap();
    let argument = matches.value_of("ARGUMENT").unwrap();

    match command {
        "search" => println!("{}", search(argument)),
        "get" => println!("{}", get(argument)),
        _ => println!("Choose a valid command. (wheater --help for more information)"),
    }
}
