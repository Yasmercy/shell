mod builtins;
mod command;
mod format;
mod history;
mod process;
mod data;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    // handle the -f and -h cases
    // ensure that valid args
    // https://rust-cli.github.io/book/tutorial/cli-args.html
}
