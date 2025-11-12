use super::Command;

pub struct CommandFilter;

impl CommandFilter {
    pub fn filter_commands(commands: &[Command], query: &str) -> Vec<Command> {
        if query.is_empty() {
            return commands.to_vec();
        }

        let query_lower = query.to_lowercase();
        commands
            .iter()
            .filter(|cmd| cmd.name.to_lowercase().contains(&query_lower))
            .cloned()
            .collect()
    }
}