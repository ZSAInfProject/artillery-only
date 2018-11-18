use std::error::Error;
use enet::*;
use std::net::Ipv4Addr;

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
}