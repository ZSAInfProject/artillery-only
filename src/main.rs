use artillery_only::{self, Config};
use std::env;

fn main() {
    let args = env::args();
    let config = Config::new(args).unwrap();
    artillery_only::run(config);
}
