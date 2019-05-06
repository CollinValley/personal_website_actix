extern crate actix_web;

use actix_web::actix::System;
use std::path::{PathBuf, Path};
use std::fs::File;
use actix_web::{server::HttpServer, http, App, HttpRequest, HttpResponse};
use std::io::Read;

/// Reads a file from the 'static' directory and returns its
/// contents an Option<String>
fn read_static_file(path: &Path) -> Option<String> {
   let static_path = Path::new("static/").join(path);
   let file = File::open(static_path);
   match file {
       Ok(mut file) => {
          let mut contents = String::new();
          match file.read_to_string(&mut contents) {
              Ok(_) => Some(contents),
              Err(_) => None,
          }
       },
       Err(_) => return None,
   }
}

fn index( info: &HttpRequest) -> HttpResponse {
    let requested_file = PathBuf::from(info.path().trim_start_matches("/"));

    let contents = read_static_file(requested_file.as_path());
    match contents {
        Some(file_bytes) => {
            HttpResponse::Ok()
                .body(file_bytes)
        },
        None => HttpResponse::NotFound().body("Whoops that file doesn't exist"),
    }
}

fn main() {
    let sys = System::new("guide");

    HttpServer::new(|| App::new().default_resource( |r| r.f(index)))
        .bind("127.0.0.1:1337")
        .unwrap()
        .start();

    let _ = sys.run();
}


/*
use std::path::PathBuf;
use actix_web::{server, pred, App, HttpRequest, Result, http::Method, fs::NamedFile};

fn collinvalleyindex( _req: &HttpRequest) -> &'static str {
    "Hello from CollinValley.com"
}

fn dawncroninindex(_req: &HttpRequest) -> &'static str {
    "Hello from DawnCronin.com"
}

fn index( _req: &HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open("static/index")?)
}

fn main() {
    let server = server::new( || {
        vec![
            App::new()
                .filter(pred::Host("collinvalley.com"))
                .resource("/", |r| r.f(collinvalleyindex)),
            App::new()
                .filter(pred::Host("dawncronin.com"))
                .resource("/", |r| r.f(dawncroninindex)),
            App::new()
                .resource("/", |r| r.method(Method::GET).f(index)),
        ]
    });

    server.bind("127.0.0.1:8000").unwrap().run();
}

*/
