extern crate enet;
extern crate serde;
use artillery_only::structs::Map;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Artillery Only Game");

    if args.len() == 1 {
        println!("Usage: 'artillery-only client/server'");
        return ();
    }

    match args[1].as_ref() {
        "client" => run_client(&args[1..].to_vec()),
        "server" => run_server(&args[1..].to_vec()),
        "both" => {
            let arg_ref = args[1..].to_vec();
            let server = std::thread::spawn(move || run_server(&arg_ref));
            run_client(&args[1..].to_vec());
            server.join().unwrap();
        }
        _ => {
            println!("Invalid argument!");
            return ();
        }
    }
}

fn run_client(args: &Vec<String>) {
    println!("client");
    let mut map = Map::new(1000, 1000);

    //TODO: Connecting and setting up rendering

    loop {
        // update() and render()
    }
}

fn run_server(args: &Vec<String>) {
    let mut map = Map::new(1000, 1000);

    //TODO: Connecting and setting up rendering

    loop {
        // update() and render()
    }
}
