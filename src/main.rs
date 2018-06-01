extern crate actix;
extern crate actix_web;

use actix::*;
use actix_web::fs::NamedFile;
use actix_web::*;
use std::path::PathBuf;

fn index(_req: HttpRequest) -> Result<NamedFile> {
    println!("fn1");
    let path = PathBuf::from("./static/html/index.html");
    Ok(NamedFile::open(path)?)
}

fn about(_req: HttpRequest) -> Result<NamedFile> {
    println!("fn2");
    let path = PathBuf::from("./static/html/about.html");
    Ok(NamedFile::open(path)?)
}

fn system(_req: HttpRequest) -> Result<NamedFile> {
    println!("fn3");
    let path = PathBuf::from("./static/html/system.html");
    Ok(NamedFile::open(path)?)
}

struct Ws;

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<ws::Message, ws::ProtocolError> for Ws {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => ctx.text(text),
            ws::Message::Binary(bin) => ctx.binary(bin),
            _ => (),
        }
    }
}

fn main() {
    server::new(|| {
        vec![
            App::new().prefix("/about").resource("/", |r| r.f(about)),
            App::new().prefix("/system").resource("/", |r| r.f(system)),
            App::new()
                .resource("/ws/", |r| r.f(|req| ws::start(req, Ws)))
                .handler(
                    "/",
                    fs::StaticFiles::new("./static/").default_handler(index),
                ),
        ]
    }).bind("127.0.0.1:80")
        .expect("Can not start server on given IP/Port")
        .run();
}
