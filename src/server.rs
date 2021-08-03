//! Estruturas e funções relativas ao processo servidor.

use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
};

use crate::Request;

/// Executa um novo processo servidor de forma blocante por tempo indeterminado. O servidor é
/// capaz de servir um número indeterminado de clientes. Ao receber uma nova solicitação de conexão TCP, o servidor
/// aloca uma nova thread para o cliente, responsável por servi-lo exclusivamente durante seu tempo
/// de vida.
pub fn run() {
    let listener = TcpListener::bind(crate::SERVER_ADDR).unwrap();

    println!("Server started at {}", crate::SERVER_ADDR);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || handle_client(stream));
    }
}

/// Serve um cliente até que a conexão seja finalizada pelo usuário. O servidor irá bloquear a thread
/// de execução até que uma nova mensagem seja recebida. Quando há recebimento de requisião, o servidor
/// deserializa a mensagem e responde de acordo, ou com echo, ou finalizando a sessão. O servidor
/// também pode terminar a conexão em caso de desconexão pelo cliente.
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
