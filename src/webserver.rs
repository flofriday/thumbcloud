use actix::*;
use actix_web::*;
use actix_web::fs::{NamedFile, StaticFiles};
use actix_web::http::header::*;
use actix_web::dev::Handler;
use futures::future::Future;
use std::path::PathBuf;

use config::Config;
use decoder;

fn index(_req: HttpRequest) -> Result<NamedFile> {
    println!("Visiting index");
    let path = PathBuf::from("./static/html/index.html");
    Ok(NamedFile::open(path)?)
}

fn about(_req: HttpRequest) -> Result<NamedFile> {
    println!("Visiting about");
    let path = PathBuf::from("./static/html/about.html");
    Ok(NamedFile::open(path)?)
}

fn system(_req: HttpRequest) -> Result<NamedFile> {
    println!("Visiting system");
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
            ws::Message::Text(text) => ctx.text(decoder::decode(text)),
            ws::Message::Binary(bin) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub fn run(_config: Config) {
    let addr = "127.0.0.1:80";

    server::new(|| {
        App::new()
            .resource("/about", |r| r.f(about))
            .resource("/system", |r| r.f(system))
            .resource("/ws/", |r| r.f(|req| ws::start(req, Ws)))
            .handler("/download", |req: HttpRequest| {
                StaticFiles::new("./").handle(req).map(|ok| {
                    ok.map(|mut response| {
                        response
                            .headers_mut()
                            .insert(CONTENT_DISPOSITION, "attachment".parse().unwrap());
                        response
                    }).responder()
                })
            })
            .handler("/", StaticFiles::new("./static/").default_handler(index))
    }).bind(addr)
        .expect(format!("Can not start server on: {}", addr).as_str())
        .run();
}
