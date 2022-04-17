use std::process::Command as StdCommand;

pub fn execute_command(command: &str, args: &str) {
    let mut cmd = StdCommand::new(command);

    if !args.is_empty() {
        let args = args.split(" ");
        cmd.args(args);
    }

    let _ = cmd.spawn();
}