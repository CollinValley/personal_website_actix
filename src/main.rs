extern crate actix_web;
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
