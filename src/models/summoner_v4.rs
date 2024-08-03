use reqwest::{blocking::Client, header::{HeaderMap, HeaderValue}, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::v4_traits::V4Summoner;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SummonerV4 {
    pub puuid: String,
    pub game_name: String,
    pub tag_line: String,
}

impl V4Summoner for SummonerV4 {
    type T = Self;
    fn fetch(region: &str, sn_name: &str, tag: &str, api_key: &str) -> Result<Self::T, Error> {
        let url = format! (
            "https://{}.api.riotgames.com/riot/account/v1/accounts/by-riot-id/{}/{}",
            region,
            sn_name,
            tag
        );

        // headerにapikeyを設定
        // unwrapのエラー処理は後で行う
        let mut headers = HeaderMap::new();
        headers.insert("X-Riot-Token",  HeaderValue::from_str(api_key).unwrap());

        // 一旦serde_json::Valueで受け取って型変換する
        let client = Client::new();
        let response = client
            .get(&url)
            .headers(headers)
            .send()?
            .json::<Value>()?;
        println!("{}", response);
        // unwrapのエラー処理は後で行う
        let summoner_v4_response: SummonerV4 = serde_json::from_value(response).unwrap();

        Ok(summoner_v4_response)
    }
}

impl Default for SummonerV4 {
    fn default() -> Self {
        Self {
            puuid: String::default(),
            game_name: String::default(),
            tag_line: String::default(),
        }
    }
}
