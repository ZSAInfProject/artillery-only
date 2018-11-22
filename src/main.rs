extern crate enet;
extern crate serde;
use artillery_only::structs::Map;
use std::{env, process};

fn main() {
    let mut args = env::args().skip(1);
    let run_as = args.next().unwrap_or_else(|| {
        println!("Usage: 'artillery-only client/server/both'");
        process::exit(1);
    });

    let args: Vec<String> = args.collect();

    println!("Artillery Only Game");

    match run_as.as_ref() {
        "client" => run_client(&args),
        "server" => run_server(&args),
        "both" => {
            let server_args = args.clone();
            let server = std::thread::spawn(move || run_server(&server_args));
            run_client(&args);
            server.join().unwrap();
        }
        _ => {
            println!("Invalid argument!");
            process::exit(1);
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
