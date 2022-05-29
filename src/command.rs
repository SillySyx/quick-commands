use std::process::Command;
use std::thread::spawn;

pub fn execute_command(command: &str, args: &[String]) {
    spawn({
        let command = command.to_owned();
        let args = args.to_owned();
        move || {
            let mut cmd = Command::new(command);

            if !args.is_empty() {
                cmd.args(args);
            }
        
            let _ = cmd.output();
        }
    });
}