use reqwest::{blocking::get, Error};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("{}", help());
        return;
    }
    println!(
        "{}",
        match args[1].as_str() {
            "ping" => ping(),
            _ => Ok(help()),
        }
        .unwrap_or_else(|e| format!("Error: {}", e))
    )
}

fn help() -> String {
    "usage: vc COMMAND ...
Commands:
ping: ping the server"
        .into()
}

fn ping() -> Result<String, Error> {
    get("http://localhost:8080")?.text()
}
