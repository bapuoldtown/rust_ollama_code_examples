use serde::{Deserialize, Serialize};
use std::time::Duration;
//use struct for serializ and desrialize  thr request and response
#[derive(Serialize, Debug)]
struct Options{
    num_predict: i32,
}

#[derive(Serialize, Debug)]
struct GenerateRequest{
    model: String,
    prompt: String,
    stream: bool,
    options: Options,
}

#[derive(Deserialize, Debug)]
pub struct GenerateResponse {
    pub response: String,
}

pub fn generate_code_review(model: &str, prompt: &str) -> Result<GenerateResponse, Box<dyn std::error::Error>>{
    let url = "http://127.0.0.1:11434/api/generate";
    let builder1 = reqwest::blocking::Client::builder();
    let builder2 = builder1.no_proxy();
    let builder3 = builder2.timeout(Duration::from_secs(240));
    let client = builder3.build()?;
    let options = Options { num_predict: 250 };
    let request_body: GenerateRequest = GenerateRequest{
        model: model.to_string(),
        prompt: prompt.to_string(),
        stream: false,
        options,
    };
    let resp = client.post(url).json(&request_body).send()?;
    let body_text = resp.text()?;
    println!("Response Body Text: {}", body_text);

    let generate_response: GenerateResponse = serde_json::from_str(&body_text)?;
    Ok(generate_response)


}

