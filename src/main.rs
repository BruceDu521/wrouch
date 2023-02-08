#![feature(file_set_times)]
use clap::Parser;

mod cli;

use cli::{Executor, Cli};

fn main() {
    let cli = Cli::parse();
    let exc = Executor::with_cli(cli);
    exc.execute();
}
