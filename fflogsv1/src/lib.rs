#![feature(slice_group_by)]
pub mod extensions;
pub mod parses;
pub mod report;
pub mod tables;
use log::{error, info};
use reqwest::{Response, StatusCode};
use serde::{de::DeserializeOwned, Deserialize};
use thiserror::Error;

use crate::{parses::*, report::*, tables::*};
#[derive(Clone)]
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
    pub fn new_from_client(api_key: &str, client: reqwest::Client) -> FF14 {
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
        time_frame: &str,
    ) -> Result<Vec<Parses>, FFError> {
        info!("{} ⏳︎正在获取", character_name);
        let mut build = self
            .client
            .get(format!(
                "{}/parses/character/{character_name}/{server_name}/{server_region}?api_key={}",
                &self.url, &self.api_key,
            ))
            .query(&[("metric", metric), ("timeframe", time_frame)]);
        if zone.is_some() {
            build = build.query(&[("zone", zone.expect("获取character_parses的zone为空"))]);
        }
        let rsp = build.send().await?;
        let rsp = parse_response::<Vec<Parses>>(rsp).await;
        info!("获取 {} ✅", character_name);
        rsp
    }
    ///根据code获取这场日志的战斗记录
    pub async fn fights_report(&self, code: &str) -> Result<Fights, FFError> {
        let rsp = self
            .client
            .get(format!(
                "{}/report/fights/{code}?api_key={}",
                &self.url, &self.api_key
            ))
            .query(&[("translate", "true")])
            .send()
            .await?;
        parse_response::<Fights>(rsp).await
    }
    pub async fn tables_report(&self, code: &str, start: i32, end: i32) -> Result<Tables, FFError> {
        let rsp = self
            .client
            .get(format!(
                "{}/report/tables/summary/{code}?api_key={}",
                self.url, self.api_key
            ))
            .query(&[("translate", "true")])
            .query(&[("start", start), ("end", end)])
            .send()
            .await?;
        parse_response::<Tables>(rsp).await
    }
    ///获取一场战斗的死亡记录
    pub async fn tables_report_deaths(
        &self,
        code: &str,
        start: i64,
        end: i64,
    ) -> Result<DeathTables, FFError> {
        let rsp = self
            .client
            .get(format!(
                "{}/report/tables/deaths/{code}?api_key={}",
                self.url, self.api_key
            ))
            .query(&[("translate", "true")])
            .query(&[("start", start)])
            .query(&[("end", end)])
            .send()
            .await?;
        parse_response::<DeathTables>(rsp).await
    }
}

async fn parse_response<T: DeserializeOwned>(response: Response) -> Result<T, FFError> {
    match response.status() {
        StatusCode::OK => {
            let rsp_bytes = response.bytes().await?;
            let response = serde_json::from_slice(&rsp_bytes);
            //反序列化不成功输出错误body
            let response = match response {
                Ok(n) => n,
                Err(e) => {
                    error!(
                        "解析json错误，body: {}",
                        String::from_utf8_lossy(&rsp_bytes)
                    );
                    return Err(FFError::SerializeError(e));
                }
            };
            Ok(response)
        }
        _ => {
            let rsp_bytes = response.bytes().await?;
            let response = serde_json::from_slice::<FFLogsV1ErrorBody>(&rsp_bytes)?;
            Err(FFError::FFLogsV1Error(format!(
                "{}:{}",
                response.status, response.error
            )))
        }
    }
}

#[derive(Deserialize, Debug)]
struct FFLogsV1ErrorBody {
    status: u16,
    error: String,
}

#[derive(Debug, Error)]
pub enum FFError {
    #[error("🙃请求fflogs错误,{0}")]
    FFLogsV1Error(String),
    #[error("🙃请求物品价格错误,{0}")]
    ItemPrice(String),
    #[error("🙃搜索物品错误,{0}")]
    ItemSearch(String),
    #[error("🙃请求接口异常,{0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("🙃序列化失败,{0}")]
    SerializeError(#[from] serde_json::Error),
    #[error("😒没有找到物品")]
    ItemNotFound,
    #[error(transparent)]
    Unknown(#[from] Box<dyn std::error::Error + Send>),
}
