use reqwest::{blocking::Client, header::{HeaderMap, HeaderValue}, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use super::riot_api_trait::{V1Account, V4Summoner};
use crate::RIOT_TOKEN;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SummonerV4 {
    pub id: String,
    pub account_id: String,
    pub puuid: String,
    pub profile_icon_id: u32,
    pub revision_date: i64,
    pub summoner_level: u32,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Response {
    Success(SummonerV4),
    Error(Value),
}

impl V4Summoner for SummonerV4 {
    type T = Self;
    fn fetch(user_region: &str, puuid: &str) -> Result<Self::T, Error> {
        let url = format! (
            "https://{}.api.riotgames.com/lol/summoner/v4/summoners/by-puuid/{}",
            user_region,
            puuid
        );

        // headerにRIOT_TOKEN(グローバル変数)を設定
        // unwrapのエラー処理は後で行う
        let mut headers = HeaderMap::new();
        headers.insert("X-Riot-Token",  HeaderValue::from_str(&*RIOT_TOKEN).unwrap());

        // Response型で受け取る（成功→SummonerV4、失敗→Value）
        let client = Client::new();
        let response = client
            .get(&url)
            .headers(headers)
            .send()?
            .json::<Response>()?;

        // enum Responseのdebug用
        println!("{:?}", response);

        // enumで成功したときだけ型で通して、エラーが来たらその内容を出力する。
        // Errorの実装を行う
        let summoner_v4_response = match response {
            Response::Success(data) => data,
            Response::Error(error) => panic!("{}", error),
        };

        Ok(summoner_v4_response)
    }
}
