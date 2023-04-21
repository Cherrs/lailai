use std::sync::Arc;

use anyhow::Result;
use futures_lite::stream::StreamExt;
use lapin::{
    self,
    options::{
        BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, BasicQosOptions,
        QueueBindOptions, QueueDeclareOptions,
    },
    types::FieldTable,
    BasicProperties, Connection, ConnectionProperties,
};
use ricq::{
    msg::{elem::At, MessageChain},
    Client,
};
use serde_json::json;
use tracing::{info, trace};

fn get_mq_addr() -> Result<String> {
    Ok(std::env::var("MQ_ADDR")?)
}

pub async fn start_consume(client: Arc<Client>) -> Result<()> {
    let addr = get_mq_addr()?;

    let uin = client.uin().await;

    let queue_name = format!("sd.callback.{}", uin);

    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    info!("图片MQ已连接");

    let queue = channel
        .queue_declare(
            &queue_name,
            QueueDeclareOptions {
                passive: false,
                durable: false,
                exclusive: true,
                auto_delete: true,
                nowait: false,
            },
            FieldTable::default(),
        )
        .await?;

    channel
        .queue_bind(
            &queue_name,
            "sd",
            &format!("sd.callback.{}", uin),
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;
    trace!(?queue, "声明队列成功{}", queue_name);

    channel.basic_qos(100, BasicQosOptions::default()).await?;
    trace!("Qos设置为100");

    let mut consumer = channel
        .basic_consume(
            &queue_name,
            "process",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery?;
        trace!("接收到图片");

        let header = delivery.properties.headers().clone().unwrap();
        let header = header.inner();

        let (send_to, from_uin, send_type) = (
            header["send_to"].as_long_long_int().unwrap(),
            header["from_uin"].as_long_long_int().unwrap(),
            header["send_type"].as_long_string().unwrap().to_string(),
        );

        let mut msg = MessageChain::default();

        match send_type.as_str() {
            "friend" => {
                trace!("来自好友的图片生成");
                let image: ricq::msg::elem::FriendImage =
                    client.upload_friend_image(send_to, &delivery.data).await?;
                msg.push(image);
                client.send_friend_message(send_to, msg).await?;
            }
            "group" => {
                trace!("来自群的图片生成");
                let image = client.upload_group_image(send_to, &delivery.data).await?;
                msg.push(image);
                msg.push(At::new(from_uin));
                client.send_group_message(send_to, msg).await?;
            }
            _ => {}
        }
        delivery.ack(BasicAckOptions::default()).await?;
    }

    Ok(())
}
/// 发送图片生成请求
/// TODO 使用
pub async fn send(
    prompt: String,
    uin: i64,
    send_to: i64,
    send_type: String,
    from_uin: i64,
) -> Result<()> {
    let addr = get_mq_addr()?;
    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    let body =
        json!({"tag":prompt,"uin":uin,"send_to":send_to,"send_type":send_type,"from_uin":from_uin});

    channel
        .basic_publish(
            "",
            "sdqueue",
            BasicPublishOptions::default(),
            body.to_string().as_bytes(),
            BasicProperties::default(),
        )
        .await?;
    Ok(())
}
