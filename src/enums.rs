use teloxide::macros::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase")]
pub enum AdminCommands {
    Ban,
    Unban,
    Mute,
    Unmute,
}

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase")]
pub enum BotCommonCommands {
    Start,
    Help,
}

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase")]
pub enum DocsCommands {
    Rust,
    Csharp,
    Cpp,
    C,
    Java,
    Python,
    Kotlin,
    Javascript,
    Typescript,
    Php,
    Go,
    Ruby,
    Lua,
}

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase")]
pub enum FunCommands {
    Send,
}