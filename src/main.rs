mod models;
mod drawing;
mod errors;
mod cli;
mod application;

use std::process;

use crate::cli::start_cli;

fn main() {
    if let Err(e) = start_cli() {
        eprintln!("{}", e);
        process::exit(e.exit_code());
    }
}
