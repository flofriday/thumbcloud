use clap::{AppSettings, Arg, ArgMatches};
use std::path::PathBuf;

// TODO: the addres shouldn't be a String
pub struct Config {
    pub path: PathBuf,
    pub addr: String,
}

impl Config {
    fn from_matches(matches: ArgMatches) -> Config {
        Config {
            path: PathBuf::from(matches.value_of("INPUT").unwrap()),
            addr: String::from(matches.value_of("address").unwrap_or("127.0.0.1:8080")),
        }
    }
}

fn is_valid_path(input: String) -> Result<(), String> {
    let path = PathBuf::from(&input);
    if path.is_dir() {
        return Ok(());
    }

    if path.is_file() {
        return Err(String::from(
            "The path pointed to a file, but only directories can be shared",
        ));
    }

    Err(String::from(format!(
        "Could not locate the given directory {:?}",
        input
    )))
}

pub fn parse_arguments() -> Config {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the path thumbcloud will share")
                .required(true)
                .validator(is_valid_path)
                .index(1),
        )
        .arg(
            Arg::with_name("address")
                .help("Sets the IP address and port the server will launch")
                .short("a")
                .long("addr")
                .takes_value(true),
        )
        .setting(AppSettings::ColorAlways)
        .get_matches();

    Config::from_matches(matches)
}
