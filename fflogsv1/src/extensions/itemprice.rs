use futures::future::try_join_all;
use log::error;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use thiserror::Error;

use crate::FF14;

impl FF14 {
    ///#### 搜索物品，获取搜索到的第一个物品在猫小胖服务器的价格
    /// 目前这个方法只支持猫小胖🤣🤣🤣
    pub async fn get_item_price(&self, name: &str) -> Result<ItemsPrice, GetItemPriceError> {
        let server_list = vec![
            "紫水栈桥",
            "摩杜纳",
            "延夏",
            "海猫茶屋",
            "静语庄园",
            "琥珀原",
            "柔风海湾",
        ];
        let mut f = Vec::new();
        //获取第一个模糊搜索到的物品
        let item = self.get_first_item(name).await?;
        for i in server_list {
            f.push(self.get_item_price_by_server(i, item.id));
        }
        let items_price = try_join_all(f).await?;
        let mut items = Vec::new();

        for i in items_price {
            let mut is: Vec<ItemsPriceList> = i
                .listings
                .iter()
                .map(|x| ItemsPriceList {
                    num: x.quantity,
                    price: x.total,
                    unit_price: x.price_per_unit,
                    server_name: i.world_name.to_string(),
                    seller_name: x.retainer_name.clone(),
                    last_update_time: i.last_upload_time,
                })
                .collect();
            items.append(&mut is);
        }
        items.sort_unstable_by_key(|x| x.unit_price);
        Ok(ItemsPrice {
            icon: item.icon.clone(),
            name: item.name.clone(),
            price_list: items,
        })
    }
    ///#### 🛒🛒🛒从universalis.app查询服务器物品价格
    async fn get_item_price_by_server(
        &self,
        server_name: &str,
        item_id: i32,
    ) -> Result<ItemPriceResult, GetItemPriceError> {
        let item_price = self
            .client
            .get(format!(
                "https://universalis.app/api/{}/{}",
                server_name, item_id,
            ))
            .send()
            .await?
            .json::<ItemPriceResult>()
            .await?;
        Ok(item_price)
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemPriceResult {
    #[serde(rename = "itemID")]
    pub item_id: i64,
    #[serde(rename = "worldID")]
    pub world_id: i64,
    pub last_upload_time: i64,
    pub listings: Vec<Listing>,
    pub recent_history: Vec<RecentHistory>,
    pub current_average_price: f64,
    #[serde(rename = "currentAveragePriceNQ")]
    pub current_average_price_nq: f64,
    #[serde(rename = "currentAveragePriceHQ")]
    pub current_average_price_hq: f64,
    pub regular_sale_velocity: f64,
    pub nq_sale_velocity: f64,
    pub hq_sale_velocity: f64,
    pub average_price: f64,
    #[serde(rename = "averagePriceNQ")]
    pub average_price_nq: f64,
    #[serde(rename = "averagePriceHQ")]
    pub average_price_hq: f64,
    pub min_price: i64,
    #[serde(rename = "minPriceNQ")]
    pub min_price_nq: i64,
    #[serde(rename = "minPriceHQ")]
    pub min_price_hq: i64,
    pub max_price: i64,
    #[serde(rename = "maxPriceNQ")]
    pub max_price_nq: i64,
    #[serde(rename = "maxPriceHQ")]
    pub max_price_hq: i64,
    pub world_name: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Listing {
    pub last_review_time: i64,
    pub price_per_unit: i64,
    pub quantity: i64,
    #[serde(rename = "stainID")]
    pub stain_id: i64,
    pub creator_name: String,
    #[serde(rename = "creatorID")]
    pub creator_id: Value,
    pub hq: bool,
    pub is_crafted: bool,
    #[serde(rename = "listingID")]
    pub listing_id: Value,
    pub materia: Vec<Materum>,
    pub on_mannequin: bool,
    pub retainer_city: i64,
    #[serde(rename = "retainerID")]
    pub retainer_id: String,
    pub retainer_name: String,
    #[serde(rename = "sellerID")]
    pub seller_id: String,
    pub total: i64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Materum {
    #[serde(rename = "slotID")]
    pub slot_id: i64,
    #[serde(rename = "materiaID")]
    pub materia_id: i64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentHistory {
    pub hq: bool,
    pub price_per_unit: i64,
    pub quantity: i64,
    pub timestamp: i64,
    pub buyer_name: String,
    pub total: i64,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ItemsPrice {
    pub icon: Vec<u8>,
    pub name: String,
    pub price_list: Vec<ItemsPriceList>,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ItemsPriceList {
    pub price: i64,
    pub num: i64,
    pub unit_price: i64,
    pub seller_name: String,
    pub server_name: String,
    pub last_update_time: i64,
}
#[derive(Debug, Error)]
pub enum GetItemPriceError {
    #[error("查询物品价格错误！")]
    GetPriceError,
    #[error("🙃请求查询接口错误,{0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("🤔{0}")]
    GetItemError(#[from] super::items::GetItemError),
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error>),
}
