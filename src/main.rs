#![feature(linux_pidfd)]
#![feature(extract_if)]

mod builtins;
mod command;
mod data;
mod format;
mod history;
mod process;

use crate::format::*;
use clap::*;
use data::ProcessInfo;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
    os::fd::{AsRawFd, FromRawFd, RawFd},
    path::PathBuf,
    str::FromStr,
};

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

fn mainloop(read: RawFd, write: Option<RawFd>) {
    let mut line = String::new();
    let mut processes = Vec::new();

    print_prompt(
        &std::env::current_dir().unwrap(),
        std::process::id() as isize,
    );
    let mut reader = unsafe { BufReader::new(File::from_raw_fd(read)) };

    while let Ok(num_bytes) = reader.read_line(&mut line) {
        if num_bytes == 0 {
            break;
        }

        if let Some(command) = data::Command::new(&line) {
            if let Ok(pinfo) = ProcessInfo::execute(command) {
                print_prompt(
                    &std::env::current_dir().unwrap(),
                    std::process::id() as isize,
                );

                if !pinfo.done() {
                    processes.push(pinfo);
                }
            }
        } else {
            print_usage();
        }

        line.clear();

        ProcessInfo::reap(&mut processes);
    }
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

    let reader = if let Some(sfile) = script_file {
        if let Ok(f) = File::open(sfile) {
            f.as_raw_fd()
        } else {
            io::stdin().as_raw_fd()
        }
    } else {
        io::stdin().as_raw_fd()
    };

    let writer = if let Some(hfile) = history_file {
        if let Ok(f) = File::open(hfile) {
            Some(f.as_raw_fd())
        } else {
            None
        }
    } else {
        None
    };

    mainloop(reader, writer);

    // cleanup here
    // TODO
}
