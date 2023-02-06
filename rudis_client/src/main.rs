use std::{net::TcpStream, io::{Result, Read, Write}};

fn main() {
    let addr = "127.0.0.1:1234";
    let mut stream = match TcpStream::connect(addr) {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e),
        };
    
    match query(&mut stream, "hello1")
        .and(query(&mut stream, "hello2"))
        .and(query(&mut stream, "hello3")) {
            Ok(()) => (),
            Err(e) => eprintln!("error: {:?}", e),
        };
}

const LEN_OFFSET: usize = 4;
const READ_BUF_SIZE: usize = 4096;
fn query(stream:&mut TcpStream, data: &str) -> Result<()> {
    let mut buf = [0; LEN_OFFSET + READ_BUF_SIZE];

    stream.write_all(&(data.len() as u32).to_be_bytes())?;
    stream.write_all(&data.as_bytes())?;

    stream.read_exact(&mut buf[0..LEN_OFFSET])?;
    let read_len: usize = u32::from_be_bytes(buf[0..LEN_OFFSET].try_into().unwrap()) as usize;
    stream.read_exact(&mut buf[LEN_OFFSET..LEN_OFFSET + read_len])?;
    
    let data = match std::str::from_utf8(&buf[LEN_OFFSET..LEN_OFFSET + read_len]) {
        Ok(s) => s,
        Err(_) => "error data",
    };

    println!("server sent: {data}");

    Ok(())
}
