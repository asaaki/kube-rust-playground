use std::{env, net::SocketAddr};

// static APP_NAME: &str = env!("CARGO_PKG_NAME");

static DEFAULT_PORT: &str = "8080";
static DEFAULT_HOST: &str = "127.0.0.1";

fn addr() -> SocketAddr {
    format!("{}:{}", host(), port())
        .parse()
        .expect("HOST:PORT does not form a valid address")
}
fn host() -> String { env::var("HOST").unwrap_or_else(|_| DEFAULT_HOST.into()) }
fn port() -> String { env::var("PORT").unwrap_or_else(|_| DEFAULT_PORT.into()) }

async fn handler(req: tide::Request<()>) -> tide::Result {
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
        "#, &req);
        let resp = tide::Response::new(tide::StatusCode::Ok)
        .body_string(body)
        .set_header("content-type".parse().unwrap(), "text/html; charset=utf-8");
        Ok(resp) 
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new();
    app.at("/*").get(handler);

    println!("Runs at: {}", addr());
    app.listen(addr()).await?;
    Ok(())
}
