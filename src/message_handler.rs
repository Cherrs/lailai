use crate::{
    config::{DOTA_GROUP_CONF_BYQQ, GROUP_CONF_BYQQ},
    openai::get_ai_message,
};
use async_trait::async_trait;
use chrono::{FixedOffset, TimeZone, Utc};
use fflogsv1::{FFError, FF14};
use opendota::heroes::HEROS;
use ricq::{
    handler::{Handler, QEvent},
    msg::elem::*,
    msg::MessageChain,
    Client,
};
use std::sync::Arc;
use tracing::{debug, error, info};

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
                    m.inner.group_code, m.inner.elements
                );
                let reply = Reply {
                    reply_seq: m.inner.seqs[0],
                    sender: m.inner.from_uin,
                    time: m.inner.time,
                    elements: m.inner.elements.clone(),
                };
                let mut elme = m.inner.elements.into_iter();
                if let Some(RQElem::At(at)) = elme.next() && at.target == m.client.uin().await {
                    match elme.next() {
                        Some(RQElem::Text(t)) if t.content != " " => {
                            let mut args:Vec<&str> = t.content.split(' ').collect();
                            args.retain(|x|!x.is_empty());
                            let mut args = args.iter().copied();
                            match args.next_back(){
                                Some(c) if c == "物品"=>{
                                    let itemstr:Vec<&str> = args.collect();
                                    let itemstr = itemstr.join(" ");
                                    let msg = send_item_data_to_group(&itemstr,m.inner.group_code,&self.ff14client,&m.client).await;
                                    if let Err(e) = m.client.send_group_message(m.inner.group_code, msg).await{
                                        error!("发送错误{}",e);
                                    }
                                }
                                Some(c) if c == "价格"=>{
                                    let itemstr:Vec<&str> = args.collect();
                                    let itemstr = itemstr.join(" ");
                                    let msg = send_item_price_to_group(&itemstr,m.inner.group_code,&self.ff14client,&m.client).await;
                                    if let Err(e) = m.client.send_group_message(m.inner.group_code, msg).await{
                                        error!("发送错误{}",e);
                                    }
                                    info!("{}",itemstr);
                                }
                                Some(c) if c == "战绩"=>{
                                    let msg = send_dota_recent_matches_to_group(&m.inner.from_uin,&self.ff14client.client).await;
                                    if let Err(e) = m.client.send_group_message(m.inner.group_code, msg).await{
                                        error!("发送错误{}",e);
                                    }
                                }
                                _=>{
                                    let mut msg = MessageChain::default();
                                    let rsp = get_ai_message(&self.ff14client.client,&t.content,m.inner.from_uin).await;
                                    msg.with_reply(reply);
                                    match rsp{
                                        Err(e)=>{
                                            error!("{}",e);
                                            msg.push(Text::new("不知道，你可以再问一次试试".to_string()));
                                        },
                                        Ok(r)=>{
                                            msg.push(Text::new(r));
                                        }
                                    }
                                    if let Err(e) = m.client.send_group_message(m.inner.group_code, msg).await{
                                        error!("发送错误{}",e);
                                    }
                                }
                            }
                        }
                        _=>{
                            if GROUP_CONF_BYQQ.get().is_some(){
                                let msg = send_highest_data_to_group(m.inner.from_uin,&self.ff14client,).await;
                                if let Some(msg) = msg{
                                    if let Err(e) = m.client.send_group_message(m.inner.group_code, msg).await{
                                        error!("发送错误{}",e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            QEvent::FriendMessage(m) => {
                info!(
                    "MESSAGE (FRIEND={}): {}",
                    m.inner.from_uin, m.inner.elements
                );
            }
            QEvent::GroupTempMessage(m) => {
                info!("MESSAGE (TEMP={}): {}", m.inner.from_uin, m.inner.elements);
            }
            QEvent::GroupRequest(m) => {
                info!(
                    "REQUEST (GROUP={}, UIN={}): {}",
                    m.inner.group_code, m.inner.req_uin, m.inner.message
                );
            }
            QEvent::NewFriendRequest(m) => {
                info!("REQUEST (UIN={}): {}", m.inner.req_uin, m.inner.message);
            }
            QEvent::NewMember(m) => {
                if m.inner.member_uin == m.client.uin().await {
                    let mut mc = MessageChain::default();
                    let s = String::from_utf8(vec![
                        229, 176, 143, 232, 173, 166, 229, 175, 159, 230, 157, 165, 229, 149, 166,
                        239, 188, 129, 230, 173, 164, 233, 161, 185, 231, 155, 174, 230, 152, 175,
                        228, 189, 191, 231, 148, 168, 65, 71, 80, 76, 32, 51, 46, 48, 229, 141,
                        143, 232, 174, 174, 229, 188, 128, 230, 186, 144, 231, 154, 132, 229, 133,
                        141, 232, 180, 185, 230, 156, 186, 229, 153, 168, 228, 186, 186,
                    ])
                    .unwrap();
                    mc.push(Text::new(s));
                    m.client
                        .send_group_message(m.inner.group_code, mc)
                        .await
                        .unwrap();
                }
            }
            QEvent::Login(_) => {}
            _ => {
                debug!("{:?}", e);
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
    let Some(configs) = GROUP_CONF_BYQQ.get()
    else {
        error!("QQ号：{} 没有配置在group_config文件中，或配置有误",from_uin);
        return None;
    };
    let Some(config) = configs.get(&from_uin)
    else {
        error!("QQ号：{} 没有配置在group_config文件中，或配置有误",from_uin);
        return None;
    };
    let mut msg = MessageChain::default();
    msg.push(Text::new(format!("{} {}\n", config.name, config.server)));
    // 当前版本数据和排行
    if let Ok(dtos) = ff14client
        .get_highest(&config.name, &config.server, &config.region, None)
        .await
    {
        let mut dtost = dtos.iter();
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
    }
    // 幻想龙诗绝境战
    if let Ok(dtos) = ff14client
        .get_highest(&config.name, &config.server, &config.region, Some(45))
        .await
    {
        let mut dtost = dtos.iter();
        for d in dtost.by_ref() {
            if d.difficulty == 100 {
                msg.push(Text::new(format!(
                    "{}({}) {:.1}% rdps:{:.1} {}\n",
                    d.bossname, "绝境战", d.rank, d.rdps, d.spec
                )));
            }
        }
    }
    // 旧版本绝境战
    if let Ok(dtos) = ff14client
        .get_highest(&config.name, &config.server, &config.region, Some(43))
        .await
    {
        let mut dtost = dtos.iter();
        for d in dtost.by_ref() {
            if d.difficulty == 100 {
                msg.push(Text::new(format!(
                    "{}({}) {:.1}% rdps:{:.1} {}\n",
                    d.bossname, "绝境战", d.rank, d.rdps, d.spec
                )));
            }
        }
    }
    Some(msg)
}
///模糊查询物品
async fn send_item_data_to_group(
    item_name: &str,
    group_code: i64,
    ff14client: &FF14,
    client: &Arc<Client>,
) -> MessageChain {
    let mut msg = MessageChain::default();
    let icon = match ff14client.get_items(item_name).await {
        Ok(icon) => Some(icon),
        Err(err) => {
            let errmsg = format!("获取物品失败,{err}");
            error!("{}", errmsg);
            if let FFError::ItemNotFound = err {
                msg.push(Text::new(format!("😒什么是 {item_name} ?")));
            }
            return msg;
        }
    };
    if let Some(icon) = icon {
        for i in icon {
            if let Ok(g) = client.upload_group_image(group_code, i.icon).await {
                msg.push(g);
            }
            let name = Text::new(format!("{}\n", i.name));
            msg.push(name);
        }
    }
    msg
}
///发送最近的比赛到群
async fn send_dota_recent_matches_to_group(uin: &i64, client: &reqwest::Client) -> MessageChain {
    let mut msg = MessageChain::default();
    let Some(steam_id) =DOTA_GROUP_CONF_BYQQ.get()
    else {
        error!("QQ:{} 没有配置steam_id",uin);
        msg.push(Text::new(String::from("不知道")));
        return msg;
    };
    let steam_id = &steam_id[uin];
    let opendota = opendota::OpenDota::new(client.clone());
    let data = opendota.players_recent_matches(&steam_id.steam_id).await;
    match data {
        Ok(data) => {
            for i in data {
                msg.push(Text::new(format!(
                    "比赛ID:{} 使用英雄:{} KDA:{}/{}/{}\n",
                    i.match_id.unwrap(),
                    HEROS.lock()[&i.hero_id.unwrap()].as_str(),
                    i.kills.unwrap(),
                    i.deaths.unwrap(),
                    i.assists.unwrap()
                )));
            }
        }
        Err(e) => {
            error!("获取最近场次失败，{}", e);
            msg.push(Text::new(String::from("不知道")));
        }
    }
    msg
}

///🎉查询物品价格并且发送
async fn send_item_price_to_group(
    item_name: &str,
    group_code: i64,
    ff14client: &FF14,
    client: &Arc<Client>,
) -> MessageChain {
    let mut msg = MessageChain::default();
    let item_price = match ff14client.get_item_price(item_name).await {
        Ok(icon) => icon,
        Err(err) => {
            let emsg = format!("获取物品价格失败,{err}");
            error!("{}", emsg);
            msg.push(Text::new(emsg));
            return msg;
        }
    };
    if item_price.price_list.is_empty() {
        msg.push(Text::new(format!("板子上没有{}", item_price.name)));
        return msg;
    }
    let mut item_price_list = item_price.price_list.iter().take(8);
    if let Ok(g) = client.upload_group_image(group_code, item_price.icon).await {
        msg.push(g);
    }
    msg.push(Text::new(item_price.name));
    let mut last_update_time = i64::MAX;
    for i in item_price_list.by_ref() {
        if last_update_time > i.last_update_time {
            last_update_time = i.last_update_time;
        }
        let name = Text::new(format!(
            "\n{} x {} 总价:{} {}({})",
            i.num, i.unit_price, i.price, i.seller_name, i.server_name
        ));
        msg.push(name);
    }
    // TODO 处理LocalResult
    match Utc.timestamp_millis_opt(last_update_time) {
        chrono::LocalResult::None => todo!(),
        chrono::LocalResult::Single(last_update_time) => {
            if let Some(offset) = FixedOffset::east_opt(8 * 3600) {
                let last_update_time = last_update_time.with_timezone(&offset);
                msg.push(Text::new(format!(
                    "\n最后更新时间 {}",
                    last_update_time.format("%Y-%m-%d %H:%M:%S")
                )));
            }
        }
        chrono::LocalResult::Ambiguous(_, _) => todo!(),
    }
    msg
}
