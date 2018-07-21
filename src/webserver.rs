use actix::*;
use actix_web::dev::Handler;
use actix_web::fs::{NamedFile, StaticFiles};
use actix_web::http::header::*;
use actix_web::*;
use askama::Template;
use futures::future::Future;
use std::path::PathBuf;

use config;
use config::Config;
use decoder;

struct AppState {
    config: config::Config,
}

impl AppState {
    fn new() -> AppState {
        AppState {
            config: config::parse_arguments(),
        }
    }
}

fn index(_req: HttpRequest<AppState>) -> Result<NamedFile> {
    println!("Visiting index");
    let path = PathBuf::from("./static/html/index.html");
    Ok(NamedFile::open(path)?)
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate<'a> {
    app_name: String,
    description: &'a str,
    version: &'a str,
    license: &'a str,
    repository: &'a str,
    repository_name: &'a str,
}

// TODO: This code shouldn't be that hardcoded. The right way would be to load
// and parse the Cargo.toml file at compile-time.
fn about(_req: HttpRequest<AppState>) -> Result<HttpResponse> {
    println!("Visiting about");

    let content = AboutTemplate {
        app_name: {
            // Capitalize the first character of the name
            let s1 = env!("CARGO_PKG_NAME");
            let mut v: Vec<char> = s1.chars().collect();
            v[0] = v[0].to_uppercase().nth(0).unwrap();
            v.into_iter().collect()
        },
        description: env!("CARGO_PKG_DESCRIPTION"),
        version: env!("CARGO_PKG_VERSION"),
        license: "MIT",
        repository: env!("CARGO_PKG_HOMEPAGE"),
        repository_name: "flofriday/thumbcloud",
    }.render()
        .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(content))
}

#[derive(Template)]
#[template(path = "system.html")]
struct SystemTemplate {
    //name: &'a str,
}

fn system(_req: HttpRequest<AppState>) -> Result<HttpResponse> {
    println!("Visiting system");

    let content = SystemTemplate {
        //name: "hello"
    }.render()
        .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(content))
}

fn ws_route(req: HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let path = req.state().config.path.clone();
    ws::start(req, WsSession { path: path })
}

// TODO: this should not be inside an panic but a standalone message
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
        App::with_state(AppState::new())
            .resource("/about", |r| r.f(about))
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
