use std::env;
use std::net::UdpSocket;
use std::io;
use std::io::prelude::*;


fn main() {
    let mut args=env::args().skip(1);
    let address:&str=&args.next().expect("Exactly 1 parameter needed: ip:port");

    let stdin=io::stdin();
    let socket=UdpSocket::bind("[::]:0").expect("UdpSocket::bind");
    if address.len()>=15 && &address[..15]=="255.255.255.255"{
        socket.set_broadcast(true).expect("Error (set_broadcast)");
        println!("Broadcast enabled.");
    }

    for line in stdin.lock().lines(){
        let line=match line {
            Err(..) => continue,
            Ok(line) => line,
        };

        match socket.send_to(line.as_bytes(), address) {
            Err(e) => println!("Error (connection.Write): {}", e),
            Ok(n) => println!("Data sent: {}", n),
        }
    }
}