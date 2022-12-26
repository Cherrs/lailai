use crate::{
    config::{GROUP_CONF, GROUP_CONF_BYGROUPID},
    message_handler,
    pg_store::PgStore,
    sled_store::SledStore,
};
use chrono::{DateTime, Duration, FixedOffset, TimeZone, Utc};
use fflogsv1::{extensions::*, parses::Parses, FF14};
use futures::future::try_join_all;
use ricq::{
    msg::{
        elem::{At, Text},
        MessageChain,
    },
    Client,
};
use std::{collections::HashMap, env};
use tokio::time::Instant;
use tracing::info;

pub async fn send_message_init(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let store = match env::var("store")
        .unwrap_or_else(|_| "local".to_string())
        .as_str()
    {
        "postgres" => PgStore::create(&env::var("rsconstr").expect("数据库没配置！")).await,
        _ => SledStore::create(&env::var("localpath").unwrap_or_else(|_| "/db/cache".to_string())),
    };
    let now = Instant::now();
    //开始请求api获取角色数据
    let ff14 = FF14::new(env::var("logskey").unwrap().as_str());
    let mut parses_futures = Vec::new();
    for u in GROUP_CONF.get().unwrap() {
        let region = &u.1.region;
        let res = ff14.character_parses(&u.1.name, &u.1.server, region, "rdps", None, "historical");
        parses_futures.push(res);
    }

    let parses_futures = try_join_all(parses_futures).await?;
    let mut parses_map: HashMap<i64, Vec<Parses>> = HashMap::new();
    //筛选数据
    let utc: DateTime<Utc> =
        Utc::now() - Duration::days(env::var("historydays").unwrap().parse::<i64>().unwrap());
    let utcstamp = utc.timestamp_millis();

    //转换hashmap
    for i in parses_futures {
        for ii in i {
            if ii.start_time < utcstamp {
                continue;
            }
            match parses_map.get_mut(&ii.start_time) {
                Some(v) => v.push(ii),
                None => {
                    let starttime = ii.start_time;
                    let mut _v = vec![ii];
                    parses_map.insert(starttime, _v);
                }
            }
        }
    }
    //如果数据库没有数据就初始化数据库
    if store.is_empty().await {
        store.init(&parses_map).await;
    }

    let mut rows: Vec<i64> = store.query_by_start_time(utcstamp).await;
    rows.sort();
    for (k, mut v) in parses_map {
        match rows.contains(&k) {
            true => {}
            false => {
                v.sort_unstable_by(|a, b| b.percentile.partial_cmp(&a.percentile).unwrap());
                let ppp = &v.first().unwrap();
                let group_conf = GROUP_CONF_BYGROUPID.get().unwrap();
                for (groupid, confs) in group_conf {
                    let mut msg = MessageChain::default();
                    let group_qqs = get_group_qqs(client, groupid).await?;
                    let group_name_server: Vec<String> = confs
                        .iter()
                        .map(|x| format!("{}/{}", x.name, x.server))
                        .collect();
                    for u in &v {
                        let qq = confs.iter().find(|x| x.name == u.character_name);
                        if let Some(c) = qq {
                            if group_qqs.contains(&c.qq) {
                                let mut at = At::new(c.qq);
                                at.display = u.character_name.to_string();
                                msg.push(at);
                                msg.push(Text::new(" ".to_string()));
                            } else {
                                msg.push(Text::new(format!("{} ", u.character_name)));
                            }
                        }
                    }
                    let fight = ff14.get_fight(&ppp.report_id, ppp.fight_id).await?;
                    //时区转换为+8
                    //TODO 不应该使用unwarp
                    let time = Utc
                        .timestamp_millis_opt(ppp.start_time)
                        .unwrap()
                        .with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap())
                        + Duration::milliseconds(fight.fiexdtime);
                    msg.push(Text::new(format!(
                        "在 {} 击杀了 {}({})\n",
                        time.format("%Y-%m-%d %H:%M:%S"),
                        ppp.encounter_name,
                        message_handler::difficulty_to_string(ppp.difficulty)
                    )));
                    let mut issend = false;
                    for u in &v {
                        if group_name_server.contains(&format!("{}/{}", u.character_name, u.server))
                        {
                            issend = true;
                            msg.push(Text::new(format!(
                                "{} {:.1}% rdps:{:.1} {} {}\n",
                                u.character_name,
                                u.percentile,
                                u.total,
                                u.spec,
                                get_death_str(u, &fight)
                            )));
                        }
                        println!("{group_name_server:#?}");
                    }
                    msg.push(Text::new(format!(
                        "https://cn.fflogs.com/reports/{}#fight={}&type=damage-done",
                        ppp.report_id, ppp.fight_id
                    )));
                    if issend {
                        client.send_group_message(*groupid, msg).await.unwrap();
                    }
                }
                store.add_cache(ppp).await;
            }
        }
    }
    let elapsed = now.elapsed();
    info!("{:.2?}", elapsed);
    Ok(())
}

async fn get_group_qqs(
    client: &Client,
    groupid: &i64,
) -> Result<Vec<i64>, Box<dyn std::error::Error>> {
    let group_owner = client.get_group_info(*groupid).await?.unwrap().owner_uin;
    let oicq_members = client.get_group_member_list(*groupid, group_owner).await?;
    let group_qqs: Vec<i64> = oicq_members.iter().map(|x| x.uin).collect();
    Ok(group_qqs)
}

fn get_death_str(parse: &Parses, fight: &fight::GetFightDto) -> String {
    let name = &parse.character_name;
    let fight_iter = fight.deaths.iter();
    let f = fight_iter.filter(|x| x.name == *name);
    let dstr: Vec<String> = f.map(|x| x.deathname.to_string()).collect();
    let dcount = dstr.len();
    if dcount > 0 {
        return format!("死亡 {dcount} 次,死亡技能：{}", dstr.join(","));
    }
    "".to_string()
}
