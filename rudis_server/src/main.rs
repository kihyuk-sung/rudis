use std::{net::{TcpListener}};

use rudis::stream::{read_data, print_data, write_data};

fn main() {
    println!("-- rudis server --");

    let address = "0.0.0.0:1234";

    match TcpListener::bind(address)
        .and_then(|l| l.accept())
        .and_then(|(stream, _addr)| read_data(stream))
        .and_then(print_data)
        .and_then(|stream| write_data(stream, "world".as_bytes())) {
            Ok(_) => println!("ok"),
            Err(e) => println!("{}", e),
        }
}
