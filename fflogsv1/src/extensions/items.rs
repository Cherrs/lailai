use futures::future::try_join_all;
use log::error;
use reqwest::Response;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::FFError;
use crate::FF14;

impl FF14 {
    ///从wakingsands搜索物品
    pub async fn get_items(&self, name: &str) -> Result<Vec<Item>, FFError> {
        let result= self.client.get("https://cafemaker.wakingsands.com/search?string_algo=multi_match&limit=6&indexes=Item")
        .query(&[("string",name)])
        .send()
        .await?;
        let result = parse_response::<ItemsResult>(result).await?;
        let mut f = Vec::new();
        for i in result.results {
            f.push(self.get_icon(i));
        }
        let result = try_join_all(f).await?;
        if result.is_empty() {
            return Err(FFError::ItemNotFound);
        }
        Ok(result)
    }
    ///从wakingsands搜索物品
    pub async fn get_first_item(&self, name: &str) -> Result<Item, FFError> {
        let result= self.client.get("https://cafemaker.wakingsands.com/search?string_algo=multi_match&limit=6&indexes=Item")
            .query(&[("string",name)])
            .send()
            .await?;
        let result = parse_response::<ItemsResult>(result).await?;
        let first_item = result.results.first();
        match first_item {
            Some(first_item) => Ok(self.get_icon(first_item.clone()).await?),
            None => Err(FFError::ItemNotFound),
        }
    }
    async fn get_icon(&self, item: WResult) -> Result<Item, FFError> {
        let result = self
            .client
            .get(format!("{}{}", "https://xivapi.com", item.icon))
            .send()
            .await?
            .bytes()
            .await?
            .to_vec();
        Ok(Item {
            icon: result,
            id: item.id,
            name: item.name.clone(),
        })
    }
}

async fn parse_response<T: DeserializeOwned>(response: Response) -> Result<T, FFError> {
    match response.status() {
        StatusCode::OK => {
            let rspbytes = response.bytes().await?;
            let response = serde_json::from_slice(&rspbytes);
            //反序列化不成功输出错误body
            let response = match response {
                Ok(n) => n,
                Err(e) => {
                    error!("解析json错误，body: {}", String::from_utf8_lossy(&rspbytes));
                    return Err(FFError::SerializeError(e));
                }
            };
            Ok(response)
        }
        //TODO:解析cafemaker.wakingsands.com api 返回的错误
        _ => match response.text().await {
            Ok(s) => {
                error!("{}", s);
                Err(FFError::ItemPrice(String::from("not 200")))
            }
            Err(e) => {
                error!("{}", e);
                Err(FFError::ItemPrice(String::from("请求错误啦")))
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn get_items_test() {
        let ff = FF14::new("123");
        let p = ff.get_items("翅膀").await.unwrap();
        for i in p {
            println!("{}", i.name);
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemsResult {
    #[serde(rename = "Pagination")]
    pub pagination: Pagination,
    #[serde(rename = "Results")]
    pub results: Vec<WResult>,
    #[serde(rename = "SpeedMs")]
    pub speed_ms: i64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    #[serde(rename = "Page")]
    pub page: i64,
    #[serde(rename = "PageNext")]
    pub page_next: Option<i64>,
    #[serde(rename = "PagePrev")]
    pub page_prev: Value,
    #[serde(rename = "PageTotal")]
    pub page_total: i64,
    #[serde(rename = "Results")]
    pub results: i64,
    #[serde(rename = "ResultsPerPage")]
    pub results_per_page: i64,
    #[serde(rename = "ResultsTotal")]
    pub results_total: i64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WResult {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "Icon")]
    pub icon: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Url")]
    pub url: String,
    #[serde(rename = "UrlType")]
    pub url_type: String,
    #[serde(rename = "_")]
    pub field: String,
    #[serde(rename = "_Score")]
    pub score: String,
}

pub struct Item {
    pub id: i32,
    pub name: String,
    pub icon: Vec<u8>,
}
