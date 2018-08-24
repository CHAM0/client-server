#[macro_use]
extern crate serde_derive;

extern crate tokio;
extern crate serde_json;


use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

use std::net::SocketAddr;


#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32
}

fn main() {

    let toto = Person {name: "jean".to_string(), age: 40};
    let tata = Person {name: "jean".to_string(), age: 40};

    let addr = "127.0.0.1:6142".parse::<SocketAddr>().unwrap();
    // let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8081);
   
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener.incoming()
        .map_err(|e| eprintln!("Error : {:?}", e))
        .for_each(move |socket| {
            println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

            let j = serde_json::to_string(&toto);
            let j2 = serde_json::to_string(&tata);

            let connection = io::write_all(socket, j.unwrap())
                .then(|res| {
                    println!("wrote message; success={:?}", res.is_ok());
                    Ok(())
                });

            // Spawn a new task that processes the socket:
            tokio::spawn(connection);
            Ok(())
        });

    tokio::run(server);
        
}
