use std::{net::TcpStream, io::{Result, Read, Write}, str::from_utf8};

pub fn read_data(mut stream: TcpStream) -> Result<(TcpStream, [u8; 64])> {
    let mut buf = [0; 64];
    match stream.read(&mut buf) {
        Ok(_) => Ok((stream, buf)),
        Err(_) => todo!(),
    }
}

pub fn read_full(mut stream: TcpStream, buf: &mut [u8]) -> Result<(TcpStream, &mut [u8])> {
    match stream.read_exact(buf) {
        Ok(_) => Ok((stream, buf)),
        Err(e) => Err(e),
    }
}

pub fn write_data(mut stream: TcpStream, data: &[u8]) -> Result<TcpStream> {
    match stream.write_all(data) {
        Ok(_) => Ok(stream),
        Err(e) => Err(e),
    }
}

pub fn print_data((stream, data): (TcpStream, [u8; 64])) -> Result<TcpStream> {
    println!("{}", from_utf8(&data).unwrap());
    Ok(stream)
}
