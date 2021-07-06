use serde::{Deserialize, Serialize};

pub const SERVER_ADDR: &'static str = "127.0.0.1:7878";

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    Echo(String),
    Exit,
}
