extern crate url;
extern crate reqwest;

use crate::{
    errors::api_error::ApiError, libs::get_champ_id_map, models::{
        account_v1::AccountV1, champion_mastery_v4::ChampionMasteryV4, riot_api_trait::{V1Account, V4UseSummoner}
    }
};

pub fn resp_mastery(sn: &str, tag: &str) -> Result<Vec<String>, ApiError> {
    // アカウント情報の取得
    let server_region: &str = "asia";
    let account_v1_resp = AccountV1::fetch(server_region, sn, tag)?;
    // println!("{:?}", account_v1_resp);  // debug

    // マスタリー情報の取得
    let puuid = &account_v1_resp.puuid;
    let user_region = "jp1";
    let count: usize = 3;
    let champion_mastery_v4_resp: Vec<ChampionMasteryV4> = ChampionMasteryV4::fetch(user_region, puuid, &count.to_string())?;
    // println!("{:?}", champion_mastery_v4_resp);  // debug

    // チャンピオンとidの対応マップを取得(ファイルを更新しても大丈夫なように都度作成する)
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