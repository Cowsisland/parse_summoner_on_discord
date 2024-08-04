use reqwest::Error;
use serde::{Deserialize, Serialize};

use super::riot_api_trait::V4UseSummoner;

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
    fn fetch(region: &str, puuid: &str, count: &str) -> Result<Self::T, Error> {
        todo!();
        // https://jp1.api.riotgames.com/lol/league/v4/entries/by-summoner/ZD86xG3DzyjbyJ_znmGaF2fY1gMyIzsyv0h-fCDZ5NQnN539oLURYsL_83u7UORCG8YiO8HXO1Tz7Q
        // let url = format! (
        //     "https://{}.api.riotgames.com/riot/account/v1/accounts/by-riot-id/{}/{}",
        //     server_region,
        //     sn,
        //     tag
        // );

        // // headerにRIOT_TOKEN(グローバル変数)を設定
        // // unwrapのエラー処理は後で行う
        // let mut headers = HeaderMap::new();
        // headers.insert("X-Riot-Token",  HeaderValue::from_str(&*RIOT_TOKEN).unwrap());

        // // Response型で受け取る（成功→SummonerV4、失敗→Value）
        // let client = Client::new();
        // let response = client
        //     .get(&url)
        //     .headers(headers)
        //     .send()?
        //     .json::<Response>()?;
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
