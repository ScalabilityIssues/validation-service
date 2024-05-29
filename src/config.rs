use serde::Deserialize;
use std::net::IpAddr;

fn default_ip() -> IpAddr {
    IpAddr::from([0, 0, 0, 0])
}

fn default_port() -> u16 {
    50051
}

fn default_path_signing_key_file() -> String {
    "sign.pem".to_string()
}

#[derive(Deserialize, Debug)]
pub struct Options {
    #[serde(default = "default_ip")]
    pub ip: IpAddr,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_path_signing_key_file")]
    pub signing_key_file: String,
    #[serde(default)]
    pub generate_signing_key: bool,
}
