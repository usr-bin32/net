use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
};

use net::Request;

fn main() {
    let listener = TcpListener::bind(net::SERVER_ADDR).unwrap();

    println!("Server started at {}", net::SERVER_ADDR);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || handle_client(stream));
    }
}

fn handle_client(mut stream: TcpStream) {
    let peer_addr = stream.peer_addr().unwrap();

    println!("Opened connection with {:?}", peer_addr);

    loop {
        let request = match rmp_serde::from_read(&mut stream) {
            Ok(request) => request,
            Err(_) => break,
        };

        match request {
            Request::Echo(string) => {
                let response = rmp_serde::to_vec(&string).unwrap();
                stream.write(&response).unwrap();
            }
            Request::Exit => {
                break;
            }
        }
    }

    println!("Closed connection with {:?}", peer_addr);
}
