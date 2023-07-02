use std::{net::UdpSocket, thread};

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8888").expect("Could not bind to socket");

    loop {
        let mut buffer = [0u8; 1500];
        let sock = socket.try_clone().expect("Failed to clone socket");

        match socket.recv_from(&mut buffer) {
            Ok((_, source)) => {
                thread::spawn(move || {
                    println!("Handling connection from {}", source);
                    sock.send_to(&buffer, &source)
                        .expect("Failed to send a response");
                });
            }
            Err(error) => {
                eprintln!("Couldn't receive a datagram: {}", error);
            }
        }
    }
}
