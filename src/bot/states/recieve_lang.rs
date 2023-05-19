use teloxide::{requests::Requester, types::Message, Bot};

use crate::types::state::{HandlerResult, MyDialogue, State};

pub async fn code_type_update(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    if let Some(text) = msg.text() {
        match text.to_lowercase().as_str() {
            "python" | "py" => {
                dialogue
                    .update(State::SetLangCode {
                        lang: String::from("py"),
                    })
                    .await?;
                bot.send_message(msg.chat.id, "Selected lang is Python. Now send code")
                    .await?;
                return Ok(());
            }
            "go" => {
                dialogue
                    .update(State::SetLangCode {
                        lang: String::from("go"),
                    })
                    .await?;
                bot.send_message(msg.chat.id, "Selected lang is Go. Now send code")
                    .await?;
                return Ok(());
            }
            "java" => {
                dialogue
                    .update(State::SetLangCode {
                        lang: String::from("java"),
                    })
                    .await?;
                bot.send_message(msg.chat.id, "Selected lang is Java. Now send code")
                    .await?;
                return Ok(());
            }
            "c#" | "csharp" => {
                dialogue
                    .update(State::SetLangCode {
                        lang: String::from("cs"),
                    })
                    .await?;
                bot.send_message(msg.chat.id, "Selected lang is C#. Now send code")
                    .await?;
                return Ok(());
            }
            "cpp" | "c++" => {
                dialogue
                    .update(State::SetLangCode {
                        lang: String::from("cpp"),
                    })
                    .await?;
                bot.send_message(msg.chat.id, "Selected lang is C++. Now send code")
                    .await?;
                return Ok(());
            }
            _ => {
                bot.send_message(msg.chat.id, "Err. Try send name of lang")
                    .await?;
            }
        }
    }

    Ok(())
}
