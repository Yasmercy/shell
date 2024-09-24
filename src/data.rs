use std::path::PathBuf;

/// private struct to represent logical chaining
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
    /// the command that executes THIS command
    /// does not contain the commands for chain
    pub command_str: String,
    /// optional chaining with && || ;
    pub chain: Option<(Logic, Box<Command>)>,
    /// whether this command should be in fg or bg
    pub synchronous: bool,
    /// a filename to output into
    pub output: PathBuf,
    /// a filename to append into
    pub append: PathBuf,
    /// a filename to get input from
    pub input: PathBuf,
}

/// A vector of stored commands in memory
pub struct History {
    pub history: Vec<Command>,
}

/// Storing all the active processes
pub struct ProcessInfo {}
