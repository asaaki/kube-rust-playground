#![forbid(unsafe_code)]
#![warn(rust_2018_idioms)]
#![deny(warnings)]
#![deny(clippy::cargo)]
// workspace might have projects naturally depending on different versions:
#![allow(clippy::multiple_crate_versions)]
// we're not going to release a crate anyway:
#![allow(clippy::cargo_common_metadata)]
#![deny(clippy::pedantic)]
#![deny(clippy::result_unwrap_used)]
#![deny(clippy::panic)]

use std::io::Error;
use tide::{Request, Result as TideResult};

async fn handler(mut req: Request<()>) -> TideResult {
    let reqbody: String = req.body_string().await.unwrap().trim().into();
    println!("DATA: {}", reqbody);
    Ok("Ok\n".into())
}

#[async_std::main]
async fn main() -> Result<(), Error> {
    let addr = "127.0.0.1:9091";
    let mut app = tide::new();
    app.at("/").get(handler);
    app.at("*").all(handler);
    println!("Runs at: {}", addr);
    app.listen(addr).await?;
    Ok(())
}
