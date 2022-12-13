use anyhow::{Ok, Result};
use chatgpt_rs::client::GPTClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Request {
    prompt: String,
    temperature: f64,
    max_tokens: usize,
    top_p: f64,
    frequency_penalty: f64,
    presence_penalty: f64,
}

pub async fn get_ai_message(input: String) -> Result<String> {
    let mut gpt_client = GPTClient::new()?;
    let resp = gpt_client.post(input.to_string()).await?;

    Ok(serde_json::from_str(&format!("\"{resp}\"")).unwrap())
}
