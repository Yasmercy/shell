#![allow(dead_code)]

use crate::data::*;
use std::{io::Write, path::PathBuf};

/// Re-exporting pid_t as isize
pub type Pid = isize;

/// Describing how to use the command line arguments
pub fn print_usage() {
    println!("cargo run shell [-f script_file.sh] [-h history_file.txt] [--help]");
}

/// User prompt before entering commands
pub fn print_prompt(dir: &PathBuf, pid: Pid) {
    print!("({})[{}]$", dir.display(), pid);
    std::io::stdout().flush().expect("flushing failed");
}

/// Displays a command to the user, with a newline
pub fn print_command(command: &Command) {}

/// Displays the promtp for an invalid command.
/// This happens during parsing.
pub fn print_invalid_command(command: &str) {}

/// Displays a command to the user,
/// after it has finished executing
pub fn print_command_executed(pid: Pid, command: &Command) {
    println!("({}) {} {:?}", pid, command.filename, command.args);
}

/// Error during the opening of a script file
/// that is first argument after -f
pub fn print_script_file_error() {}

/// Error during the opening of a script file
/// that is first argument after -h
pub fn print_history_file_error() {}

/// Error during an invalid cd directory.
/// Note that empty dirs are always valid.
pub fn print_no_directory(dir: &PathBuf) {}

/// When spawning a new process failed
pub fn print_exec_failed(command: &Command) {}

/// When setting the process group id fails
pub fn print_setpgid_failed() {}

/// Signaling to a child process that shell cannot find
pub fn print_no_process_found() {}

/// Successfully killed a child process
pub fn print_killed_process(pid: Pid, command: &Command) {}

/// Successfully stopped a child process
pub fn print_stopped_process(pid: Pid, command: &Command) {}

/// Successfully continued a child process
pub fn print_continued_process(pid: Pid, command: &Command) {}

/// Header before printing a group of processes
pub fn print_process_info_header() {}

/// Displays a process info, and a newline
pub fn print_process_info(pinfo: ProcessInfo) {}

/// Error during redirecting a file to another
pub fn print_redirection_file_error() {}
