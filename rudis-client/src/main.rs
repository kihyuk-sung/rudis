use std::{net::TcpStream, io::{Write, Result}};

fn main() {
    let addr = "127.0.0.1:1234";
    match TcpStream::connect(addr)
        .and_then(write_data) {
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
