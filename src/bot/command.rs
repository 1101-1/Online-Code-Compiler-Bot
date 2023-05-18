use teloxide::utils::command::BotCommands;
use teloxide::{requests::Requester, types::Message, Bot};

use crate::types::state::{HandlerResult, MyDialogue, State};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "Display all commands.")]
    Help,
    #[command(description = "Compile code on Python lang")]
    Python,
    #[command(description = "Compile code on Rust lang")]
    Rust,
    #[command(description = "Return to main menu")]
    Cancel,
}

pub async fn command_handler(
    bot: Bot,
    msg: Message,
    cmd: Command,
    dialogue: MyDialogue,
) -> HandlerResult {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Python => {
            dialogue.update(State::SendPythonCode).await?;
            bot.send_message(msg.chat.id, "Send python code, that need to compile")
                .await?
        }
        Command::Rust => {
            dialogue.update(State::SendRustCode).await?;
            bot.send_message(msg.chat.id, "Send rust code, that need to compile")
                .await?
        }
        Command::Cancel => {
            dialogue.exit().await?;
            bot.send_message(msg.chat.id, "Canceled\n/help").await?
        }
    };
    Ok(())
}

pub async fn invalid_command(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Please, send /help to show available commands")
        .await?;
    Ok(())
}
