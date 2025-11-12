#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    pub name: String,
    pub id: String,
}

impl Command {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
        }
    }
}

pub struct CommandRegistry {
    commands: Vec<Command>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        let commands = vec![
            Command::new("open_file", "Open File"),
            Command::new("save", "Save"),
            Command::new("save_as", "Save As"),
            Command::new("new_file", "New File"),
            Command::new("close_file", "Close File"),
        ];

        Self { commands }
    }

    pub fn all_commands(&self) -> &[Command] {
        &self.commands
    }

    pub fn find_by_id(&self, id: &str) -> Option<&Command> {
        self.commands.iter().find(|cmd| cmd.id == id)
    }
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}