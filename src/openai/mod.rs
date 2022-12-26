use std::env::var;

use anyhow::{anyhow, Result};
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Value};
use tracing::{error, trace};

pub async fn get_ai_message(client: &reqwest::Client, input: &str, uin: i64) -> Result<String> {
    let headers = match openai_headers() {
        Ok(header) => header,
        Err(e) => {
            error!("openai 无法获取header，请检查配置是否正确");
            return Err(e);
        }
    };
    let body = json!(
     {
      "model": "text-davinci-003",
      "prompt": format!("我是小警察卸坤，一个可爱的人，可以回答任何问题，玩dota2，最终幻想14，喜欢回答游戏相关的问题。\n{uin}:{input}\n小警察卸坤:"),
      "temperature": 0.9,
      "max_tokens": 500,
      "top_p": 0.3,
      "frequency_penalty": 0.5,
      "presence_penalty": 0,
      "user": uin.to_string()
    }
            );
    let data = client
        .post("https://api.openai.com/v1/completions")
        .headers(headers)
        .body(body.to_string())
        .send()
        .await?
        .text()
        .await?;
    trace!("{data}");
    match serde_json::from_str::<Value>(&data) {
        Ok(data) => Ok(data["choices"][0]["text"].as_str().unwrap().to_string()),
        Err(e) => {
            error!("解析openai消息失败,body:{data},error:{e}");
            Err(anyhow!(e))
        }
    }
}
fn openai_headers() -> Result<HeaderMap> {
    let mut map = HeaderMap::new();
    map.insert(AUTHORIZATION, format!("Bearer {}", var("openai")?).parse()?);
    map.insert(CONTENT_TYPE, "application/json".parse()?);
    Ok(map)
}

#[tokio::test]
async fn test_model_list() {
    let client = reqwest::Client::new();
    let data = client
        .get("https://api.openai.com/v1/models")
        .header(AUTHORIZATION, format!("Bearer {}", var("openai").unwrap()))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{data}");
}
#[tokio::test]
async fn get_ai_message_test() {
    super::log::init();
    let client = reqwest::Client::new();
    let data = get_ai_message(&client, "你叫什么名字？", 110)
        .await
        .unwrap();
    println!("{data}");
}
