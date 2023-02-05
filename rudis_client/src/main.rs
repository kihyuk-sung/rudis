use std::{net::TcpStream, io::{Result, Read}, str::from_utf8};
use rudis::stream::{write_data, read_full};

fn main() {
    let addr = "127.0.0.1:1234";
    match TcpStream::connect(addr)
        .and_then(|stream| query(stream, "hello1"))
        .and_then(|stream| query(stream, "hello2"))
        .and_then(|stream| query(stream, "hello3")) {
            Ok(_) => println!("ok"),
            Err(e) => println!("not ok {e}"),
        }
}

fn query(stream: TcpStream, data: &str) -> Result<TcpStream> {
    let mut buf = [0; 64];
    
    match write_data(stream, &data.len().to_be_bytes()[4..8])
        .and_then(|stream| write_data(stream, data.as_bytes()))
        .and_then(|stream| read_full(stream, &mut buf[0..4]))
        .and_then(data_len)
        .and_then(|(mut stream, len)| match stream.read_exact(&mut buf[4..4 + len]) {
            Ok(()) => Ok((stream, len)),
            Err(e) => Err(e),
        })
        .and_then(|(stream, l)| match from_utf8(&buf[4.. 4 + l]) {
            Ok(d) => Ok((stream, d)),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
        })
        .and_then(|(stream, d)| {
            println!("server sent: {d}");
            Ok(stream)
        }) {
            Ok(stream) => Ok(stream),
            Err(e) => Err(e),
        }
}

fn data_len((stream, buf): (TcpStream, &[u8])) -> Result<(TcpStream, usize)> {
    Ok(
        (stream, usize::from_be_bytes([0, 0, 0, 0, buf[0], buf[1], buf[2], buf[3]]))
    )
}
