use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone, Default)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Commands {
    #[default]
    #[command(description = "Start bot")]
    Start,
    // #[command(description = "select_mode")]
    // SelectMode,
    #[command(description = "select_model")]
    SelectModel,
    #[command(description = "info")]
    Info,
    #[command(description = "Display Help.")]
    Help,
}
