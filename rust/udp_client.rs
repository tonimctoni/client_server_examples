use std::net::UdpSocket;


fn main() {
    let socket=UdpSocket::bind("[::]:0").expect("UdpSocket::bind");

    match socket.send_to("Hello(udp) from (rust)client".as_bytes(), "[::1]:1234") {
        Err(e) => println!("Error (connection.Write): {}", e),
        Ok(n) => println!("Data sent: {}", n),
    }
}