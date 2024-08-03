use reqwest::Error;
use serde::{Deserialize, Serialize};

use super::v4_traits::V4UseSummoner;

#[derive(Serialize, Deserialize, Debug)]
pub struct LeagueV4 {
    pub league_id: String,
    pub queue_type: String,
    pub tier: String,
    pub rank: String,
    pub summoner_id: String,
    pub summoner_name: String,
    pub league_points: i32,
    pub wins: i32,
    pub losses: i32,
    pub veteran: bool,
    pub inactive: bool,
    pub fresh_blood: bool,
    pub hot_streak: bool
}

impl V4UseSummoner for LeagueV4 {
    type T = Vec<Self>;
    fn fetch(region: &str, puuid: &str, count: &str, riot_token: &str) -> Result<Self::T, Error> {
        todo!()
    }
}

impl Default for LeagueV4 {
    fn default() -> Self {
        Self {
            league_id: String::default(),
            queue_type: String::default(),
            tier: String::default(),
            rank: String::default(),
            summoner_id: String::default(),
            summoner_name: String::default(),
            league_points: i32::default(),
            wins: i32::default(),
            losses: i32::default(),
            veteran: bool::default(),
            inactive: bool::default(),
            fresh_blood: bool::default(),
            hot_streak: bool::default()
        }
    }
}
