use std::net::TcpStream;

const LEN_OFFSET: usize = 4;
const READ_BUF_SIZE: usize = 4096;

enum State {
    REQ,
    RES,
    END,
}

pub struct Connection {
    stream: TcpStream,
    state: State,
    rbuf: [u8; LEN_OFFSET + READ_BUF_SIZE],
}
