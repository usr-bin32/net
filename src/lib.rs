use serde::{Deserialize, Serialize};

pub mod command;

pub const SERVER_ADDR: &'static str = "127.0.0.1:7878";

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Request {
    Echo(String),
    Exit,
}
