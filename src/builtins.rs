use crate::format::*;
use std::path::PathBuf;

pub enum Builtins {
    /// change directory
    Cd(PathBuf),
    /// search history by prefix
    HPrefix(String),
    /// search history by index
    HIndex(usize),
    /// list all current processes
    Ps,
    /// Kill signal to a process
    Kill(Pid),
    /// Stop signal to a process
    Stop(Pid),
    /// Cont signal to a process
    Cont(Pid),
    /// Exiting the shell
    Exit,
}
