use super::SlashCommand;

#[derive(Default)]
pub struct SlashCommandContainer {
    pub commands: Vec<Box<dyn SlashCommand + Sync + Send>>,
}

impl SlashCommandContainer {
    pub fn add_command<T: 'static + SlashCommand + Send + Sync>(mut self, command: T) -> Self {
        self.commands.push(Box::new(command));
        self
    }
    #[allow(clippy::borrowed_box)]
    pub fn get(&self, name: &str) -> Option<&Box<dyn SlashCommand + Sync + Send>> {
        self.commands.iter().find(|command| command.name().eq(name))
    }
    pub fn iter(&self) -> impl Iterator<Item = &Box<dyn SlashCommand + Sync + Send>> {
        self.commands.iter()
    }
}
