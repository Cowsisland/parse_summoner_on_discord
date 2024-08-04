use reqwest::{blocking::Client, header::{HeaderMap, HeaderValue}, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use super::riot_api_trait::V1Account;
use crate::RIOT_TOKEN;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountV1 {
    pub puuid: String,
    pub game_name: String,
    pub tag_line: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Response {
    Success(AccountV1),
    Error(Value),
}

impl V1Account for AccountV1 {
    type T = Self;
    fn fetch(server_region: &str, sn: &str, tag: &str) -> Result<Self::T, Error> {
        let url = format! (
            "https://{}.api.riotgames.com/riot/account/v1/accounts/by-riot-id/{}/{}",
            server_region,
            sn,
            tag
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
        // println!("{:?}", response);

        // enumで成功したときだけ型で通して、エラーが来たらその内容を出力する。
        // Errorの実装を行う
        let summoner_v4_response = match response {
            Response::Success(data) => data,
            Response::Error(error) => panic!("{}", error),
        };

        Ok(summoner_v4_response)
    }
}
