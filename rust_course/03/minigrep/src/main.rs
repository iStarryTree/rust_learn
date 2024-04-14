use std::env;

use minigrep::Config;

fn main() {
    // let args:Vec<String> = env::args().collect();
    let args = env::args().collect::<Vec<String>>();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    println!("Searching for {} in {}", config.query, config.file_path);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        std::process::exit(1);
    }
}
