use artillery_only::{self, structs::Map, Config};
use std::io::prelude::*;
use std::{env, process};

fn main() {
    let args = env::args();

    let config = Config::new(args).unwrap();
    artillery_only::run(config);
}
