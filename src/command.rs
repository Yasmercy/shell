use crate::data::*;

impl Command {
    pub fn new(cmd: &str) -> Self {
        let mut tokens = cmd.split_whitespace();

        Self {
            chain: None,
            synchronous: true,
            output: None,
            append: None,
            input: None,
            filename: tokens.next().unwrap().to_string(),
            args: tokens.into_iter().map(|x| x.to_string()).collect(),
        }
    }
}
