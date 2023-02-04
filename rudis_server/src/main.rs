use std::{net::{TcpListener}};

use rudis::stream::{read_data, print_data, write_data};

fn main() {
    println!("-- rudis server --");

    let address = "0.0.0.0:1234";

    let listener = match TcpListener::bind(address) {
        Ok(it) => it,
        Err(e) => {
            println!("cannot bind {e}");
            return
        },
    };

    loop {
        accept(&listener);
    }
}

fn accept(listener: &TcpListener) {
    match listener.accept()
        .and_then(|(stream, _addr)|read_data(stream))
        .and_then(print_data)
        .and_then(|stream| write_data(stream, "world".as_bytes())) {
            Ok(_) => (),
            Err(_) => (),
        }
}
