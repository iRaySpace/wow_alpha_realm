use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
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
    stream.write(b"127.0.0.1:8086").unwrap(); // redirection to world server
    stream.write(&[0]).unwrap();
}

fn handle_world(mut stream: TcpStream) {
    // probably: request for credentials
    stream
        .write(&[0x00, 0x08, 0xdd, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
        .unwrap();

    // credentials are read
    let mut buf = [0u8; 64];
    stream.read(&mut buf).unwrap();
    // println!("{:?}", String::from_utf8_lossy(&buf));

    // client responds "Success"
    stream.write(&[0x00, 0x05]).unwrap();
    stream.write(&[0xdf, 0x01]).unwrap();
    stream.write(&[0x00, 0x00]).unwrap();
    stream.write(&[0x0c]).unwrap();

    // seems the client sends protocol message for retrieving something?
    buf.iter_mut().for_each(|x| *x = 0);
    stream.read(&mut buf).unwrap();

    // probably a protocol to initiate sending back any character info
    stream.write(&[0x00, 0xa3]).unwrap();
    stream.write(&[0x3b, 0x00]).unwrap();
    stream.write(&[0x00, 0x00]).unwrap();

    // sending character information
    stream
        .write(&[
            0x01, 0x01, 0x00, 0x07, 0x40, 0x00, 0x00, 0x00, 0x00, 0x6d, 0x61, 0x67, 0x61, 0x00,
            0x01, 0x08, 0x00, 0x02, 0x01, 0x00, 0x00, 0x03, 0x01, 0x90, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48,
            0x43, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0xd8, 0x26, 0x00, 0x00, 0x04, 0x90, 0x27, 0x00, 0x00, 0x14,
            0x00, 0x00, 0x00, 0x00, 0x00, 0xd9, 0x26, 0x00, 0x00, 0x07, 0xda, 0x26, 0x00, 0x00,
            0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4a, 0x14, 0x00, 0x00, 0x15, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ])
        .unwrap();
}

fn main() {
    let realm_listener = TcpListener::bind("127.0.0.1:9100").unwrap();
    let proxy_listener = TcpListener::bind("127.0.0.1:9090").unwrap();
    let world_listener = TcpListener::bind("127.0.0.1:8086").unwrap();

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
    let world_thread = thread::spawn(move || {
        println!("World: Listening...");
        for stream in world_listener.incoming() {
            let stream = stream.unwrap();
            println!("World: New connection {}", stream.peer_addr().unwrap());
            handle_world(stream);
            println!("World: Stream handled...");
        }
    });

    realm_thread.join().unwrap();
    proxy_thread.join().unwrap();
    world_thread.join().unwrap();
}
