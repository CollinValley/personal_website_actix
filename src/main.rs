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

//Starting off, just want to serve a static index file, as html. Right now the browser tries to download
// the file, though that might just be a product of it not being the .html extension.
// It would also be nice to serve DawnCronin.com and CollinValley.com from seperate workers. Essentially
// we do want to leverage what makes this framework good.
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
