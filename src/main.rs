use std::env;

use dotenv::dotenv;
use teloxide::{dispatching::UpdateFilterExt, dptree};
use teloxide::{
    dispatching::{dialogue::InMemStorage, HandlerExt},
    prelude::Dispatcher,
};
use teloxide::{
    types::{Message, Update},
    Bot,
};

use crate::bot::command::{command_handler, invalid_command, Command};
use crate::bot::{python_code, rust_code};
use crate::types::state::State;

mod bot;
mod types;

#[tokio::main]
async fn main() {
    dotenv().ok();

    pretty_env_logger::init();

    log::info!("Starting bot");

    let bot = Bot::new(env::var("BOT_TOKEN").unwrap());

    let handler = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<State>, State>()
        .branch(
            dptree::case![State::HandleCommand]
                .branch(
                    dptree::entry()
                        .filter_command::<Command>()
                        .endpoint(command_handler),
                )
                .branch(dptree::endpoint(invalid_command)),
        )
        .branch(
            dptree::case![State::SendPythonCode]
                .branch(
                    dptree::entry()
                        .filter_command::<Command>()
                        .endpoint(command_handler),
                )
                .branch(dptree::endpoint(python_code::send_code)),
        )
        .branch(
            dptree::case![State::SendRustCode]
                .branch(
                    dptree::entry()
                        .filter_command::<Command>()
                        .endpoint(command_handler),
                )
                .branch(dptree::endpoint(rust_code::send_code)),
        );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
