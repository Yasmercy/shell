mod builtins;
mod command;
mod data;
mod format;
mod history;
mod process;

use clap::*;
use std::path::PathBuf;
use crate::format::*;

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
#[clap(disable_help_flag = true)]
enum Command {
    Shell {
        #[clap(short = 'h')]
        history_file: Option<PathBuf>,

        #[clap(short = 'f')]
        script_file: Option<PathBuf>,

        #[clap(long)]
        help: bool,
    },
}

fn main() {
    let args = Args::parse();

    let Command::Shell {
        history_file,
        script_file,
        help,
    } = args.cmd;

    if help {
        print_usage();
        return;
    }

    if let Some(hfile) = history_file {
        println!("Got a history file of {}", hfile.display());
    }

    if let Some(sfile) = script_file {
        println!("Got a script file of {}", sfile.display());
    }
}
