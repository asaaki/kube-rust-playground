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
