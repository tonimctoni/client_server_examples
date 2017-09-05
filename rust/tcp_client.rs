use std::net::TcpStream;
use std::io::prelude::*;


fn main() {
    let mut stream = TcpStream::connect("[::1]:1234").expect("TcpStream::connect");

    match stream.write("Hello(tcp) from (rust)client".as_bytes()) {
        Err(e) => println!("Error (connection.Write): {}", e),
        Ok(n) => println!("Data sent: {}", n),
    }
}