use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug)]
pub enum Jsonrpc {
    // #[serde(rename = "jsonrpc")]
    Jsonrpc,
}

impl Serialize for Jsonrpc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let str = match self {
            Jsonrpc::Jsonrpc => "2.0".to_string(),
        };
        serializer.serialize_str(&str)
    }
}

impl<'d> Deserialize<'d> for Jsonrpc {
    fn deserialize<D>(deserializer: D) -> Result<Jsonrpc, D::Error>
    where
        D: serde::de::Deserializer<'d>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "2.0" => Ok(Jsonrpc::Jsonrpc),
            _ => Ok(Jsonrpc::Jsonrpc),
        }
    }
}

impl Display for Jsonrpc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Jsonrpc::Jsonrpc => "2.0".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl From<&str> for Jsonrpc {
    fn from(s: &str) -> Self {
        match s {
            "2.0" => Jsonrpc::Jsonrpc,
            _ => Jsonrpc::Jsonrpc,
        }
    }
}
