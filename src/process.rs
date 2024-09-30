use crate::data::*;
use crate::format::*;

use std::process;

pub struct ShellErr {}

impl ProcessInfo {
    pub fn execute(cmd: Command) -> Result<Self, ShellErr> {
        // TODO: redirect stdin for cmd
        let mut command = process::Command::new(cmd.filename.clone());
        command.args(cmd.args.clone());

        if let Ok(child) = command.spawn() {
            print_command_executed(child.id() as Pid);

            if cmd.synchronous {
                let output = child.wait_with_output().expect("failed to wait");
                print!("{}", String::from_utf8(output.stdout).unwrap());
                Ok(Self {})
            } else {
                Ok(Self {})
            }
        } else {
            print_exec_failed(&cmd);
            Err(ShellErr {})
        }
    }
}
