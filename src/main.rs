use std::{env, process};
use voxov_cli::client::Client;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Print help if no argument supplied.
    if args.len() == 1 {
        eprint_help();
        process::exit(1);
    }

    // Init client
    let client = Client::default();

    // Match commands.
    let result = match args[1].as_str() {
        "ping" if args.len() == 2 => client.ping(),
        "auth" if args.len() == 2 => client.auth(),
        "cost" if args.len() == 3 => client.cost(&args[2]),
        "gene" => client.gene(&args),
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
ping
    Ping the server.
auth
    Interactively authenticate.
cost pay|get
    Pay money or get balance.
gene [fed FID] (meta GID|call GID ARG)
    Get the metadata of gene with gid, or call it with arg.
    Fed field is optional, defaults to the local instance.
meme (meta HASH|raw-put DAYS FILE|raw-get [-p] HASH)
    Get the metadata of meme by its hash.
    Put file as meme, keep some days.
    Get meme by hash, -p means public meme."
    );
}
