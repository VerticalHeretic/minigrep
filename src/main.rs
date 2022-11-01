use minigrep_lswarss::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep_lswarss::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
