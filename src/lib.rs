extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate enet;
extern crate bincode;

use std::{thread, time};

mod network;

pub fn run(){
    let server = true;
    if server {
        let mut network = network::Network::new(true).expect("test");
        network.connect();
        loop {
            thread::sleep(time::Duration::from_millis(10));
            network.update();
            let message = network::message::Message::Ping{num: 5};
            network.send_message(message);
        }
    }
    else{
        let mut network = network::Network::new(false).expect("test");
        network.connect();
        loop {
            network.update();
            let message = network::message::Message::Ping{num: 10};
            network.send_message(message);
        }
    }
}