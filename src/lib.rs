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
        let mut counter = 0;
        loop {
            thread::sleep(time::Duration::from_millis(1000));
            network.update();
            let message = network::message::Message::Ping{num: counter};
            network.send_message(message);
            counter += 1;
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