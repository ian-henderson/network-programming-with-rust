extern crate rand;
use rand::{thread_rng, Rng};
use std::{
    io::{Error, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

// handles a single client
fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);
    let mut buffer = [0; 512];

    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            return Ok(());
        }
        let sleep = Duration::from_secs(*thread_rng().choose(&[0, 1, 2, 3, 4, 5]).unwrap());
        println!("Sleeping for {:?} before replying", sleep);
        thread::sleep(sleep);
        stream.write(&buffer[..bytes_read])?;
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Could not bind");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error))
                });
            }
            Err(error) => {
                eprintln!("failed: {}", error)
            }
        }
    }
}
