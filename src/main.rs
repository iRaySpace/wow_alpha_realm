use std::net::{TcpListener, TcpStream};
use std::io::{Write};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:9100").unwrap();
    for stream in listener.incoming() {
        handle_stream(stream.unwrap());
    }
}

fn handle_stream(mut stream: TcpStream) {
    stream.write(&[1]).unwrap(); // protocol for realm list
    stream.write(b"Alpha").unwrap();
    stream.write(&[0]).unwrap();
    stream.write(b"0.0.0.0:9090").unwrap();
    stream.write(&[0]).unwrap();
    stream.write(&[0, 0, 0, 0]).unwrap(); // population of the server
}
