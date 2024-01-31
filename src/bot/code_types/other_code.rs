use teloxide::{requests::Requester, types::Message, Bot};

use crate::types::{
    json_response::{OtherPlayGroundRequest, OtherPlayGroundResponse},
    state::{HandlerResult, MyDialogue, State},
};

pub async fn send_code(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    lang: String,
) -> HandlerResult {
    if let Some(text) = msg.text() {
        let client = reqwest::Client::new();
        let playground_req = serde_json::to_string(&OtherPlayGroundRequest {
            code: text.to_string(),
            codeld: None,
            input: String::from(""),
            language: lang,
        })
        .unwrap();

        let response = client
            .post("https://api2.sololearn.com/v2/codeplayground/v2/compile")
            .header("Content-Type", "application/json")
            .body(playground_req)
            .send()
            .await?
            .text()
            .await?;
        let playground_res: OtherPlayGroundResponse = serde_json::from_str(&response).unwrap();
        let result = playground_res.data.output;
        bot.send_message(msg.chat.id, format!("{}", result)).await?;
        bot.send_message(
            msg.chat.id,
            String::from(
                "You can send another one code. To cancel autodetecting just write /cancel",
            ),
        )
        .await?;
        dialogue.update(State::AutoCompile).await?;
        return Ok(());
    }

    bot.send_message(msg.chat.id, "Write a code, that you want to compile")
        .await?;
    Ok(())
}
