//////////////////////////////
// 12.1. Minigrep
// https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html
// https://doc.rust-lang.org/stable/book/ch12-01-accepting-command-line-arguments.html
// https://doc.rust-lang.org/stable/book/ch12-02-reading-a-file.html
// https://doc.rust-lang.org/stable/book/ch12-03-improving-error-handling-and-modularity.html
// https://doc.rust-lang.org/stable/book/ch12-04-testing-the-librarys-functionality.html
// https://doc.rust-lang.org/stable/book/ch12-05-working-with-environment-variables.html
// https://doc.rust-lang.org/stable/book/ch12-06-writing-to-stderr-instead-of-stdout.html
//////////////////////////////

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // El primer argumento es el path del binario
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
