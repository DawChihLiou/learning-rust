use std::env;
use std::process;

use improving_our_io_project::Config;

fn main() {
    // unwrap `Ok` or handle error with "closure".
    // in the closure, print custom error message and exit the process.
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // `eprintln!` print errors to Standard Error.
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = improving_our_io_project::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
