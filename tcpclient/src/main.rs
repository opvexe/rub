use std::net::{TcpListener, TcpStream};

#[warn(unused_variables)]
#[warn(unused_imports)]
fn main() {
    let _stream = TcpStream::connect("127.0.0.1:3000").unwrap();
}
