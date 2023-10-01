use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let envs: Vec<String> = env::args().collect();
    let config = Config::build(envs.into_iter()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(&config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
