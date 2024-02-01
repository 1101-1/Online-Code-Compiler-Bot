use teloxide::payloads::SendMessageSetters;
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
    #[command(description = "Compile code on [Rust] lang")]
    Rust,
    #[command(description = "AI will define type of code. Send any code in available languages")]
    Autodetect,
    #[command(description = "Compile code on chosen lang([Go] [Java] [C++] [Python] [C#])")]
    OtherLang,
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
        Command::Autodetect => {
            dialogue.update(State::AutoCompile).await?;
            bot.send_message(msg.chat.id, "Send any available type of code")
                .await?
        }
        Command::Rust => {
            dialogue.update(State::SendRustCode).await?;
            bot.send_message(msg.chat.id, "Send rust code, that need to compile")
                .await?
        }
        Command::OtherLang => {
            dialogue.update(State::RecieveLang).await?;
            bot.send_message(msg.chat.id, "Send name of lang, that need to compile code")
                .await?;
            bot.send_message(msg.chat.id, "Available __Go__ __Java__ __С\\+\\+__ __Python__ __C\\#__").parse_mode(teloxide::types::ParseMode::MarkdownV2)
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
