
pub mod message;

use std::error::Error;
use enet::*;
use std::net::Ipv4Addr;
use self::message::*;
use bincode::{deserialize, serialize};

pub struct Network {
    enet: Enet,
    host: Host<()>,
}

impl Network {
    pub fn new(is_server: bool) -> Result<Network, Box<dyn Error>> {
        let enet = Enet::new().expect("Could not initialize ENet");
        let network = Network{
            host: 
            if is_server {
                let local_addr = Address::new(Ipv4Addr::LOCALHOST, 9001);
                enet.create_host::<()>(
                    Some(&local_addr),
                    10,
                    ChannelLimit::Maximum,
                    BandwidthLimit::Unlimited,
                    BandwidthLimit::Unlimited,
                )
                .expect("could not create host")
            }
            else{
                enet.create_host::<()>(
                    None,
                    10,
                    ChannelLimit::Maximum,
                    BandwidthLimit::Unlimited,
                    BandwidthLimit::Unlimited,
                )
                .expect("could not create host")
            },
            enet
        };
        Ok(network)
    }

    pub fn update(&mut self){
        match self.host.service(0).expect("service failed") {
            Some(Event::Connect(_)) => println!("new connection!"),
            Some(Event::Disconnect(..)) => println!("disconnect!"),
            Some(Event::Receive {
                channel_id,
                ref packet,
                ..
            }) => {
                let decoded: Message = deserialize(&packet.data()).unwrap();
                match decoded {
                    Message::Ping{num} => println!("Data: {}", num),
                    _ => (),
                }
            }
            _ => (),
        }
    }

    pub fn send_message(&mut self, message: Message){
        let encoded: Vec<u8> = serialize(&message).unwrap();
        for mut peer in self.host.peers() {
            let packet = Packet::new(&encoded, PacketMode::ReliableSequenced).unwrap();
            peer.send_packet(
            packet,
            1,
            );
        }
    }

    pub fn connect(&mut self){
        self.host.connect(&Address::new(Ipv4Addr::LOCALHOST, 9001), 10, 0)
            .expect("connect failed");
        loop {
            match self.host.service(1000).expect("service failed") {
                Some(Event::Connect(_)) => break,
                _ => continue,
            }
        }
    }
}