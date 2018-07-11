use actix::*;
use actix_web::dev::Handler;
use actix_web::fs::{NamedFile, StaticFiles};
use actix_web::http::header::*;
use actix_web::*;
use futures::future::Future;
use std::path::PathBuf;

use config;
use config::Config;
use decoder;

struct AppState {
    config: config::Config,
}

fn index(_req: HttpRequest<AppState>) -> Result<NamedFile> {
    println!("Visiting index");
    let path = PathBuf::from("./static/html/index.html");
    Ok(NamedFile::open(path)?)
}

fn about(_req: HttpRequest<AppState>) -> Result<NamedFile> {
    println!("Visiting about");
    let path = PathBuf::from("./static/html/about.html");
    Ok(NamedFile::open(path)?)
}

fn system(_req: HttpRequest<AppState>) -> Result<NamedFile> {
    println!("Visiting system");
    let path = PathBuf::from("./static/html/system.html");
    Ok(NamedFile::open(path)?)
}

fn ws_route(req: HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let path = req.state().config.path.clone();
    ws::start(req, WsSession { path: path })
}

fn get_bind_error(addr: &String) -> String {
    format!(
        "\n
            --------------------------------------------------------------------
            Can not bind server to: {}
            
            Possible reasons for this error are:
            1. The given IP address is invalid or does not belong to your 
               computer
            2. The given Port number is already used by another programm
            3. The IP and Port number are valid however, your OS needs root 
               permission to use it, in which case `sudo thumbcloud` should work
            --------------------------------------------------------------------
            \n",
        addr
    )
}

struct WsSession {
    path: PathBuf,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self, AppState>;
}

impl StreamHandler<ws::Message, ws::ProtocolError> for WsSession {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => ctx.text(decoder::decode(text, &self.path)),
            ws::Message::Binary(bin) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub fn run(config: Config) {
    println!("Started webserver at: {}", config.addr);

    // TODO: there should be no need to create the conf variable here, because
    // config already has the path
    server::new(move || {
        //App::with_state(config::parse_arguments())
        App::with_state(AppState {
            config: config::parse_arguments(),
        }).resource("/about", |r| r.f(about))
            .resource("/system", |r| r.f(system))
            .resource("/ws/", |r| r.route().f(ws_route))
            .handler("/download", move |req: HttpRequest<AppState>| {
                let conf = config::parse_arguments();
                StaticFiles::new(conf.path).handle(req).map(|ok| {
                    ok.map(|mut response| {
                        response
                            .headers_mut()
                            .insert(CONTENT_DISPOSITION, "attachment".parse().unwrap());
                        response
                    }).responder()
                })
            })
            .handler("/", StaticFiles::new("./static/").default_handler(index))
    }).bind(&config.addr)
        .expect(get_bind_error(&config.addr).as_str())
        .run();
}
