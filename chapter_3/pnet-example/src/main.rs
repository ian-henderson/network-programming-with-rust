extern crate pnet;
use pnet::{
    datalink::{self, channel, Channel::Ethernet},
    packet::{
        ethernet::{EtherTypes, EthernetPacket},
        ip::IpNextHeaderProtocols,
        ipv4::Ipv4Packet,
        tcp::TcpPacket,
        Packet,
    },
};
use std::env;

// handles a single ethernet packet
fn handle_packet(ethernet: &EthernetPacket) {
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => {
            let header = Ipv4Packet::new(ethernet.payload());

            if let Some(header) = header {
                match header.get_next_level_protocol() {
                    IpNextHeaderProtocols::Tcp => {
                        let tcp = TcpPacket::new(header.payload());

                        if let Some(tcp) = tcp {
                            println!(
                                "Got a TCP packet {}:{} to {}:{}",
                                header.get_source(),
                                tcp.get_source(),
                                header.get_destination(),
                                tcp.get_destination()
                            );
                        }
                    }
                    _ => println!("Ignoring non TCP packet"),
                }
            }
        }
        _ => println!("Ignoring non IPv4 packet"),
    }
}

fn main() {
    let interface_name = env::args().nth(1).unwrap();

    // get all the interfaces
    let interfaces = datalink::interfaces();

    // filter the list to find the given interface name
    let interface = &interfaces
        .into_iter()
        .filter(|interface| interface.name == interface_name)
        .next()
        .expect("Error getting interface");

    let (_tx, mut rx) = match channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(error) => panic!(
            "An error occurred when creating the datalink channel: {}",
            error
        ),
    };

    // loop over the packets arriving on the given interface
    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();
                handle_packet(&packet);
            }
            Err(error) => panic!("An error occurred while reading: {}", error),
        }
    }
}
