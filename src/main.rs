mod admin;
mod entities;
mod ip;
mod member;

fn main() {
    println!("{:?}", ip::get_local_ipv4());
}
