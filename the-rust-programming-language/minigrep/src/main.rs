use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // unwrap `Ok` or handle error with "closure".
    // in the closure, print custom error message and exit the process.
    let config = Config::new(&args).unwrap_or_else(|err| {
        // `eprintln!` print errors to Standard Error.
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
