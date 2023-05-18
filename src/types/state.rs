use teloxide::{dispatching::dialogue::InMemStorage, prelude::Dialogue};

#[derive(Clone, Default)]
pub enum State {
    #[default]
    HandleCommand,
    RecieveLang,
    SetLangCode {
        lang: String,
    },
    SendRustCode,
}

pub type MyDialogue = Dialogue<State, InMemStorage<State>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
