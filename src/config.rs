use clap::{AppSettings, Arg, ArgMatches};
use std::path::PathBuf;

pub struct Config {
    pub path: PathBuf,
}

impl Config {
    fn from_matches(matches: ArgMatches) -> Config {
        Config {
            path: PathBuf::from(matches.value_of("INPUT").unwrap()),
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
                .help("Sets the path to share")
                .required(true)
                .validator(is_valid_path)
                .index(1),
        )
        .setting(AppSettings::ColorAlways)
        .get_matches();

    Config::from_matches(matches)
}
