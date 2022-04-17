#[derive(Clone)]
pub struct Command {
    pub name: String,
    pub command: String,
    pub args: String,
}

impl Command {
    pub fn new(name: String, command: String, args: String) -> Self {
        Self {
            name,
            command,
            args,
        }
    }
}