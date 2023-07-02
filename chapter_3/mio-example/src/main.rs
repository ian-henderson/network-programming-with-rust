extern crate mio;

use mio::tcp::TcpListener;
use mio::*;

use std::{env, net::SocketAddr, process};

// This will later be used to identify the server on the event loop
const SERVER: Token = Token(0);

// Represents a simple TCP server using mio
struct TCPServer {
    address: SocketAddr,
}

impl TCPServer {
    fn new(port: u32) -> Self {
        TCPServer {
            address: format!("0.0.0.0:{}", port).parse().unwrap(),
        }
    }

    // Actually binds the server to a given address and runs it.
    // This function also sets up the event loop that dispatches events.
    // Later, we use a match on the token on the event to determine if the
    // event is for the server.
    fn run(&self) {
        let server = TcpListener::bind(&self.address).expect("Could not bind to port");

        let poll = Poll::new().unwrap();

        poll.register(&server, SERVER, Ready::readable(), PollOpt::edge())
            .unwrap();

        let mut events = Events::with_capacity(1024);

        loop {
            poll.poll(&mut events, None).unwrap();

            for event in events.iter() {
                match event.token() {
                    SERVER => {
                        let (_stream, remote) = server.accept().unwrap();
                        println!("Connection from {}", remote);
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Please provide only one port number as an argument");
        process::exit(1);
    }

    let server = TCPServer::new(args[1].parse().expect("Could not parse as u32"));

    server.run();
}
