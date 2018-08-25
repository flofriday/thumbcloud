use actix::*;
use actix_web::*;
use actix_web::{dev::Handler, error::Error, fs::StaticFiles, http::header::*, http::Method};
use askama::Template;
use futures::{Future, Stream};
use std::io;
use std::net::SocketAddr;

use config;
use config::Config;
use decoder;
use system;
use upload;

struct AppState {
    config: config::Config,
}

impl AppState {
    fn from_config(config: Config) -> AppState {
        AppState { config }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    app_name: &'a str,
    page: &'a str,
}

fn index(req: &HttpRequest<AppState>) -> Result<HttpResponse> {
    println!("Visit index");

    let content = IndexTemplate {
        app_name: &req.state().config.app_name,
        page: "Index",
    }.render()
        .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(content))
}

fn upload(req: &HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let config = req.state().config.clone();

    Box::new(
        req.multipart()
            .map_err(error::ErrorInternalServerError)
            .map(move |x| upload::handle_multipart_item(x, &config))
            .flatten()
            .collect()
            .map(|sizes| HttpResponse::Ok().json(sizes))
            .map_err(|e| {
                println!("Upload failed: {}", e);
                e
            }),
    )
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate<'a> {
    app_name: &'a str,
    page: &'a str,
    header: String,
    description: &'a str,
    version: &'a str,
    license: &'a str,
    repository: &'a str,
    repository_name: &'a str,
}

// TODO: This code shouldn't be that hardcoded. The right way would be to load
// and parse the Cargo.toml file at compile-time.
fn about(req: &HttpRequest<AppState>) -> Result<HttpResponse> {
    println!("Visit about");

    let header =
        if req.state().config.app_name.to_lowercase() == env!("CARGO_PKG_NAME").to_lowercase() {
            format!("About {}", &req.state().config.app_name)
        } else {
            format!(
                "About {} (based on {})",
                &req.state().config.app_name,
                &req.state().config.crate_name
            )
        };

    let content = AboutTemplate {
        app_name: &req.state().config.app_name,
        page: "About",
        header,
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
struct SystemTemplate<'a> {
    app_name: &'a str,
    page: &'a str,
    addr: &'a SocketAddr,
    os: &'a str,
}

fn system(req: &HttpRequest<AppState>) -> Result<HttpResponse> {
    println!("Visit system");

    let content = SystemTemplate {
        app_name: &req.state().config.app_name,
        page: "System",
        addr: &req.state().config.addr,
        os: system::get_os(),
    }.render()
        .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(content))
}

#[derive(Template)]
#[template(path = "default.html")]
struct DefaultTemplate<'a> {
    app_name: &'a str,
    page: &'a str,
}

fn default(req: &HttpRequest<AppState>) -> Result<HttpResponse> {
    println!("Visit Page not found");

    let content = DefaultTemplate {
        app_name: &req.state().config.app_name,
        page: "Page not found",
    }.render()
        .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(content))
}

fn ws_route(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let config = req.state().config.clone();
    ws::start(&req, WsSession { config })
}

fn print_bind_error(err: &io::Error, addr: &SocketAddr) {
    println!(
        "\n
            BIND ERROR: \"{}\" 
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
        err.get_ref().unwrap_or(err),
        addr
    )
}

struct WsSession {
    config: Config,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self, AppState>;
}

impl StreamHandler<ws::Message, ws::ProtocolError> for WsSession {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => ctx.text(decoder::decode(&text, &self.config)),
            ws::Message::Binary(bin) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub fn run(config: &Config) {
    //NOTE: config2 and config3 are exact copies of the config struct passed to this function.
    //I know that this code looks more that unusual, and is just a copy-hell, but it was the only
    //way I could come up with to parse the commandline arguments just once.
    //Feel free to improve this code.

    let config2 = config.clone();
    let server = match server::new(move || {
        let config3 = config2.clone();
        App::with_state(AppState::from_config(config2.clone()))
            .resource("/about", |r| r.f(about))
            .resource("/system", |r| r.f(system))
            .resource("/ws/", |r| r.route().f(ws_route))
            .handler("/download", move |req: &HttpRequest<AppState>| {
                StaticFiles::new(config3.clone().path)
                    .unwrap()
                    .handle(&req)
                    .map(|ok| {
                        ok.map(|mut response| {
                            response
                                .headers_mut()
                                .insert(CONTENT_DISPOSITION, "attachment".parse().unwrap());
                            response
                        }).responder()
                    })
            })
            .handler(
                "/static",
                StaticFiles::new("./static/")
                    .unwrap()
                    .default_handler(default),
            )
            .resource("/upload", |r| {
                r.method(Method::POST).f(upload);
            })
            .resource("/", |r| r.route().f(index))
            .default_resource(|r| r.f(default))
    }).bind(&config.addr)
    {
        Ok(server) => server,
        Err(e) => {
            print_bind_error(&e, &config.addr);
            return;
        }
    };

    println!("Started {} at: {}\n", config.app_name, config.addr);
    server.run();
}
