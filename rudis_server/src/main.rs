use std::{net::{TcpListener, TcpStream}, io::Result, str::from_utf8};

use rudis::stream::{read_full, write_data};

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

const MAX_SIZE: usize = 4096;
fn accept(listener: &TcpListener) {
    let mut buf = [0; 4 + MAX_SIZE];
    match listener.accept()
        .and_then(|(stream, _addr)| read_full(stream, &mut buf[0..4]))
        .and_then(data_len)
        .and_then(|(stream, len)| read_full(stream, &mut buf[4..4 + len]))
        .and_then(print_data)
        .and_then(|(stream, buf)| write_data(stream, buf)) {
            Ok(_) => (),
            Err(_) => (),
        }
}

fn data_len((stream, buf): (TcpStream, &[u8])) -> Result<(TcpStream, usize)> {
    Ok(
        (stream, usize::from_be_bytes([0, 0, 0, 0, buf[0], buf[1], buf[2], buf[3]]))
    )
}

pub fn print_data((stream, data): (TcpStream, &[u8])) -> Result<(TcpStream, &[u8])> {
    println!("{}", from_utf8(data).unwrap());
    Ok((stream, data))
}
