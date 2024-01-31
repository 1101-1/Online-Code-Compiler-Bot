use std::env;

use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::json;
use teloxide::{requests::Requester, types::Message, Bot};

use crate::types::{
    json_response::GeminiResult,
    state::{HandlerResult, MyDialogue},
};

use super::{other_code, rust_code};

pub async fn auto_compile(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    if let Some(text) = msg.text() {
        let text = text.to_string();
        let code_info = gemini_autodetect(text.clone()).await;
        if code_info == "C++" {
            bot.send_message(msg.chat.id, "Defined lang is C++. Sending result..")
                .await?;
            other_code::send_code(
                bot.clone(),
                msg.clone(),
                dialogue.clone(),
                "cpp".to_string(),
            )
            .await?;
        }
        if code_info == "C#" {
            bot.send_message(msg.chat.id, "Defined lang is C#. Sending result..")
                .await?;
            other_code::send_code(bot.clone(), msg.clone(), dialogue.clone(), "cs".to_string())
                .await?;
        }
        if code_info == "Java" || text.find("private static void").is_some() {
            bot.send_message(msg.chat.id, "Defined lang is Java. Sending result..")
                .await?;
            other_code::send_code(
                bot.clone(),
                msg.clone(),
                dialogue.clone(),
                "java".to_string(),
            )
            .await?;
        }
        if code_info == "Rust" {
            bot.send_message(msg.chat.id, "Defined lang is Rust. Sending result..")
                .await?;
            rust_code::send_code(bot.clone(), msg.clone(), dialogue.clone()).await?;
        }
        if code_info == "Go" {
            bot.send_message(msg.chat.id, "Defined lang is Go. Sending result..")
                .await?;
            other_code::send_code(bot.clone(), msg.clone(), dialogue.clone(), "go".to_string())
                .await?;
        }
        if code_info == "Python" {
            bot.send_message(msg.chat.id, "Defined lang is Python. Sending result..")
                .await?;
            other_code::send_code(bot.clone(), msg.clone(), dialogue.clone(), "py".to_string())
                .await?;
        }
        if code_info == "undefined" {
            bot.send_message(msg.chat.id, "Defined language compiler can't compile.")
                .await?;
            return Ok(());
        }
    }

    Ok(())
}

async fn gemini_autodetect(code: String) -> String {
    let gemini_api_key = env::var("API_KEY").expect("API_KEY does not set");
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}",
        gemini_api_key
    );
    let client = reqwest::Client::builder().build().unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    headers.insert(
        "User-Agent",
        HeaderValue::from_static(
            "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/115.0",
        ),
    );
    headers.insert(
        "Accept",
        HeaderValue::from_static(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
        ),
    );

    let json_data = json!({
        "contents":[{"parts":[{"text": format!("Here is a code:{}\n Write what language of this code are: Python, Go, C++, C#, Java, Rust. If type not in previous, write just `undefined`. Answer with one word only", code)}]}]
    });

    let query = serde_json::to_string(&json_data).unwrap();

    let code_type_info = client
        .post(url)
        .body(query)
        .headers(headers)
        .send()
        .await
        .unwrap()
        .json::<GeminiResult>()
        .await
        .unwrap();

    code_type_info.candidates[0].content.parts[0].text.clone()
}
