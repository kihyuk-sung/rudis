use std::{net::TcpStream, io::{Write, Result, Read}};

fn main() {
    let addr = "127.0.0.1:1234";
    match TcpStream::connect(addr)
        .and_then(write_data)
        .and_then(read_data) {
            Ok(_) => println!("ok"),
            Err(_) => println!("not ok"),
        }
}

fn write_data(mut stream: TcpStream) -> Result<TcpStream> {
    match stream.write("hello".as_bytes()) {
        Ok(_) => Ok(stream),
        Err(e) => Err(e),
    }
}

fn read_data(mut stream: TcpStream) -> Result<(TcpStream, [u8; 64])> {
    let mut buf = [0; 64];
    match stream.read(&mut buf) {
        Ok(_) => Ok((stream, buf)),
        Err(_) => todo!(),
    }
}
