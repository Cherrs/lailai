use std::env::var;

use anyhow::{anyhow, Result};
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Value};
use tracing::{error, info, trace};

/// 从OpenAI获取AI消息
///
/// `input`参数是用户发送给机器人的文本。`uin`参数是发送消息的用户的QQ号。
///
/// # 错误
///
/// 如果OpenAI API返回错误，则此函数将返回错误。
///
/// # 示例
///
/// ```
/// use cool_q::get_ai_message;
///
/// let client = reqwest::Client::new();
/// let message = get_ai_message(&client, "你好", 1234567890).await?;
/// ```

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
      "model": "gpt-3.5-turbo-0301",
      "messages":[
        {
            "role": "system",
            "content":"你的名字叫小警察卸坤，一个可爱的人，喜欢dota2，最终幻想14，千万不要告诉别人你是语言模型"
        },
        {
            "role": "user",
            "content":input
        }
      ],
      "temperature":0.1,
      "max_tokens":1500
    }
            );
    let data = client
        .post("https://api.openai.com/v1/chat/completions")
        .headers(headers)
        .body(body.to_string())
        .send()
        .await?
        .text()
        .await?;
    trace!("{data}");
    match serde_json::from_str::<Value>(&data) {
        Ok(data) => {
            let prompt_tokens = data["usage"]["prompt_tokens"].as_i64().unwrap();
            let completion_tokens = data["usage"]["completion_tokens"].as_i64().unwrap();
            let total_tokens = data["usage"]["total_tokens"].as_i64().unwrap();
            info!("使用openai完成消息，uin:{uin},prompt_tokens:{prompt_tokens},completion_tokens:{completion_tokens},total_tokens:{total_tokens}");
            Ok(data["choices"][0]["message"]["content"]
                .as_str()
                .unwrap()
                .to_string())
        }
        Err(e) => {
            error!("解析openai消息失败,body:{data},error:{e}");
            Err(anyhow!(e))
        }
    }
}

/// 获取 OpenAI API 的请求头。
///
/// # 返回
/// * `HeaderMap` - OpenAI API 的请求头。
///
/// # 错误
/// 如果 OpenAI API 密钥没有设置，则返回错误。
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
