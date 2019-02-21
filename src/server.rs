use crate::config::ServerConfig;
use crate::network::message::*;
use crate::network::Network;
use crate::structs::Map;

use enet::Enet;

pub struct Server {
    config: ServerConfig,
    map: Map,
    network: Network,
}

impl Server {
    fn lobby(&mut self) {
        let mut count = 0;
        while count < self.config.game.player_count {
            let mut msgs = self.network.update();
            while let Some((peer, msg)) = msgs.pop() {
                match msg {
                    Message::Initialize { nick } => count += 1,
                    Message::Disconnect => count -= 1,
                    _ => {}
                }
            }
        }
    }
    fn calculate(&mut self) {}
    fn send(&mut self) {}
    fn recive(&mut self) {}
}

pub fn run_server(server_config: ServerConfig, enet_handle: Enet) {
    let mut server = Server {
        map: Map::new(512, 512),
        network: Network::new(true, enet_handle).expect("Network for server not created"),
        config: server_config,
    };
    server.lobby();
    loop {
        server.send();
        server.recive();
        server.calculate();
    }
}
