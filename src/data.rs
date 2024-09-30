use std::{path::PathBuf, process::Child};

/// Re-exporting pid_t as isize
pub type Pid = isize;

/// private struct to represent logical chaining
/// these are all *left-associative*
pub enum Logic {
    /// A && B --- execute B only if A
    And,
    /// A || B --- execute B only if !A
    Or,
    /// A; B   --- execute B after A always
    Sep,
}

/// A fully parsed line from the user
/// which is implemented as a linked list
/// to support redirection and logical ops
pub struct Command {
    /// note: does not contain any info about chain
    /// the filename of the command (in PATH)
    pub filename: String,
    /// the arguments to pass into the cmd
    pub args: Vec<String>,
    /// optional chaining with && || ;
    pub chain: Option<(Logic, Box<Command>)>,
    /// whether this command should be in fg or bg
    pub synchronous: bool,
    /// a filename to output into
    pub output: Option<PathBuf>,
    /// a filename to append into
    pub append: Option<PathBuf>,
    /// a filename to get input from
    pub input: Option<PathBuf>,
}

/// A vector of stored commands in memory
pub struct History {
    pub history: Vec<Command>,
}

/// Storing all the active processes
pub struct ProcessInfo {
    // if no child, then this process is already done
    pub child: Option<Child>,
}
