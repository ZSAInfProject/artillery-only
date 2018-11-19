mod network;

pub fn run(){
    let server = false;
    if(server){
        let mut network = network::Network::new(true).expect("test");
        loop {
            network.send_message(b"String");
            network.update();
        }
    }
    else{
        let mut network = network::Network::new(false).expect("test");
        network.connect();
        loop {
            network.update();
        }
    }
}