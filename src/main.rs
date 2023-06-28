use reqwest::{blocking::get, Error};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Print help if no argument supplied.
    if args.len() == 1 {
        eprint_help();
        process::exit(1);
    }

    // Match commands.
    let result = match args[1].as_str() {
        "ping" => ping(),
        _ => {
            eprint_help();
            process::exit(1);
        }
    };

    // Print response or error.
    match result {
        Ok(s) => println!("{}", s),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
}

/// Print help information to stderr.
fn eprint_help() {
    eprintln!(
        "usage: vc COMMAND ...
Commands:
ping: ping the server"
    );
}

/// Check connectivity.
fn ping() -> Result<String, Error> {
    get("http://localhost:8080")?.text()
}
