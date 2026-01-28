use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Debug)]
struct Options {
    num_predict: i32,
}

#[derive(Serialize, Debug)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    options: Options,
}

#[derive(Deserialize, Debug)]
struct OllamaResponse {
    response: String,
}

fn generate_blocking(model: &str, prompt: &str) -> Result<OllamaResponse, Box<dyn std::error::Error>> {
    let url = "http://127.0.0.1:11434/api/generate";

    let client = reqwest::blocking::Client::builder()
        .no_proxy()
        .timeout(Duration::from_secs(180)) // increase for slow first load
        .build()?;

    let request_body = OllamaRequest {
        model: model.to_string(),
        prompt: prompt.to_string(),
        stream: false,
        options: Options { num_predict: 250 }, // keep it short
    };

    let resp = client.post(url).json(&request_body).send()?;

    //if !resp.status().is_success() {
        //let status = resp.status();
        //let body_text = resp.text().unwrap_or_else(|_| "<failed to read body>".to_string());
        //return Err(format!("Ollama error {}: {}", status, body_text).into());
    //}

    let data: OllamaResponse = resp.json()?;
    Ok(data)
}

fn main() {
    let model = "llama3.1:latest";
    let prompt = "Say hello in one line.";
    let prompt1 = "Write a short note on rust programming language";

    let result = generate_blocking(model, prompt1);

    match result {
        Ok(res) => println!("Ollama Response: {:?}", res.response),
        Err(e) => {
            eprintln!("Error (Display): {}", e);
            eprintln!("Error (Debug):   {:?}", e);
        }
    }
}
