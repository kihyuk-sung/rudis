use std::{net::TcpStream, io::Read};

const LEN_OFFSET: usize = 4;
const READ_BUF_SIZE: usize = 4096;

pub enum State {
    TryReadHeader,
    TryReadData,
    REQ,
    RES,
    END,
}

pub struct Connection {
    stream: TcpStream,
    pub state: State,
    current_read_size: usize,
    read_size: usize,
    rbuf: [u8; LEN_OFFSET + READ_BUF_SIZE],
}

impl Connection {
    pub fn init(stream: TcpStream) -> Connection {
        stream.set_nonblocking(true).expect("Cannot set non-blocking");
        Connection { 
            stream: stream,
            state: State::TryReadHeader, 
            rbuf: [0; LEN_OFFSET + READ_BUF_SIZE],
            current_read_size: 0,
            read_size: 0,
        }
    }

    pub fn try_read_header(&mut self) {
        match self.stream.read(&mut self.rbuf[self.current_read_size..LEN_OFFSET]) {
            Ok(size) => {
                self.current_read_size += size;
                if self.current_read_size == LEN_OFFSET {
                    let read_size = u32::from_be_bytes(self.rbuf[0..LEN_OFFSET].try_into().unwrap()) as usize;
                    println!("size: {:?}", read_size);
                    self.read_size = read_size;
                    // NEXT_STATE
                    self.state = State::TryReadData;
                    self.current_read_size = 0;
                }
            }
            Err(_) => self.state = State::END,
        }
    }

    pub fn try_read_data(&mut self) {
        match self.stream.read(&mut self.rbuf[LEN_OFFSET + self.current_read_size..LEN_OFFSET + self.read_size]) {
            Ok(size) => {
                self.current_read_size += size;
                if self.current_read_size == self.read_size {
                    let data = match std::str::from_utf8(&self.rbuf[LEN_OFFSET..LEN_OFFSET + self.read_size]) {
                        Ok(s) => s,
                        Err(_) => {
                            self.state = State::END;
                            return;
                        },
                    };
                    println!("data: {:?}", data);

                    // NEXT STATE
                    self.state = State::END;
                    self.current_read_size = 0;
                }
            }
            Err(_) => self.state = State::END,
        }
    }
}
