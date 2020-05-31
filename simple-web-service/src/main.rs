#![forbid(unsafe_code)]
#![deny(warnings)]

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use env::var;
use std::{env, io::Error, net::SocketAddr};
use tide::{Request, Response, Result as TideResult, StatusCode};

static DEFAULT_PORT: &str = "8080";
static DEFAULT_HOST: &str = "127.0.0.1";

fn addr() -> SocketAddr {
    format!("{}:{}", host(), port())
        .parse()
        .expect("HOST:PORT does not form a valid address")
}
fn host() -> String {
    var("HOST").unwrap_or_else(|_| DEFAULT_HOST.into())
}
fn port() -> String {
    var("PORT").unwrap_or_else(|_| DEFAULT_PORT.into())
}

async fn handler(req: Request<()>) -> TideResult {
    let body = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>simple web service</title>
        </head>
        <body>
            <h1>Hello from Rust in a container in kubernetes cluster!</h1>
            <pre>{:#?}</pre>
        </body>
        </html>
        <!-- end -->
        "#,
        &req
    );
    let resp = Response::new(StatusCode::Ok)
        .body_string(body)
        .set_mime(mime::TEXT_HTML_UTF_8);
    Ok(resp)
}

#[async_std::main]
async fn main() -> Result<(), Error> {
    femme::with_level(tide::log::Level::Info.to_level_filter());
    let mut app = tide::new();
    app.at("/").get(handler);
    app.at("*").all(handler);
    println!("Runs at: {}", addr());
    app.listen(addr()).await?;
    Ok(())
}
