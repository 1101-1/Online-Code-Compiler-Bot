use teloxide::{requests::Requester, types::Message, Bot};

use crate::types::{state::{HandlerResult, MyDialogue, State}, json_response::{PythonPlayGroundRequest, PythonPlayGroundResponse}};

pub async fn send_code(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    if let Some(text) = msg.clone().text() {
        let client = reqwest::Client::new();
        let playground_req = serde_json::to_string(&PythonPlayGroundRequest {
            code: text.to_string(),
            codeld: None,
            input: String::from(""),
            language: String::from("py")
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
        let playground_res: PythonPlayGroundResponse= serde_json::from_str(&response).unwrap();
        let result = playground_res.data.output;
        bot.send_message(msg.chat.id, format!("{}", result)).await?;
        dialogue.exit().await?;
        return Ok(());
    }


    bot.send_message(msg.chat.id, "Write a code, that you want to compile").await?;
    dialogue.update(State::SendRustCode).await?;
    Ok(())
}
