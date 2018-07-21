extern crate actix;
extern crate actix_web;

#[macro_use]
extern crate askama;

#[macro_use]
extern crate clap;
extern crate futures;
extern crate pretty_bytes;
extern crate serde;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

pub mod config;
pub mod decoder;
pub mod files;
pub mod webserver;

fn main() {
    let config = config::parse_arguments();
    webserver::run(config);
}
