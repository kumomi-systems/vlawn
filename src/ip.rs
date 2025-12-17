use std::{net::Ipv4Addr, str::FromStr};

pub fn get_local_ipv4() -> Ipv4Addr {
    let addr = String::from_iter(
        std::process::Command::new("hostname")
            .args(["-I"])
            .output()
            .unwrap()
            .stdout
            .iter()
            .map(|t| *t as char),
    )
    .replace(" \n", "");
    Ipv4Addr::from_str(&addr).unwrap()
}
