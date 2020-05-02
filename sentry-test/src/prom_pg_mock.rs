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
