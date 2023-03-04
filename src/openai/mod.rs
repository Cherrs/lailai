use std::env::var;

use anyhow::{anyhow, Result};
use once_cell::sync::OnceCell;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sled::Db;
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
///
pub static DB: OnceCell<Db> = OnceCell::new();

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Message {
    user: String,
    assistant: String,
}

pub async fn get_ai_message(
    client: &reqwest::Client,
    input: &str,
    uin: i64,
    group_code: i64,
) -> Result<String> {
    let input = input.trim();
    let headers = match openai_headers() {
        Ok(header) => header,
        Err(e) => {
            error!("openai 无法获取header，请检查配置是否正确");
            return Err(e);
        }
    };
    let db = DB.get_or_init(|| sled::open("db/openai").unwrap());
    let db = db.open_tree(group_code.to_be_bytes()).unwrap();
    if input == "重生" {
        db.clear()?;
        return Ok("重生成功".to_string());
    }
    let mut body = json!(
     {
      "model": "gpt-3.5-turbo-0301",
      "messages":[
        {
            "role": "system",
            "content":"你的名字叫小警察卸坤，一个可爱的人，喜欢dota2，最终幻想14，千万不要告诉别人你是语言模型"
        },
      ],
      "max_tokens":1500,
    }
            );
    let messages = body["messages"].as_array_mut().unwrap();
    // 遍历数据库
    for i in db
        // 跳过前面的数据
        .iter()
        // 如果数据库长度小于25，那么从0开始，否则从数据库长度-25开始
        .skip(if db.len() < 25 { 0 } else { db.len() - 25 })
    {
        // 从数据库中获取数据
        let messages_save = i?;
        // 将获取的数据转换为字符串
        let message: Message = bincode::deserialize(&messages_save.1)?;
        // 将获取的数据转换为JSON格式
        messages.push(json!({
            "role": "user",
            "content":message.user
        }));
        messages.push(json!({
            "role": "assistant",
            "content":message.assistant
        }));
    }
    messages.push(json!({
        "role": "user",
        "content":input
    }));
    println!("{}", body);
    let data = client
        .post("https://api.openai.com/v1/chat/completions")
        .headers(headers)
        .body(body.to_string())
        .send()
        .await? // 发送请求
        .text()
        .await?; // 解析响应
    trace!("{data}");
    let index = db.len() + 1;
    match serde_json::from_str::<Value>(&data) {
        Ok(data) => {
            let prompt_tokens = data["usage"]["prompt_tokens"].as_i64().unwrap();
            let completion_tokens = data["usage"]["completion_tokens"].as_i64().unwrap();
            let total_tokens = data["usage"]["total_tokens"].as_i64().unwrap();
            info!("使用openai完成消息，uin:{uin},prompt_tokens:{prompt_tokens},completion_tokens:{completion_tokens},total_tokens:{total_tokens}");
            let rsp = data["choices"][0]["message"]["content"]
                .as_str()
                .unwrap()
                .to_string();
            let msg = Message {
                assistant: rsp.clone(),
                user: input.to_string(),
            };
            let msg: Vec<u8> = bincode::serialize(&msg).unwrap();
            db.insert(index.to_be_bytes(), msg)?;
            Ok(rsp)
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
    let data = get_ai_message(&client, "你叫", 110, 15).await.unwrap();
    println!("{data}");
}
