extern crate actix;
extern crate actix_web;

pub mod webserver;

fn main() {
    webserver::run();
}
