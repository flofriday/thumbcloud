extern crate actix;
extern crate actix_web;

#[macro_use]
extern crate clap;
extern crate futures;
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
    config::parse_arguments();
    webserver::run();
}
