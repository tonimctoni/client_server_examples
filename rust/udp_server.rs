use std::net::UdpSocket;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};


fn main() {
    let socket=UdpSocket::bind("[::]:1234").expect("UdpSocket::bind");
    let default_addr=SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0,0,0,0,0,0,0,0)), 0);
    let local_addr=socket.local_addr().unwrap_or(default_addr);
    println!("Listening on: {}", local_addr);

    let mut buffer=[0;1000];
    loop {
        let (n,peer_addr)=match socket.recv_from(&mut buffer) {
            Ok((n,peer_addr)) => (n,peer_addr),
            Err(e) => {
                println!("Error (socket.recv_from is Err): {}", e);
                continue;
            }
        };

        let message=std::str::from_utf8(&mut buffer[..n]).unwrap_or("[{(ERROR)}]");
        println!("{}->{}: [{}] {}", peer_addr, local_addr, n, message);
    }
}