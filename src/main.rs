extern crate actix;
extern crate actix_web;

#[macro_use]
extern crate askama;

#[macro_use]
extern crate clap;
extern crate futures;
extern crate htmlescape;
extern crate machine_ip;
extern crate open;
extern crate pretty_bytes;
extern crate serde;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

pub mod config;
pub mod decoder;
pub mod files;
pub mod system;
pub mod upload;
pub mod webserver;

/// Starts the main steps of the application.
/// 1. parse the command line arguments
/// 2. start the server
fn main() {
    let config = config::parse_arguments();
    webserver::run(&config);
}
