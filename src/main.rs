use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn handle_realm(mut stream: TcpStream) {
    stream.write(&[1]).unwrap(); // protocol for realm list
    stream.write(b"Alpha").unwrap();
    stream.write(&[0]).unwrap();
    stream.write(b"127.0.0.1:9090").unwrap(); // redirection to proxy server
    stream.write(&[0]).unwrap();
    stream.write(&[0, 0, 0, 0]).unwrap(); // population of the server
}

fn handle_proxy(mut stream: TcpStream) {
    stream.write(b"127.0.0.1:8100").unwrap(); // redirection to world server
    stream.write(&[0]).unwrap();
}

fn main() {
    let realm_listener = TcpListener::bind("127.0.0.1:9100").unwrap();
    let proxy_listener = TcpListener::bind("127.0.0.1:9090").unwrap();

    let realm_thread = thread::spawn(move || {
        println!("Realm: Listening...");
        for stream in realm_listener.incoming() {
            let stream = stream.unwrap();
            println!("Realm: New connection {}", stream.peer_addr().unwrap());
            handle_realm(stream);
            println!("Realm: Stream handled...");
        }
    });
    let proxy_thread = thread::spawn(move || {
        println!("Proxy: Listening...");
        for stream in proxy_listener.incoming() {
            let stream = stream.unwrap();
            println!("Proxy: New connection {}", stream.peer_addr().unwrap());
            handle_proxy(stream);
            println!("Proxy: Stream handled...");
        }
    });

    realm_thread.join().unwrap();
    proxy_thread.join().unwrap();
}
