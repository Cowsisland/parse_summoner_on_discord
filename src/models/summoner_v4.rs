use reqwest::Error;
use serde::{Deserialize, Serialize};

use super::v4_trait::V4;

#[derive(Serialize, Deserialize, Debug)]
pub struct SummonerV4 {
    pub id: String,
    pub account_id: String,
    pub puuid: String,
    pub name: String,
    pub profile_icon_id: i32,
    pub revision_date: i64,
    pub summoner_level: i64
}

impl V4 for SummonerV4 {
    type T = Self;
    fn fetch(region: &str, name: &str, api_key: &str) -> Result<Self::T, Error> {
        let url = format! (
            "https://{}1.api.riotgames.com/lol/summoner/v4/summoners/by-name/{}?api_key={}",
            region,
            name,
            api_key
        );

        let resp: Self = reqwest::blocking::get(&url)?.json()?;

        Ok(resp)
    }
}

impl Default for SummonerV4 {
    fn default() -> Self {
        Self {
            id: String::default(),
            account_id: String::default(),
            puuid: String::default(),
            name: String::default(),
            profile_icon_id: i32::default(),
            revision_date: i64::default(),
            summoner_level: i64::default(),
        }
    }
}
