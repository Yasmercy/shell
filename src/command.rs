use std::collections::VecDeque;

use crate::data::*;
use crate::format::*;

impl Command {
    pub fn new(cmd: &str) -> Option<Self> {
        let mut tokens: VecDeque<_> = cmd.split_whitespace().collect();

        let synchronous = tokens.len() == 1 || *tokens.back().unwrap() != "&";
        if !synchronous {
            tokens.pop_back();
        }

        if tokens.len() == 0 {
            print_usage();
            return None;
        }

        let filename = tokens.pop_front().unwrap().to_string();
        let args = tokens.into_iter().map(|x| x.to_owned()).collect();

        Some(Self {
            chain: None,
            synchronous,
            output: None,
            append: None,
            input: None,
            filename,
            args,
        })
    }
}
