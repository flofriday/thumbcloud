extern crate actix;
extern crate actix_web;
extern crate futures;
extern crate serde;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

pub mod args;
pub mod decoder;
pub mod files;
pub mod webserver;

fn main() {
    webserver::run();
}
