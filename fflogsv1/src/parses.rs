use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Parses {
    #[serde(rename = "encounterID")]
    pub encounter_id: i64,
    pub encounter_name: String,
    pub class: String,
    pub spec: String,
    pub rank: i64,
    pub out_of: i64,
    pub duration: i64,
    pub start_time: i64,
    #[serde(rename = "reportID")]
    pub report_id: String,
    #[serde(rename = "fightID")]
    pub fight_id: i64,
    pub difficulty: i32,
    #[serde(rename = "characterID")]
    pub character_id: i64,
    pub character_name: String,
    pub server: String,
    pub percentile: f32,
    pub ilvl_key_or_patch: f64,
    pub total: f32,
    pub estimated: Option<bool>,
}
