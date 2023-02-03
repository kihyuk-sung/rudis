use std::{net::{TcpListener, TcpStream, SocketAddr}, io::{Result, Read, Write}};

fn main() {
    println!("-- rudis server --");

    let address = "0.0.0.0:1234";

    match TcpListener::bind(address)
        .and_then(|l| l.accept())
        .and_then(read_data)
        .and_then(print_data)
        .and_then(write_data) {
            Ok(_) => println!("ok"),
            Err(e) => println!("{}", e),
        }
}

fn read_data((mut stream, _addr): (TcpStream, SocketAddr)) -> Result<(TcpStream, [u8; 64])> {
    let mut buf = [0; 64];
    match stream.read(&mut buf) {
        Ok(_) => Ok((stream, buf)),
        Err(_) => todo!(),
    }
}

fn print_data((stream, data): (TcpStream, [u8; 64])) -> Result<TcpStream> {
    println!("{:?}", data);
    Ok(stream)
}

fn write_data(mut stream: TcpStream) -> Result<usize> {
    stream.write("world".as_bytes())
}
