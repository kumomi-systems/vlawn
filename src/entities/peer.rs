use std::net::IpAddr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Peer {
    username: String,
    addr: IpAddr,
}

impl Peer {
    pub fn get_local() -> Self {
        Self {
            username: whoami::username(),
            addr: IpAddr::V4(crate::ip::get_local_ipv4()),
        }
    }

    pub fn addr(&self) -> &IpAddr {
        &self.addr
    }
}
