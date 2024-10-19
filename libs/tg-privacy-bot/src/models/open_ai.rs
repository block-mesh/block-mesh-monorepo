use anyhow::{anyhow, Context};
use openai_api_rust::chat::{ChatApi, ChatBody};
use openai_api_rust::{Auth, Message, OpenAI, Role};

pub async fn ask(question: String) -> anyhow::Result<String> {
    let auth = Auth::from_env().context("Failed to read authentication page")?;
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
    let body = ChatBody {
        model: "gpt-3.5-turbo".to_string(),
        max_tokens: Some(7),
        temperature: Some(0_f32),
        top_p: Some(0_f32),
        n: Some(2),
        stream: Some(false),
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        messages: vec![Message {
            role: Role::User,
            content: question,
        }],
    };
    let rs = openai.chat_completion_create(&body);
    let choice = rs.unwrap().choices;
    let message = &choice[0].message.as_ref().ok_or(anyhow!("No message"))?;
    Ok(message.content.clone())
}
