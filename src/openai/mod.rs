use std::env::var;

use anyhow::{anyhow, Result};
use async_recursion::async_recursion;
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
    let data = post_data(&db, client, &headers, input, uin, 0).await?;
    Ok(data)
}

pub async fn get_sd_prompt(client: &reqwest::Client, input: &str) -> Result<String> {
    let input = input.trim();
    let headers = match openai_headers() {
        Ok(header) => header,
        Err(e) => {
            error!("openai 无法获取header，请检查配置是否正确");
            return Err(e);
        }
    };
    let content = format!("从我输入的话中提取关键词翻译成英文，你只回复英文单词，用逗号分隔，其他什么都不要说，如果有one替换成solo，包含动词，如果我的输入中有人物，男的回复man，女的回复girl。不要在结尾使用.
例子
我:下雨天在树下避雨的女孩
你:rainy day, sheltering under tree, girl, tree
我:拿着武士刀的男孩
你:holding katana,holding,katana,boy
我输入:{}",input);
    let body = json!(
     {
      "model": "gpt-3.5-turbo",
      "messages":[
        {"role":"user","content":content}
      ],
      "temperature": 1,
      "max_tokens":300,
    }
            );

    let data = client
        .post("https://api.openai.com/v1/chat/completions")
        .headers(headers.clone())
        .body(body.to_string())
        .send()
        .await? // 发送请求
        .text()
        .await?;

    match serde_json::from_str::<Value>(&data) {
        Ok(data) => {
            let prompt_tokens = data["usage"]["prompt_tokens"].as_i64().unwrap();
            let completion_tokens = data["usage"]["completion_tokens"].as_i64().unwrap();
            let total_tokens = data["usage"]["total_tokens"].as_i64().unwrap();
            info!("使用openai完成消息,prompt_tokens:{prompt_tokens},completion_tokens:{completion_tokens},total_tokens:{total_tokens}");
            let rsp = data["choices"][0]["message"]["content"]
                .as_str()
                .unwrap()
                .to_string();
            Ok(rsp)
        }
        Err(e) => {
            error!("解析openai消息失败,body:{data},error:{e}");
            Err(anyhow!(e))
        }
    }
}

#[async_recursion]
async fn post_data(
    db: &sled::Tree,
    client: &reqwest::Client,
    headers: &HeaderMap,
    input: &str,
    uin: i64,
    skip: usize,
) -> Result<String> {
    let mut body = json!(
     {
      "model": "gpt-3.5-turbo",
      "messages":[
      ],
      "temperature": 1,
      "max_tokens":1000,
    }
            );
    let messages = body["messages"].as_array_mut().unwrap();
    // 遍历数据库
    for i in db
        // 跳过前面的数据
        .iter()
        // 如果数据库长度小于25，那么从0开始，否则从数据库长度-25开始
        .skip((if db.len() < 25 { 0 } else { db.len() - 25 }) + skip)
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
        .headers(headers.clone())
        .body(body.to_string())
        .send()
        .await? // 发送请求
        .text()
        .await?;
    // 解析响应
    trace!("{data}");
    let index = db.len() + 1;
    match serde_json::from_str::<Value>(&data) {
        Ok(data) => {
            if !data["error"]["code"].is_null()
                && data["error"]["code"].as_str().unwrap() == "context_length_exceeded"
            {
                return post_data(db, client, headers, input, uin, skip + 1).await;
            }
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
#[tokio::test]
async fn get_sd_prompt_test() {
    super::log::init();
    let client = reqwest::Client::new();
    let data = get_sd_prompt(&client, "一个在海边的光头").await.unwrap();
    println!("{data}");
}
