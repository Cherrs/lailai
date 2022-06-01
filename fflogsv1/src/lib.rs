#![feature(slice_group_by)]
pub mod extensions;
pub mod parses;
pub mod report;
pub mod tables;
use log::info;

use crate::{parses::*, report::*, tables::*};
pub struct FF14 {
    api_key: String,
    client: reqwest::Client,
    url: String,
}

impl FF14 {
    pub fn new(api_key: &str) -> FF14 {
        FF14 {
            api_key: String::from(api_key),
            client: reqwest::Client::new(),
            url: String::from("https://cn.fflogs.com:443/v1"),
        }
    }
    pub fn new_withclient(api_key: &str, client: reqwest::Client) -> FF14 {
        FF14 {
            api_key: String::from(api_key),
            client,
            url: String::from("https://cn.fflogs.com:443/v1"),
        }
    }
    ///获取character_parses
    pub async fn character_parses(
        &self,
        character_name: &str,
        server_name: &str,
        server_region: &str,
        metric: &str,
        zone: Option<i32>,
        timeframe: &str,
    ) -> Result<Vec<Parses>, Box<dyn std::error::Error>> {
        info!("{} ⏳︎正在获取", character_name);
        let mut build = self
            .client
            .get(format!(
                "{}/parses/character/{}/{}/{}?api_key={}",
                &self.url, character_name, server_name, server_region, &self.api_key,
            ))
            .query(&[("metric", metric), ("timeframe", timeframe)]);
        if zone.is_some() {
            build = build.query(&[("zone", zone.expect("获取character_parses的zone为空"))]);
        }
        let rsp = build.send().await?.json::<Vec<Parses>>().await?;
        info!("获取 {} ✅", character_name);
        Ok(rsp)
    }
    ///根据code获取这场日志的战斗记录
    pub async fn fights_report(&self, code: &str) -> Result<Fights, Box<dyn std::error::Error>> {
        let rsp = self
            .client
            .get(format!(
                "{}/report/fights/{}?api_key={}",
                &self.url, code, &self.api_key
            ))
            .query(&[("translate", "true")])
            .send()
            .await?
            .json::<Fights>()
            .await?;
        Ok(rsp)
    }
    pub async fn tables_report(
        &self,
        code: &str,
        start: i32,
        end: i32,
    ) -> Result<Tables, Box<dyn std::error::Error>> {
        let rsp = self
            .client
            .get(format!(
                "{}/report/tables/summary/{}?api_key={}",
                self.url, code, self.api_key
            ))
            .query(&[("translate", "true")])
            .query(&[("start", start), ("end", end)])
            .send()
            .await?
            .json::<Tables>()
            .await?;
        Ok(rsp)
    }
    ///获取一场战斗的死亡记录
    pub async fn tables_report_deaths(
        &self,
        code: &str,
        start: i64,
        end: i64,
    ) -> Result<DeathTables, Box<dyn std::error::Error>> {
        let rsp = self
            .client
            .get(format!(
                "{}/report/tables/deaths/{}?api_key={}",
                self.url, code, self.api_key
            ))
            .query(&[("translate", "true")])
            .query(&[("start", start)])
            .query(&[("end", end)])
            .send()
            .await?
            .json::<DeathTables>()
            .await?;
        Ok(rsp)
    }
}
