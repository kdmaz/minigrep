use std::{env, process};

use minigrep::Config;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let config = Config::new(args.iter()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
