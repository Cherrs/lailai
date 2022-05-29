use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tables {
    pub total_time: i64,
    pub item_level: i64,
    pub composition: Vec<Composition>,
    pub damage_done: Vec<DamageDone>,
    pub healing_done: Vec<HealingDone>,
    pub damage_taken: Vec<DamageTaken>,
    pub death_events: Vec<DeathEvent>,
    pub log_version: i64,
    pub game_version: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Composition {
    pub name: String,
    pub id: i64,
    pub guid: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub specs: Vec<Spec>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    pub spec: String,
    pub role: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageDone {
    pub name: String,
    pub id: i64,
    pub guid: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub icon: String,
    pub total: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealingDone {
    pub name: String,
    pub id: i64,
    pub guid: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub icon: String,
    pub total: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageTaken {
    pub name: String,
    pub guid: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub ability_icon: String,
    pub total: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeathEvent {
    pub name: String,
    pub id: i64,
    pub guid: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub icon: String,
    pub death_time: i64,
    pub ability: Option<Ability>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
    pub name: String,
    pub guid: Option<i64>,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub ability_icon: Option<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub name: String,
    pub id: i64,
    pub guid: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub icon: String,
    pub timestamp: i64,
    pub damage: Damage,
    pub healing: Healing,
    pub fight: i64,
    pub death_window: i64,
    pub overkill: i64,
    pub events: Vec<Event>,
    pub killing_blow: Option<KillingBlow>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Damage {
    pub total: i64,
    pub active_time: i64,
    pub active_time_reduced: i64,
    pub overheal: Option<i64>,
    pub abilities: Vec<Ability>,
    pub damage_abilities: Vec<Value>,
    pub sources: Vec<Source>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub name: String,
    pub total: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Healing {
    pub total: i64,
    pub active_time: i64,
    pub active_time_reduced: i64,
    pub abilities: Vec<Ability>,
    pub damage_abilities: Vec<Ability>,
    pub sources: Vec<Source>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub timestamp: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "sourceID")]
    pub source_id: i64,
    pub source_is_friendly: bool,
    #[serde(rename = "targetID")]
    pub target_id: i64,
    pub target_is_friendly: bool,
    pub ability: Ability,
    pub fight: i64,
    pub hit_type: i64,
    pub amount: i64,
    pub unmitigated_amount: Option<i64>,
    pub overkill: Option<i64>,
    #[serde(rename = "packetID")]
    pub packet_id: i64,
    pub multiplier: f64,
    pub source_instance: Option<i64>,
    pub mitigated: Option<i64>,
    #[serde(flatten)]
    other: Map<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KillingBlow {
    pub name: String,
    pub guid: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub ability_icon: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeathTables {
    pub entries: Vec<Entry>,
}
