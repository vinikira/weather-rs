use clap::{App, Arg, SubCommand};
use weather_rs::CLIApp;

fn main() {
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
                .arg(Arg::with_name("place_name").help("Place name."))
                .arg(
                    Arg::with_name("json")
                        .takes_value(false)
                        .long("--json")
                        .help("Print JSON representation."),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get weather by place id.")
                .arg(Arg::with_name("place_id").help("Place id."))
                .arg(
                    Arg::with_name("json")
                        .takes_value(false)
                        .long("--json")
                        .help("Print JSON representation."),
                ),
        )
        .get_matches();

    let provider = matches.value_of("provider").unwrap_or("").to_owned();

    let cli = CLIApp::new(&provider, matches);

    cli.perform();
}
