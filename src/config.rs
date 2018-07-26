use clap::{AppSettings, Arg, ArgMatches};
use std::path::PathBuf;

pub struct Config {
    pub path: PathBuf,
    pub addr: String,
    pub app_name: String,
    pub crate_name: String,
}

impl Config {
    fn from_matches(matches: ArgMatches) -> Config {
        let crate_name: String = {
            // Capitalize the first character of the crate name
            let s1 = env!("CARGO_PKG_NAME");
            let mut v: Vec<char> = s1.chars().collect();
            v[0] = v[0].to_uppercase().nth(0).unwrap();
            v.into_iter().collect()
        };

        Config {
            path: PathBuf::from(matches.value_of("INPUT").unwrap()),
            addr: String::from(matches.value_of("address").unwrap_or("localhost:8080")),
            app_name: if let Some(name) = matches.value_of("name") {
                String::from(name)
            } else {
                crate_name.clone()
            },
            crate_name: crate_name,
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
        .arg(
            Arg::with_name("name")
                .help("Sets a custom servername")
                .short("n")
                .long("name")
                .takes_value(true),
        )
        .setting(AppSettings::ColorAlways)
        .get_matches();

    Config::from_matches(matches)
}
