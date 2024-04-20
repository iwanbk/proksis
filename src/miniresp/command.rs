use bytes::Bytes;

pub enum CommandName {
    Get,
    Set,
    Unknown,
}

pub struct Command {
    pub name: String,
    pub key: String,
    pub raw: Bytes,
}

impl Command {
    pub fn new(name: String, key: String, raw: Bytes) -> Command {
        Command {
            name,
            key,
            raw,
        }
    }
}