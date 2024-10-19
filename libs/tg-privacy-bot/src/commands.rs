use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone, Default)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Commands {
    #[command(description = "Display Help.")]
    Help,
    #[default]
    #[command(
        description = "Start interaction with bot and another user.\n\t\t\tUsage: /start username"
    )]
    Start,
    #[command(description = "Search user notes.\n\t\t\tUsage: /ask question")]
    Ask,
}
