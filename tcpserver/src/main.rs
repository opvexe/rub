use std::net::TcpListener;

#[warn(unused_variables)]
fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000...");

    for stream in listener.incoming() {
        let _s = stream.unwrap();
        println!("Connect ....")
    }
}
