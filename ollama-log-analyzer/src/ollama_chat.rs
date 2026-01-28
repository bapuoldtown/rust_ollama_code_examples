use serde::{Deserialize, Serialize};
use std::time::Duration;
#[derive(Serialize, Debug)]
struct ChatMessage{
    role: String,
    content: String,

}

#[derive(Serialize, Debug)]
struct Options{
    num_predict: i32,
}
#[derive(Serialize, Debug)]
struct ChatRequest{
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
    options: Options,
}

#[derive(Deserialize, Debug)]
struct ChatMessageOut{
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct ChatResponse{
    message: ChatMessageOut,
}
pub fn chat_blocking(model: &str, messages: Vec<(String, String)>) ->Result<String, Box<dyn std::error::Error>>{
    let url = "http://127.0.0.1:11434/api/chat";
    let builder1 = reqwest::blocking::Client::builder();
    let builder2=builder1.no_proxy();
    let builder3 = builder2.timeout(Duration::from_secs(240));
    let client = builder3.build()?;
    let mut chat_messages = Vec::new();
    for (role, content) in messages{
        let m_struct: ChatMessage = ChatMessage{
            role,
            content,
        };
        chat_messages.push(m_struct);
    };
    let chat_request: ChatRequest = ChatRequest{
        model: model.to_string(),
        messages: chat_messages,
        stream: false,
        options: Options{
            num_predict: 1024
        },
    };

    let resp = client.post(url).json(&chat_request).send()?;
    let status = resp.status();
    let body_text = resp.text()?;
    if !status.is_success() {
        return Err(format!("Ollama error {}: {}", status, body_text).into());
    }
    let parsed_chat_response:ChatResponse = serde_json::from_str(&body_text)?;
    Ok(parsed_chat_response.message.content)


}