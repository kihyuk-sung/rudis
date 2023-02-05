use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, str::from_utf8};

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
    match listener.accept()
    .and_then(|(s, _)| Ok(one_request(s))) {
        Ok(()) => (),
        Err(_) => (),
    }
}

fn one_request(mut stream: TcpStream) {
    let mut buf = [0; 4 + MAX_SIZE];
    loop {
        match stream.read_exact(&mut buf[0..4])
        .and_then(|()| Ok(len(&buf[0..4])))
        .and_then(|l| match stream.read_exact(&mut buf[4..4 + l]) {
            Ok(()) => Ok(l),
            Err(e) => Err(e),
        })
        .and_then(|l| match from_utf8(&buf[4.. 4 + l]) {
            Ok(d) => Ok(d),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
        })
        .and_then(|d| {
            println!("clent send: {d}");
            Ok(d)
        })
        .and_then(|d| {
            match stream.write_all(&d.len().to_be_bytes()[4..8]) {
                Ok(()) => Ok(d),
                Err(e) => Err(e),
            }
        })
        .and_then(|d| stream.write_all(d.as_bytes())) {
            Ok(()) => (),
            Err(_) => return,
        }
    }
}

fn len(len: &[u8]) -> usize {
    usize::from_be_bytes([0, 0, 0, 0, len[0], len[1], len[2], len[3]])
}
