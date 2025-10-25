use std::net::IpAddr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Peer {
    username: String,
    addr: IpAddr,
}
