use std::{
    io::{self, prelude::*},
    net::TcpStream,
};

use net::{command, Request};

fn main() {
    let mut stream = TcpStream::connect(net::SERVER_ADDR).unwrap();

    println!("Connected to the server");

    loop {
        let request = read_user_request();

        let bytes = rmp_serde::to_vec(&request).unwrap();
        stream.write(&bytes).unwrap();

        match request {
            Request::Exit => break,
            Request::Echo(_) => {
                let response: String = rmp_serde::from_read(&mut stream).unwrap();
                println!(">> {}", response);
            }
        }
    }

    println!("Finishing connection...");
}

fn read_user_request() -> Request {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = command::preprocess(&input);

        match command::parse(input) {
            Some(request) => break request,
            None => continue,
        }
    }
}
