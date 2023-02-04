use std::{net::TcpStream};
use rudis::stream::{read_data, write_data, print_data};

fn main() {
    let addr = "127.0.0.1:1234";
    match TcpStream::connect(addr)
        .and_then(|stream| write_data(stream, "hello".as_bytes()))
        .and_then(read_data)
        .and_then(print_data) {
            Ok(_) => println!("ok"),
            Err(_) => println!("not ok"),
        }
}
