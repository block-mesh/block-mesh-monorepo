use ai_interface::models::base::ModelName;
use anyhow::anyhow;
use openai_api_rust::chat::{ChatApi, ChatBody};
use openai_api_rust::{Auth, Message, OpenAI, Role};

pub async fn ask(question: String, model_name: ModelName) -> anyhow::Result<String> {
    let auth = Auth::from_env().map_err(|_| anyhow!("Cannot find OpenAI envar"))?;
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
    let body = ChatBody {
        model: model_name.to_string(),
        max_tokens: None,
        temperature: Some(0_f32),
        top_p: Some(0_f32),
        n: Some(1),
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
    let choices = rs.unwrap().choices;
    let message = &choices[0].message.as_ref().ok_or(anyhow!("No message"))?;
    Ok(message.content.clone())
}
