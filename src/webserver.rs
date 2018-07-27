use actix::*;
use actix_web::dev::Handler;
use actix_web::fs::StaticFiles;
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

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    app_name: &'a str,
    page: &'a str,
}

fn index(req: HttpRequest<AppState>) -> Result<HttpResponse> {
    println!("Visit index");

    let content = IndexTemplate {
        app_name: &req.state().config.app_name,
        page: "Index",
    }.render()
        .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(content))
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
fn about(req: HttpRequest<AppState>) -> Result<HttpResponse> {
    println!("Visit about");

    let header =
        if &req.state().config.app_name.to_lowercase() == &env!("CARGO_PKG_NAME").to_lowercase() {
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
        header: header,
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
}

fn system(req: HttpRequest<AppState>) -> Result<HttpResponse> {
    println!("Visit system");

    let content = SystemTemplate {
        app_name: &req.state().config.app_name,
        page: "System",
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
    println!("Started {} at: {}", config.app_name, config.addr);

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
