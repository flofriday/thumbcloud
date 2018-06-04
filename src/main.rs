extern crate actix;
extern crate actix_web;

extern crate serde;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub mod args;
pub mod webserver;
pub mod decoder;
pub mod files;

fn main() {
    webserver::run();
}

