use crate::data::*;
use crate::format::*;

use std::process;
use std::process::Child;

pub struct ShellErr {}

impl ProcessInfo {
    fn new(child: Option<Child>) -> Self {
        Self { child }
    }

    pub fn execute(cmd: Command) -> Result<Self, ShellErr> {
        // TODO: redirect stdin for cmd
        let mut command = process::Command::new(cmd.filename.clone());
        command.args(cmd.args.clone());

        if let Ok(child) = command.spawn() {
            let pid = child.id() as Pid;
            print_command_executed(pid);

            if cmd.synchronous {
                let output = child.wait_with_output().expect("failed to wait");
                print!("{}", String::from_utf8(output.stdout).unwrap());
                Ok(Self::new(None))
            } else {
                Ok(Self::new(Some(child)))
            }
        } else {
            print_exec_failed(&cmd);
            Err(ShellErr {})
        }
    }

    pub fn reap(processes: &mut Vec<Self>) {
        let _zombies = processes.extract_if(|x| x.terminated());
    }

    pub fn done(&self) -> bool {
        self.child.is_none()
    }

    pub fn terminated(&mut self) -> bool {
        self.child.as_mut().is_some_and(|c| c.try_wait().is_ok_and(|r| r.is_some()))
    }
}
