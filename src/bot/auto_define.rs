use teloxide::{requests::Requester, types::Message, Bot};

use crate::types::state::{HandlerResult, MyDialogue};

use super::other_code;

pub async fn auto_compile(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    if let Some(text) = msg.text() {
        let text = text.to_string();
        if text.find("#include <iostream>").is_some() {
            bot.send_message(msg.chat.id, "Defined lang is C++. Sending result..")
                .await?;
            other_code::send_code(
                bot.clone(),
                msg.clone(),
                dialogue.clone(),
                "cpp".to_string(),
            )
            .await?;
        } else if text.find("using System;").is_some() || text.find("namespace").is_some() {
            bot.send_message(msg.chat.id, "Defined lang is C#. Sending result..")
                .await?;
            other_code::send_code(bot.clone(), msg.clone(), dialogue.clone(), "cs".to_string())
                .await?;
        } else if text.find("public static void").is_some() || text.find("private static void").is_some() {
            bot.send_message(msg.chat.id, "Defined lang is Java. Sending result..")
                .await?;
            other_code::send_code(
                bot.clone(),
                msg.clone(),
                dialogue.clone(),
                "java".to_string(),
            )
            .await?;
        } else if text.find("func").is_some() {
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
