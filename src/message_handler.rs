use std::sync::Arc;

use async_trait::async_trait;
use fflogsv1::FF14;
use log::{error, info};
use ricq::{
    handler::{Handler, QEvent},
    msg::elem::*,
    msg::MessageChain,
    Client,
};

use crate::config::GROUP_CONF_BYQQ;

pub struct MyHandler {
    pub ff14client: FF14,
}

#[async_trait]
impl Handler for MyHandler {
    async fn handle(&self, e: QEvent) {
        match e {
            QEvent::GroupMessage(m) => {
                info!(
                    "MESSAGE (GROUP={}): {}",
                    m.message.group_code, m.message.elements
                );
                let mut elme = m.message.elements.into_iter();
                if let Some(RQElem::At(at)) = elme.next() && at.target == m.client.uin().await {
                    match elme.next() {
                        Some(RQElem::Text(t)) if t.content != " " => {
                            let mut args:Vec<&str> = t.content.split(' ').collect();
                            args.retain(|x|*x != "");
                            let mut args = args.iter().copied();
                            match args.next_back(){
                                Some(c) if c == "物品"=>{
                                    let itemstr:Vec<&str> = args.collect();
                                    let itemstr = itemstr.join(" ");
                                    if let Some(msg) = send_item_data_to_group(&itemstr,m.message.group_code,&self.ff14client,&m.client)
                                    .await{
                                        if let Err(e) = m.client.send_group_message(m.message.group_code, msg).await{
                                            error!("发送错误{}",e);
                                        }
                                    }
                                    info!("{}",itemstr);
                                }
                                Some(c) if c == "价格"=>{
                                    let itemstr:Vec<&str> = args.collect();
                                    let itemstr = itemstr.join(" ");
                                    if let Some(msg) = send_item_data_to_group(&itemstr,m.message.group_code,&self.ff14client,&m.client)
                                    .await{
                                        if let Err(e) = m.client.send_group_message(m.message.group_code, msg).await{
                                            error!("发送错误{}",e);
                                        }
                                    }
                                    info!("{}",itemstr);
                                }
                                _=>{}
                            }
                        }
                        _=>{

                            let msg = send_highest_data_to_group(m.message.from_uin,&self.ff14client,).await;
                            if let Some(msg) = msg{
                                if let Err(e) = m.client.send_group_message(m.message.group_code, msg).await{
                                    error!("发送错误{}",e);
                                }
                            }

                        }
                    }
                }
            }
            QEvent::FriendMessage(m) => {
                info!(
                    "MESSAGE (FRIEND={}): {}",
                    m.message.from_uin, m.message.elements
                );
            }
            QEvent::TempMessage(m) => {
                info!(
                    "MESSAGE (TEMP={}): {}",
                    m.message.from_uin, m.message.elements
                );
            }
            QEvent::GroupRequest(m) => {
                info!(
                    "REQUEST (GROUP={}, UIN={}): {}",
                    m.request.group_code, m.request.req_uin, m.request.message
                );
            }
            QEvent::FriendRequest(m) => {
                info!("REQUEST (UIN={}): {}", m.request.req_uin, m.request.message);
            }
            _ => {
                info!("{:?}", e);
            }
        }
    }
}

pub fn difficulty_to_string(difficulty: i32) -> &'static str {
    match difficulty {
        101 => "零式",
        100 => "普通",
        _ => "",
    }
}

async fn send_highest_data_to_group(from_uin: i64, ff14client: &FF14) -> Option<MessageChain> {
    let configs = GROUP_CONF_BYQQ.get().unwrap();
    let config = configs.get(&from_uin).unwrap();
    if let Ok(dtos) = ff14client
        .get_highest(&config.name, &config.server, &config.region)
        .await
    {
        let mut dtost = dtos.iter();
        let mut msg = MessageChain::default();
        msg.push(Text::new(format!("{} {}\n", config.name, config.server)));
        for d in dtost.by_ref() {
            if d.difficulty == 101 {
                msg.push(Text::new(format!(
                    "{}({}) {:.1}% rdps:{:.1} {}\n",
                    d.bossname,
                    difficulty_to_string(d.difficulty),
                    d.rank,
                    d.rdps,
                    d.spec
                )));
            }
        }
        return Some(msg);
    }
    None
}
///模糊查询物品
async fn send_item_data_to_group(
    item_name: &str,
    group_code: i64,
    ff14client: &FF14,
    client: &Arc<Client>,
) -> Option<MessageChain> {
    let icon = match ff14client.get_items(item_name).await {
        Ok(icon) => Some(icon),
        Err(err) => {
            error!("获取物品失败,{}", err);
            None
        }
    };
    if let Some(icon) = icon {
        let mut msg = MessageChain::default();
        for i in icon {
            if let Ok(g) = client.upload_group_image(group_code, i.icon).await {
                msg.push(g);
            }
            let name = Text::new(format!("{}\n", i.name));
            msg.push(name);
        }
        return Some(msg);
    }
    None
}
