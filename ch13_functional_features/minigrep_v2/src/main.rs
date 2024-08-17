//////////////////////////////
// 13.3. Minigrep
// https://doc.rust-lang.org/stable/book/ch13-03-improving-our-io-project.html
// 
//////////////////////////////

use std::env;
use std::process;

use minigrep_v2::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep_v2::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
