use teloxide::{payloads::SendMessageSetters, requests::Requester, types::Message, Bot};

use crate::types::{
    json_response::{RustPlayGroundRequest, RustPlayGroundResponse},
    state::{HandlerResult, MyDialogue, State},
};

pub async fn send_code(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    if let Some(text) = msg.text() {
        let client = reqwest::Client::new();
        let playground_req = serde_json::to_string(&RustPlayGroundRequest {
            code: text.to_string(),
            version: String::from("stable"),
            optimize: String::from("0"),
            test: false,
            separate_output: true,
            color: false,
            backtrace: String::from("0"),
        })
        .unwrap();

        let response = client
            .post("https://play.rust-lang.org/evaluate.json")
            .header("Content-Type", "application/json")
            .body(playground_req)
            .send()
            .await?
            .text()
            .await?;
        let playground_res: RustPlayGroundResponse = serde_json::from_str(&response).unwrap();
        let result = if let Some(result) = playground_res.result {
            result
        } else {
            playground_res.error.unwrap()
        };

        if result.is_empty() {
            bot.send_message(msg.chat.id, "_empty result_")
                .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                .await?;
            dialogue.exit().await?;
            return Ok(());
        }

        bot.send_message(msg.chat.id, format!("{}", result)).await?;
        dialogue.exit().await?;
        return Ok(());
    }

    bot.send_message(msg.chat.id, "Write a code, that you want to compile")
        .await?;
    dialogue.update(State::SendRustCode).await?;
    Ok(())
}
