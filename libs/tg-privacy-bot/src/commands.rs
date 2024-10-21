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
    #[command(description = "reset_on_each_message")]
    ResetOnEachMessage,
    #[command(description = "reset_on_model_change")]
    ResetOnModelChange,
    #[command(description = "reset")]
    Reset,
    #[command(description = "keep")]
    Keep,
    #[command(description = "select_model")]
    SelectModel,
    #[command(description = "info")]
    Info,
}
