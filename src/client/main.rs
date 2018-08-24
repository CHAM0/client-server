extern crate serde_json;

use std::net::TcpStream;
use std::io::{self, Read};
use serde_json::{Value, Error};


fn main() {

    let mut stream = TcpStream::connect("127.0.0.1:6142")
        .expect("Couldn't connect to the server...");
    stream.set_nonblocking(true).expect("set_nonblocking call failed");

    let mut buf = vec![];
    loop {
        match stream.read_to_end(&mut buf) {
            Ok(_) => break,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP
                
            }
            Err(e) => panic!("encountered IO error: {}", e),
        };
    };

    let s = String::from_utf8_lossy(&buf);
    let v: Value = serde_json::from_str(&s).unwrap();
    println!("result: {}", s);
    println!("bytes: {}", v["age"]);

}