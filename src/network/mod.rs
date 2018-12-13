pub mod message;

use self::message::*;
use bincode::{deserialize, serialize};
use enet::*;
use std::error::Error;
use std::net::Ipv4Addr;

#[derive(Debug, Clone)]
pub struct PeerData {
    initialized: bool,
    nick: Option<String>,
    id: Option<u32>,
    is_server: bool,
}

impl PeerData {
    pub fn new() -> PeerData {
        PeerData {
            initialized: false,
            nick: None,
            id: None,
            is_server: false,
        }
    }
    pub fn initialize(&mut self, nick: String, id: u32) {
        self.initialized = true;
        self.nick = Some(nick);
        self.id = Some(id);
    }
    pub fn server_initialize(&mut self) {
        self.initialized = true;
        self.nick = None;
        self.id = None;
        self.is_server = true;
    }
}

pub struct Network {
    enet: Enet,
    host: Host<PeerData>,
    lastID: u32,
    is_server: bool,
}

impl Network {
    pub fn new(is_server: bool) -> Result<Network, Box<dyn Error>> {
        let enet = Enet::new().expect("Could not initialize ENet");
        let network = Network {
            lastID: 0,
            host: if is_server {
                let local_addr = Address::new(Ipv4Addr::LOCALHOST, 9001);
                enet.create_host::<PeerData>(
                    Some(&local_addr),
                    10,
                    ChannelLimit::Maximum,
                    BandwidthLimit::Unlimited,
                    BandwidthLimit::Unlimited,
                )
                .expect("gkkgk")
            } else {
                enet.create_host::<PeerData>(
                    None,
                    10,
                    ChannelLimit::Maximum,
                    BandwidthLimit::Unlimited,
                    BandwidthLimit::Unlimited,
                )
                .expect("could not create host")
            },
            enet,
            is_server,
        };
        Ok(network)
    }

    pub fn update(&mut self) -> Vec<(PeerData, Message)> {
        let mut result: Vec<(PeerData, Message)> = Vec::new();
        match self.host.service(0).expect("service failed") {
            Some(Event::Connect(ref mut peer)) => {
                println!("new connection!");
                peer.set_data(Some(PeerData::new()));
            }
            Some(Event::Disconnect(ref peer, ..)) => {
                if let Some(data) = peer.data() {
                    result.push((data.clone(), Message::Disconnect));
                }
            }
            Some(Event::Receive {
                channel_id,
                ref mut sender,
                ref packet,
                ..
            }) => {
                let decoded: Message = deserialize(&packet.data()).unwrap();
                match &decoded {
                    Message::Initialize { nick } => {
                        if let Some(data) = sender.data_mut() {
                            data.initialize(nick.to_string(), 1);
                            if let Some(nick) = &data.nick {
                                println!("{} connected!", nick);
                            }
                            result.push((data.clone(), decoded));
                        } else {
                            sender.disconnect(0);
                        }
                    }
                    Message::Ping { num } => println!("Data: {}", num),
                    _ => {
                        if let Some(data) = sender.data() {
                            result.push((data.clone(), decoded));
                        }
                    }
                }
            }
            _ => (),
        }
        result
    }

    pub fn send_message(&mut self, message: Message) {
        let encoded: Vec<u8> = serialize(&message).unwrap();
        for mut peer in self.host.peers() {
            if let Some(data) = peer.data() {
                if (data.initialized) {
                    let packet = Packet::new(&encoded, PacketMode::ReliableSequenced).unwrap();
                    peer.send_packet(packet, 1);
                }
            }
        }
    }

    pub fn connect(&mut self) {
        self.host
            .connect(&Address::new(Ipv4Addr::LOCALHOST, 9001), 10, 0)
            .expect("connect failed");
        loop {
            match self.host.service(1000).expect("service failed") {
                Some(Event::Connect(ref mut peer)) => {
                    peer.set_data(Some(PeerData::new()));
                    if let Some(data) = peer.data_mut() {
                        data.server_initialize();
                    }
                    break;
                }
                _ => continue,
            }
        }
        let message = Message::Initialize {
            nick: "Omega".to_string(),
        };
        self.send_message(message);
    }
}
