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
        "meme" => client.meme(&args),
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
    Interactively authenticate with SMS.
cost pay|get
    Get the link to pay or get the account balance.
gene [fed FID] (meta GID|call GID [ARG])
    Get the metadata with GID, or call the gene with ARG.
    The Fed field is optional and defaults to the local instance.
meme (meta HASH|raw-put DAYS FILE|raw-get [-p] HASH [FILE])
    Get the metadata of the meme by HASH.
    Put the FILE as a meme, then keep DAYS days.
    Get meme by HASH, -p means public meme.
    Optionally save to FILE."
    );
}
