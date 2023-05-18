use teloxide::{requests::Requester, types::Message, Bot};

use crate::types::state::{HandlerResult, MyDialogue, State};

pub async fn send_code(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, "Doesn't supported. Check /rust").await?;
    dialogue.exit().await?;
    Ok(())
}
