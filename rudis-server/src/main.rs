use std::net::TcpListener;

fn main() {
    println!("-- rudis server --");

    let address = "0.0.0.0:1234";
    match TcpListener::bind(address) {
        Ok(_) => println!("bind {address}"),
        Err(_) => println!("cannot bind {address}"),
    }
}
