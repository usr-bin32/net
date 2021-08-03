//! Este projeto consiste em uma aplicação cliente-servidor que implementa a função de [`echo`](https://en.wikipedia.org/wiki/Echo_(command)) dentro de uma máquina via sockets TCP.
use serde::{Deserialize, Serialize};

pub mod client;
pub mod server;

/// Endereço IPV4 do servidor de echo.
pub const SERVER_ADDR: &'static str = "127.0.0.1:7878";

/// Estrutura que representa uma requisição (comando) realizada pelo processo cliente.
/// A requisição é serializada no formato MsgPack.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Request {
    /// Representa uma solicitação de echo. O servidor responderá ao cliente com uma `String`
    /// de conteúdo idêntico ao conteúdo presente na requisição.
    Echo(String),
    /// Representa uma solicitação de término da conexão. O servidor encerrará a sessão
    /// de comunicação com o cliente após receber essa mensagem.
    Exit,
}
