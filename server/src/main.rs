use std::{io, net::Ipv4Addr};

use server::Server;

fn main() -> io::Result<()> {
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let port = 8003;
    println!("Starting server at {ip}:{port}");
    let mut server = Server::new(ip, port, 60)?;
    server.run();
    Ok(())
}
