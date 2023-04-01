#![recursion_limit = "512"]

use error::OpenDotaErr;
use serde::{Deserialize, Serialize};
pub mod error;
pub mod heroes;
pub struct OpenDota {
    client: reqwest::Client,
}

impl OpenDota {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
    pub async fn players_recent_matches(
        &self,
        steam_id: &u64,
    ) -> Result<PlayersRecentMatches, OpenDotaErr> {
        Ok(self
            .client
            .get(format!(
                "https://api.opendota.com/api/players/{steam_id}/recentMatches"
            ))
            .send()
            .await?
            .json::<PlayersRecentMatches>()
            .await?)
    }
}

pub type PlayersRecentMatches = Vec<PlayersRecentMatche>;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayersRecentMatche {
    pub match_id: Option<i64>,
    pub player_slot: Option<i64>,
    pub radiant_win: Option<bool>,
    pub duration: Option<i64>,
    pub game_mode: Option<i64>,
    pub lobby_type: Option<i64>,
    pub hero_id: Option<i64>,
    pub start_time: Option<i64>,
    pub version: Option<i64>,
    pub kills: Option<i64>,
    pub deaths: Option<i64>,
    pub assists: Option<i64>,
    pub skill: Option<serde_json::Value>,
    pub average_rank: Option<i64>,
    pub xp_per_min: Option<i64>,
    pub gold_per_min: Option<i64>,
    pub hero_damage: Option<i64>,
    pub tower_damage: Option<i64>,
    pub hero_healing: Option<i64>,
    pub last_hits: Option<i64>,
    pub lane: Option<i64>,
    pub lane_role: Option<i64>,
    pub is_roaming: Option<bool>,
    pub cluster: Option<i64>,
    pub leaver_status: Option<i64>,
    pub party_size: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn players_recent_matches() {
        let client = OpenDota::new(reqwest::Client::new());
        let data = client.players_recent_matches(&87112038).await.unwrap();
        println!("{}", data.first().unwrap().deaths.unwrap());
    }
}
