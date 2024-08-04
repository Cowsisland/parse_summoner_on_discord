extern crate url;
extern crate reqwest;
use reqwest::Error;
use crate::models::{champion_mastery_v4::ChampionMasteryV4, summoner_v4::SummonerV4, v4_traits::{V4Summoner, V4UseSummoner}};
use crate::libs::get_champ_id_map;

pub fn resp_mastery(sn: &str, tag: &str, riot_token: &str) -> Result<Vec<String>, Error> {
    // アカウント情報の取得
    let server_region: &str = "asia";
    let summoner_v4_resp = SummonerV4::fetch(server_region, sn, tag, riot_token)?;
    // println!("{:?}", summoner_v4_resp);  // debug

    // マスタリー情報の取得
    let puuid = &summoner_v4_resp.puuid;
    let user_region = "jp1";
    let count: usize = 3;
    let champion_mastery_v4_resp: Vec<ChampionMasteryV4> = ChampionMasteryV4::fetch(user_region, puuid, &count.to_string(), riot_token)?;
    // println!("{:?}", champion_mastery_v4_resp);  // debug

    // iterで回してマスタリーのランキングとポイントを取得する
    // マスタリーのランキングをcountの数だけ表示
    let champ_id_map = get_champ_id_map::get_champ_id_map();

    let mut data_vec = Vec::new();
    for i in 0..count {
        let data_str = format!("{: <4} {: <12} {: <8} ({})",
            (i+1).to_string() +")",
            champ_id_map.get(&champion_mastery_v4_resp[i].champion_id).unwrap(),
            champion_mastery_v4_resp[i].champion_points,
            champion_mastery_v4_resp[i].champion_level
        );
        data_vec.push(data_str);
    }

    Ok(data_vec)
}