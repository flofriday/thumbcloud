use clap::{AppSettings, Arg, ArgMatches};
use machine_ip;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs};
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Clone)]
pub struct Config {
    pub path: PathBuf,
    pub addr: SocketAddr,
    pub app_name: String,
    pub crate_name: String,
    pub start_time: SystemTime,
}

impl Config {
    fn from_matches(matches: &ArgMatches) -> Config {
        let crate_name: String = {
            // Capitalize the first character of the crate name
            let s1 = env!("CARGO_PKG_NAME");
            let mut v: Vec<char> = s1.chars().collect();
            v[0] = v[0].to_uppercase().nth(0).unwrap();
            v.into_iter().collect()
        };

        Config {
            path: PathBuf::from(matches.value_of("INPUT").unwrap()),
            addr: get_address(matches.value_of("address")),
            app_name: match matches.value_of("name") {
                Some(name) => String::from(correct_invalid_name(name, &crate_name)),
                None => crate_name.clone(),
            },
            crate_name,
            start_time: SystemTime::now(),
        }
    }
}

fn get_address(input_address: Option<&str>) -> SocketAddr {
    // Return input address if it is a valid socket
    if let Some(input_address) = input_address {
        if let Ok(mut socket_iter) = input_address.to_socket_addrs() {
            if let Some(socket) = socket_iter.next() {
                return socket;
            }
        }
    }

    let local_ip = machine_ip::get();
    SocketAddr::new(
        local_ip.unwrap_or_else(|| IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
        8080,
    )
}

fn correct_invalid_name<'a>(app_name: &'a str, crate_name: &'a str) -> &'a str {
    if app_name.trim().is_empty() {
        //Entered invalid name, reverting back to default cratename
        crate_name
    } else {
        app_name
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))] //because clap allways passes a String
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

    Err(format!("Could not locate the given directory: {}", input))
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))] //because clap allways passes a String
fn is_valid_address(input: String) -> Result<(), String> {
    if let Ok(mut socket_iter) = input.to_socket_addrs() {
        if socket_iter.next().is_some() {
            return Ok(());
        }
    }

    Err(String::from("Invalid socket address"))
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
                .validator(is_valid_address)
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

    Config::from_matches(&matches)
}
