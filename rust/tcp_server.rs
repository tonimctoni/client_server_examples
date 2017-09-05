use std::net::{TcpListener};
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::io::prelude::*;
use std::thread;

// let mut buffer=[0;1000]; // stream.read(&mut buffer); // read_to_end
fn main() {
    let listener=TcpListener::bind("[::]:1234").expect("TcpListener::bind");
    let default_addr=SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0,0,0,0,0,0,0,0)), 0);
    println!("Listening on: {}", listener.local_addr().unwrap_or(default_addr));

    for stream in listener.incoming(){
        match stream {
            Err(e) =>println!("Error (stream from listener.incoming is Err): {}", e),
            Ok(stream) =>{
                thread::spawn(move || {
                    let mut stream=stream;
                    let mut message = String::new();

                    let n=match stream.read_to_string(&mut message) {
                        Ok(n) => n,
                        Err(e) =>{
                            println!("Error (stream.read_to_string is Err): {}", e);
                            return;
                        }
                    };

                    let peer_addr=stream.peer_addr().unwrap_or(default_addr);
                    let local_addr=stream.local_addr().unwrap_or(default_addr);
                    println!("{}->{}: [{}] {}", peer_addr, local_addr, n, message);
                });
            }
        }
    }
}