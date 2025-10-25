use std::net::IpAddr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Peer {
    username: String,
    addr: IpAddr,
}
