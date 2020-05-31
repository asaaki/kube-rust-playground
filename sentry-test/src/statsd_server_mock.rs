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

use async_std::net::UdpSocket;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8125").await?;
    println!("Listening on {}", socket.local_addr()?);

    let mut buf = vec![0u8; 1024];

    loop {
        let (recv, peer) = socket.recv_from(&mut buf).await?;
        let data = std::str::from_utf8(&buf[..recv]).unwrap().trim();
        println!("Received {} bytes from {} | DATA: {}", recv, peer, data);
    }
}
