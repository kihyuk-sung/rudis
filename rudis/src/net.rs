use std::{net::TcpStream, io::{Read, Write}};

const LEN_OFFSET: usize = 4;
const READ_BUF_SIZE: usize = 4096;

pub enum State {
    TryReadHeader,
    TryReadData,
    TryWriteData,
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
    current_write_size: usize,
    write_size: usize,
    wbuf: [u8; LEN_OFFSET + READ_BUF_SIZE],
}

impl Connection {
    pub fn init(stream: TcpStream) -> Connection {
        stream.set_nonblocking(true).expect("Cannot set non-blocking");
        Connection { 
            stream: stream,
            state: State::TryReadHeader, 
            rbuf: [0; LEN_OFFSET + READ_BUF_SIZE],
            wbuf: [0; LEN_OFFSET + READ_BUF_SIZE],
            current_read_size: 0,
            read_size: 0,
            current_write_size: 0,
            write_size: 0,
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
                    self.state = State::TryWriteData;
                    self.set_wbuf("world".as_bytes());
                }
            }
            Err(_) => self.state = State::END,
        }
    }

    pub fn try_write_data(&mut self) {
        match self.stream.write(&self.wbuf[0..self.write_size]) {
            Ok(size) => {
                self.current_write_size += size;
                if self.current_write_size == self.write_size {
                    println!("write: {}", self.write_size);
                    println!("send client: {:?}", std::str::from_utf8(&self.wbuf[LEN_OFFSET..self.write_size]).unwrap());
                    self.state = State::END;
                }
            },
            Err(_) => todo!(),
        }
    }

    pub fn set_wbuf(&mut self, data: &[u8]) {
        self.current_write_size = 0;
        self.write_size = LEN_OFFSET + data.len();
        self.wbuf[0..LEN_OFFSET].clone_from_slice(&(data.len() as u32).to_be_bytes());
        self.wbuf[LEN_OFFSET..LEN_OFFSET + data.len()].clone_from_slice(data);
    }
}
