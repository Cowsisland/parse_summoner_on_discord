use reqwest::{blocking::Client, header::{HeaderMap, HeaderValue}, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use super::v4_traits::V4UseSummoner;
use crate::RIOT_TOKEN;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RewardConfig {
    pub reward_value: String,
    pub reward_type: String,
    pub maximum_reward: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct NextSeasonMilestone {
    pub require_grade_counts: HashMap<String, u32>,
    pub reward_marks: u32,
    pub bonus: bool,
    pub reward_config: RewardConfig,
    pub total_games_requires: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChampionMasteryV4 {
    pub puuid: String,
    pub champion_id: u32,
    pub champion_level: u32,
    pub champion_points: u32,
    pub last_play_time: i64,
    pub champion_points_since_last_level: i64,
    pub champion_points_until_next_level: i64,
    pub mark_required_for_next_level: u32,
    pub tokens_earned: u32,
    pub champion_season_milestone: u32,
    pub milestone_grades: Option<Vec<String>>,
    next_season_milestone: NextSeasonMilestone,
}

#[derive(Deserialize)]
#[serde(untagged)]
#[derive(Debug)]
enum Response {
    Success(Vec<ChampionMasteryV4>),
    Error(Value),
}

impl V4UseSummoner for ChampionMasteryV4 {
    type T = Vec<Self>;
    fn fetch(user_region: &str, puuid: &str, count: &str) -> Result<Self::T, Error> {
        let url = format! (
            "https://{}.api.riotgames.com/lol/champion-mastery/v4/champion-masteries/by-puuid/{}/top?count={}",
            user_region,
            puuid,
            count
        );

        // headerにapikeyを設定
        // unwrapのエラー処理は後で行う
        let mut headers = HeaderMap::new();
        headers.insert("X-Riot-Token",  HeaderValue::from_str(&*RIOT_TOKEN).unwrap());

        // Response型で受け取る（成功→ChampionMasteryV4、失敗→Value）
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
        let champion_mastery_v4 = match response {
            Response::Success(data) => data,
            Response::Error(error) => panic!("{}", error),
        };

        Ok(champion_mastery_v4)
    }
}
