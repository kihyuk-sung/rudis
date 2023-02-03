use std::net::TcpListener;

fn main() {
    println!("-- rudis server --");

    let address = "0.0.0.0:1234";

    match TcpListener::bind(address)
        .and_then(|l| l.accept()) {
            Ok(_) => println!("ok"),
            Err(e) => println!("{}", e),
        }
}
