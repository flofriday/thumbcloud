use actix::*;
use actix_web;
use actix_web::http::header::DispositionParam::{Ext, Filename};
use actix_web::*;
use actix_web::{dev::Handler, error::Error, fs::StaticFiles, http::header::*, http::Method};
use askama::Template;
use futures::future;
use futures::{Future, Stream};
use std::fs;
use std::io;
use std::io::Write;
use std::net::SocketAddr;

use config;
use config::Config;
use decoder;
use system;

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

fn index(req: &HttpRequest<AppState>) -> Result<HttpResponse> {
    println!("Visit index");

    let content = IndexTemplate {
        app_name: &req.state().config.app_name,
        page: "Index",
    }.render()
        .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(content))
}

fn save_file(
    field: multipart::Field<actix_web::dev::Payload>,
    config: &Config,
) -> Box<Future<Item = i64, Error = Error>> {
    let raw = match field.content_disposition() {
        Some(e) => e.parameters,
        None => {
            return Box::new(future::err(error::ErrorInternalServerError(
                "no valid file",
            )))
        }
    };

    let mut file_name: Vec<u8> = Vec::new();
    let mut file_path = String::new();

    for i in raw.iter() {
        match i {
            Filename(_, _, a) => file_name = a.to_vec(),
            Ext(name, path) => {
                if name == "name" {
                    file_path = path.trim().to_string();
                }
            }
        }
    }

    if file_name.len() == 0 {
        return Box::new(future::err(error::ErrorInternalServerError(
            "no valid file",
        )));
    }

    let file_name = match String::from_utf8(file_name.clone()) {
        Ok(n) => n,
        Err(e) => return Box::new(future::err(error::ErrorInternalServerError(e))),
    };

    println!("Upload: {:?}", file_name);

    //TODO: WARNING the following line is not secure and can be abused for path tranversal attacks
    let file_path = config.path.clone().join(file_path).join(file_name);
    let mut file = match fs::File::create(file_path) {
        Ok(file) => file,
        Err(e) => return Box::new(future::err(error::ErrorInternalServerError(e))),
    };

    Box::new(
        field
            .fold(0i64, move |acc, bytes| {
                let rt = file
                    .write_all(bytes.as_ref())
                    .map(|_| acc + bytes.len() as i64)
                    .map_err(|e| {
                        println!("file.write_all failed: {:?}", e);
                        error::MultipartError::Payload(error::PayloadError::Io(e))
                    });
                future::result(rt)
            })
            .map_err(|e| {
                println!("save_file failed, {:?}", e);
                error::ErrorInternalServerError(e)
            }),
    )
}

fn handle_multipart_item<'a>(
    item: multipart::MultipartItem<actix_web::dev::Payload>,
    config: &'a Config,
) -> Box<Stream<Item = i64, Error = Error>> {
    let confignew = config.clone();

    match item {
        multipart::MultipartItem::Field(field) => Box::new(save_file(field, &config).into_stream()),
        multipart::MultipartItem::Nested(mp) => Box::new(
            mp.map_err(error::ErrorInternalServerError)
                .map(move |x| handle_multipart_item(x, &confignew))
                .flatten(),
        ),
    }
}

fn upload(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let config = req.state().config.clone();

    Box::new(
        req.multipart()
            .map_err(error::ErrorInternalServerError)
            .map(move |x| handle_multipart_item(x, &config))
            .flatten()
            .collect()
            .map(|sizes| HttpResponse::Ok().json(sizes))
            .map_err(|e| {
                println!("failed: {}", e);
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
    ws::start(&req, WsSession { config: config })
}

fn print_bind_error(err: io::Error, addr: &SocketAddr) {
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
        err.get_ref().unwrap_or(&err),
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
            ws::Message::Text(text) => ctx.text(decoder::decode(text, &self.config)),
            ws::Message::Binary(bin) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub fn run(config: Config) {
    // TODO: there should be no need to create the conf variable here, because
    // config already has the path
    let server = match server::new(move || {
        App::with_state(AppState::new())
            .resource("/about", |r| r.f(about))
            .resource("/system", |r| r.f(system))
            .resource("/ws/", |r| r.route().f(ws_route))
            .handler("/download", move |req: &HttpRequest<AppState>| {
                let conf = config::parse_arguments();
                StaticFiles::new(conf.path).unwrap().handle(&req).map(|ok| {
                    ok.map(|mut response| {
                        response
                            .headers_mut()
                            .insert(CONTENT_DISPOSITION, "attachment".parse().unwrap());
                        response
                    }).responder()
                })
            })
            .handler("/static", StaticFiles::new("./static/").unwrap().default_handler(default))
            .resource("/upload", |r| {
                r.method(http::Method::POST).with(upload)
            })
            .resource("/", |r| {
                r.method(Method::GET).f(index);
            })
            .default_resource(|r| r.f(default))
    }).bind(&config.addr)
    {
        Ok(server) => server,
        Err(e) => {
            print_bind_error(e, &config.addr);
            return;
        }
    };

    println!("Started {} at: {}\n", config.app_name, config.addr);
    server.run();
}
