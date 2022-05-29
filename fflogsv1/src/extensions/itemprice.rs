use std::fmt::format;

use futures::future::ok;
use futures::future::try_join_all;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::FF14;

impl FF14 {
    ///#### 搜索物品，获取搜索到的第一个物品在猫小胖服务器的价格
    /// 目前这个方法只支持猫小胖🤣🤣🤣
    async fn get_item_price(&self, name: &str) {
        //TODO:未完成功能
        // let server_list = vec![
        //     "紫水栈桥",
        //     "摩杜纳",
        //     "延夏",
        //     "海猫茶屋",
        //     "静语庄园",
        //     "琥珀原",
        //     "柔风海湾",
        // ];
        // self.get_items()
    }

    async fn get_item_price_by_server(
        &self,
        server_name: &str,
        item_id: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.client
            .get(format!(
                "https://universalis.app/api/{}/{}",
                server_name, item_id,
            ))
            .send()
            .await?
            .json()
            .await?;
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub current_average_price_hq: i64,
    pub regular_sale_velocity: f64,
    pub nq_sale_velocity: f64,
    pub hq_sale_velocity: i64,
    pub average_price: f64,
    #[serde(rename = "averagePriceNQ")]
    pub average_price_nq: f64,
    #[serde(rename = "averagePriceHQ")]
    pub average_price_hq: i64,
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
    pub stack_size_histogram: StackSizeHistogram,
    #[serde(rename = "stackSizeHistogramNQ")]
    pub stack_size_histogram_nq: StackSizeHistogramNq,
    #[serde(rename = "stackSizeHistogramHQ")]
    pub stack_size_histogram_hq: StackSizeHistogramHq,
    pub world_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Materum {
    #[serde(rename = "slotID")]
    pub slot_id: i64,
    #[serde(rename = "materiaID")]
    pub materia_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentHistory {
    pub hq: bool,
    pub price_per_unit: i64,
    pub quantity: i64,
    pub timestamp: i64,
    pub buyer_name: String,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StackSizeHistogram {
    #[serde(rename = "1")]
    pub n1: i64,
    #[serde(rename = "5")]
    pub n5: i64,
    #[serde(rename = "15")]
    pub n15: i64,
    #[serde(rename = "16")]
    pub n16: i64,
    #[serde(rename = "20")]
    pub n20: i64,
    #[serde(rename = "41")]
    pub n41: i64,
    #[serde(rename = "99")]
    pub n99: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StackSizeHistogramNq {
    #[serde(rename = "1")]
    pub n1: i64,
    #[serde(rename = "5")]
    pub n5: i64,
    #[serde(rename = "15")]
    pub n15: i64,
    #[serde(rename = "16")]
    pub n16: i64,
    #[serde(rename = "20")]
    pub n20: i64,
    #[serde(rename = "41")]
    pub n41: i64,
    #[serde(rename = "99")]
    pub n99: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StackSizeHistogramHq {}

//TODO
// pub struct ItemsPrice {
//     pub name: String,
//     pub icon: Vec<u8>,
//     pub
// }
