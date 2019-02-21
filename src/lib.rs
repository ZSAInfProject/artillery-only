mod client;
mod server;
use std::thread::{self, JoinHandle};

mod network;
pub mod structs;
use enet::Enet;

mod config;
pub use crate::config::{ClientConfig, Config, ServerConfig};

pub fn run(config: Config) {
    println!("{:#?}", config);

    let (client, server) = (config.client, config.server);
    let mut client_thread: Option<JoinHandle<_>> = None;
    let mut server_thread: Option<JoinHandle<_>> = None;

    let enet_client = Enet::new().expect("could not initialize enet");
    let enet_server = enet_client.clone();

    if let Some(_) = client {
        client_thread = Some(thread::spawn(move || client::run_client(enet_client)));
    }
    if let Some(server_config) = server {
        server_thread = Some(thread::spawn(move || {
            server::run_server(server_config, enet_server);
        }));
    }

    if let Some(client_thread) = client_thread.take() {
        client_thread.join().unwrap();
    }
    if let Some(server_thread) = server_thread.take() {
        server_thread.join().unwrap();
    }
}
