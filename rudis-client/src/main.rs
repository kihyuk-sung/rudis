use std::net::TcpStream;

fn main() {
    let addr = "127.0.0.1:1234";
    match TcpStream::connect(addr) {
            Ok(_) => println!("ok"),
            Err(_) => println!("not ok"),
        }
}
