use crate::data::*;
use crate::format::*;

use std::process;

impl ProcessInfo {
    pub fn execute(cmd: Command) -> Self {
        // TODO: redirect stdin for cmd
        let mut process = process::Command::new(cmd.filename.clone());
        print_command_executed(1337, &cmd);

        if cmd.synchronous {
            if let Ok(output) = process.args(cmd.args).output() {
                println!("{}", String::from_utf8(output.stdout).unwrap());
                return Self {};
            } else {
                // command failed
            }
            // call a cleanup function

            // // check if need to redirect stdout
            // if cmd.output.is_some() || cmd.append.is_some() {
            //     // redirect cmd.stdout
            // }
        }

        // TODO: do the chaining here
        todo!()
    }
}
