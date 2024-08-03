use reqwest::Error;
use serde::{Deserialize, Serialize};

use super::v4_trait::V4;

#[derive(Serialize, Deserialize, Debug)]
pub struct ChampionMasteryV4 {
    pub champion_id: i64,
    pub champion_level: i32,
    pub champion_points: i32,
    pub last_play_time: i64,
    pub champion_points_since_last_level: i64,
    pub champion_points_until_next_level: i64,
    pub chest_granted: bool,
    pub tokens_earned: i32,
    pub summoner_id: String
}

impl Default for ChampionMasteryV4 {
    fn default() -> Self {
        Self{
            champion_id: i64::default(),
            champion_level: i32::default(),
            champion_points: i32::default(),
            last_play_time: i64::default(),
            champion_points_since_last_level: i64::default(),
            champion_points_until_next_level: i64::default(),
            chest_granted: bool::default(),
            tokens_earned: i32::default(),
            summoner_id: String::default(),
        }
    }
}

impl V4 for ChampionMasteryV4 {
    type T = Vec<Self>;
    fn fetch(region: &str, summoner_id: &str, api_key: &str) -> Result<Self::T, Error> {
        let url = format! (
            "https://{}1.api.riotgames.com/lol/champion-mastery/v4/champion-masteries/by-summoner/{}?api_key={}",
            region,
            summoner_id,
            api_key
        );

        let resp: Vec<Self> = reqwest::blocking::get(&url)?.json()?;

        Ok(resp)
    }
}