use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fights {
    pub lang: String,
    pub fights: Vec<Fight>,
    pub friendlies: Vec<Friendly>,
    pub enemies: Vec<Enemy>,
    pub friendly_pets: Vec<FriendlyPet>,
    pub enemy_pets: Vec<Value>,
    pub phases: Vec<Phase>,
    pub log_version: i64,
    pub game_version: i64,
    pub title: String,
    pub owner: String,
    pub start: i64,
    pub end: i64,
    pub zone: i64,
    pub exported_characters: Vec<ExportedCharacter>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fight {
    pub id: i64,
    pub boss: i64,
    #[serde(rename = "start_time")]
    pub start_time: i64,
    #[serde(rename = "end_time")]
    pub end_time: i64,
    pub name: String,
    #[serde(rename = "zoneID")]
    pub zone_id: i64,
    pub zone_name: String,
    pub size: Option<i64>,
    pub difficulty: Option<i64>,
    pub kill: Option<bool>,
    pub partial: Option<i64>,
    pub in_progress: Option<bool>,
    pub standard_composition: Option<bool>,
    pub has_echo: Option<bool>,
    pub boss_percentage: Option<i64>,
    pub fight_percentage: Option<i64>,
    pub last_phase_as_absolute_index: Option<i64>,
    pub last_phase_for_percentage_display: Option<i64>,
    #[serde(default)]
    pub maps: Vec<Map>,
    pub original_boss: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Map {
    #[serde(rename = "mapID")]
    pub map_id: i64,
    pub map_name: String,
    pub map_file: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Friendly {
    pub name: String,
    pub id: i64,
    pub guid: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub server: Option<String>,
    pub icon: String,
    pub fights: Vec<Fight2>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fight2 {
    pub id: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Enemy {
    pub name: String,
    pub id: i64,
    pub guid: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub icon: String,
    pub fights: Vec<Fight3>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fight3 {
    pub id: i64,
    pub instances: i64,
    pub groups: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendlyPet {
    pub name: String,
    pub id: i64,
    pub guid: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub icon: String,
    pub pet_owner: i64,
    pub fights: Vec<Fight4>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fight4 {
    pub id: i64,
    pub instances: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Phase {
    pub boss: i64,
    pub separates_wipes: bool,
    pub phases: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportedCharacter {
    pub id: i64,
    pub name: String,
    pub server: String,
    pub region: String,
}
