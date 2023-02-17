use std::{net::{TcpListener, TcpStream}, io::{Read, Write, Result, self}};

use rudis::net::{Connection, State};

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
    listener.set_nonblocking(true).expect("Cannot set non-blocking");

    let mut tasks = vec![];
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                tasks.push(Connection::init(stream));
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => (),
            Err(e) => panic!("encounterd IO error: {e}"),
        }

        while let Some(t) = tasks.pop() {
            match handle_task(t) {
                Some(t) => tasks.push(t),
                None => (),
            }
        }
    }

}

fn handle_task(mut task: Connection) -> Option<Connection> {
    match task.state {
        State::TryReadHeader => {
            task.try_read_header();
            Some(task)
        },
        State::TryReadData => {
            task.try_read_data();
            Some(task)
        }
        State::REQ => {
            Some(task)
        },
        State::RES => {
            Some(task)
        },
        State::END => None,
    }
}

const LEN_OFFSET: usize = 4;
const READ_BUF_SIZE: usize = 4096;
fn one_request(mut stream: TcpStream) -> Result<()> {
    let mut buf = [0; LEN_OFFSET + READ_BUF_SIZE];
    loop {
        one_query(&mut stream, &mut buf)?;
    }
}

fn one_query(stream: &mut TcpStream, buf: &mut [u8]) -> Result<()> {
    stream.read_exact(&mut buf[0..LEN_OFFSET])?;
    let read_len: usize = u32::from_be_bytes(buf[0..LEN_OFFSET].try_into().unwrap()) as usize;
    stream.read_exact(&mut buf[LEN_OFFSET..LEN_OFFSET + read_len])?;

    let data = match std::str::from_utf8(&buf[LEN_OFFSET..LEN_OFFSET + read_len]) {
        Ok(s) => s,
        Err(_) => "error data",
    };
    println!("client sent: {data}");

    stream.write_all(&("world".len() as u32).to_be_bytes())?;
    stream.write_all(&("world").as_bytes())?;

    Ok(())
}
