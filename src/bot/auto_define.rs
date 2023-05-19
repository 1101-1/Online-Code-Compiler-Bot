use teloxide::{requests::Requester, types::Message, Bot};

use crate::types::state::{HandlerResult, MyDialogue};

use super::other_code;

pub async fn auto_compile(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    if let Some(text) = msg.text() {
        let text = text.to_string();
        if text.contains("#include <iostream>") {
            bot.send_message(msg.chat.id, "Defined lang is C++. Sending result..")
                .await?;
            other_code::send_code(
                bot.clone(),
                msg.clone(),
                dialogue.clone(),
                "cpp".to_string(),
            )
            .await?;
        } else if text.contains("using System;") || text.contains("namespace") {
            bot.send_message(msg.chat.id, "Defined lang is C#. Sending result..")
                .await?;
            other_code::send_code(bot.clone(), msg.clone(), dialogue.clone(), "cs".to_string())
                .await?;
        } else if text.contains("public static void") || text.contains("private static void") {
            bot.send_message(msg.chat.id, "Defined lang is Java. Sending result..")
                .await?;
            other_code::send_code(
                bot.clone(),
                msg.clone(),
                dialogue.clone(),
                "java".to_string(),
            )
            .await?;
        } else if text.contains("func") {
            bot.send_message(msg.chat.id, "Defined lang is Go. Sending result..")
                .await?;
            other_code::send_code(bot.clone(), msg.clone(), dialogue.clone(), "go".to_string())
                .await?;
        } else {
            bot.send_message(msg.chat.id, "Defined lang is Python. Sending result..")
                .await?;
            other_code::send_code(bot.clone(), msg.clone(), dialogue.clone(), "py".to_string())
                .await?;
        }
    }

    Ok(())
}
