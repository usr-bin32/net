//! Estruturas e funções relativas ao processo cliente.

use std::{
    io::{self, prelude::*},
    net::TcpStream,
};

use crate::Request;

/// Executa um novo processo cliente de forma síncrona. O cliente tenta se conectar ao servidor
/// local e então passa a interagir com o usuário via entrada padrão e trocar mensagens com o
/// servidor via TCP, até que o usuário encerre a sessão.
pub fn run() {
    let mut stream = TcpStream::connect(crate::SERVER_ADDR).unwrap();

    println!("Connected to the server");

    loop {
        let request = read_request();

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

/// Tenta ler uma nova requisição a partir da entrada padrão. A função não retorna
/// até que o usuário entre com um comando válido.
fn read_request() -> Request {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = preprocess_command(&input);

        match parse_command(input) {
            Some(request) => break request,
            None => continue,
        }
    }
}

/// Preprocessa a entrada do usuário, removendo espaços em branco no início da string
/// e eliminando a última quebra de linha.
fn preprocess_command(input: &str) -> &str {
    let input = input.trim_start();
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(&input)
}

/// Lê uma string e a converte na requisição equivalente, se possível. Caso a string
/// não corresponda a uma `Request` válida, a função retorna `None`.
fn parse_command(input: &str) -> Option<Request> {
    let (command, message) = match input.split_once(' ') {
        Some(content) => content,
        None => (input, ""),
    };

    match command.to_ascii_lowercase().as_str() {
        "echo" => Some(Request::Echo(message.into())),
        "exit" => Some(Request::Exit),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preprocess_regular() {
        assert_eq!("echo Hello!", preprocess_command("echo Hello!\r\n"));
    }

    #[test]
    fn preprocess_leading_whitespace() {
        assert_eq!("exit ", preprocess_command("    exit \n"));
    }

    #[test]
    fn preprocess_multi_newline() {
        assert_eq!("echo Hello!\n", preprocess_command("echo Hello!\n\n"));
    }

    #[test]
    fn parse_echo() {
        assert_eq!(
            Some(Request::Echo("Hello!".into())),
            parse_command("echo Hello!")
        );
    }

    #[test]
    fn parse_exit() {
        assert_eq!(Some(Request::Exit), parse_command("exit "));
    }

    #[test]
    fn parse_invalid() {
        assert_eq!(None, parse_command("something 123 abc"));
    }
}
