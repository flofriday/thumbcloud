use actix_web;
use actix_web::error::Error;
use actix_web::http::header::DispositionParam::{Ext, Filename};
use actix_web::*;
use futures::future;
use futures::{Future, Stream};
use std::fs;
use std::io::Write;

use config::Config;

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

pub fn handle_multipart_item<'a>(
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
