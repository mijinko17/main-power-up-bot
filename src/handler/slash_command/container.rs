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

#[cfg(test)]
mod tests {
    use super::*;

    use serenity::{
        async_trait, builder::CreateApplicationCommand,
        model::interactions::application_command::ApplicationCommandInteraction, prelude::Context,
    };

    #[derive(Debug)]
    struct SlashCommandA;

    #[async_trait]
    impl SlashCommand for SlashCommandA {
        fn name(&self) -> &'static str {
            "a"
        }
        async fn interact(&self, _ctx: &Context, _command: &ApplicationCommandInteraction) {}
        fn register<'a>(
            &self,
            command: &'a mut CreateApplicationCommand,
        ) -> &'a mut CreateApplicationCommand {
            command
        }
    }

    #[test]
    fn test_slash_command_container_add_command() {
        let mut container = SlashCommandContainer { commands: vec![] };
        assert!(container.commands.is_empty());
        container = container.add_command(SlashCommandA);
        assert_eq!(container.commands.len(), 1);
    }

    #[test]
    fn test_slash_command_container_get() {
        let container = SlashCommandContainer { commands: vec![] }.add_command(SlashCommandA);
        let command_a = container.get("a").unwrap();
        assert_eq!(command_a.name(), "a");
        let no_command = container.get("b");
        assert!(no_command.is_none());
    }
}
