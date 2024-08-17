use crate::errors::api_error::ApiError;
use crate::models::{account_v1::AccountV1, riot_api_trait::{V1Account, V4Summoner}, summoner_v4::SummonerV4};

pub fn resp_league(sn: &str, tag: &str) -> Result<Vec<String>, ApiError> {
    // アカウント情報の取得
    let server_region: &str = "asia";
    let account_v1_resp = AccountV1::fetch(server_region, sn, tag)?;
    // println!("{:?}", account_v1_resp);  // debug

    // マスタリー情報の取得
    let puuid = &account_v1_resp.puuid;
    let user_region = "jp1";
    let summoner_v4_resp: SummonerV4 = SummonerV4::fetch(user_region, puuid)?;
    println!("{:?}", summoner_v4_resp);  // debug

    let summoner_id = summoner_v4_resp.id;

    // ここから実装
    // let league_v4_resp_vec = LeagueV4::fetch(region, &account_v1_resp.id, api_key)?;


    // let mut data_vec = Vec::new();
    // data_vec.push(
    //     format!("{}    {}  {}    {}    {}",
    //         "Queue", "Tier", "Division", "LP", "Win Rate"
    //     )    
    // );

    // for league_v4_resp in league_v4_resp_vec {
    //     let win_rate: i32 = league_v4_resp.wins*100 / (league_v4_resp.wins+league_v4_resp.losses);
    //     let data_str = format!("{}    {}  {}    {}LP    {}%",
    //         league_v4_resp.queue_type,
    //         league_v4_resp.tier,
    //         league_v4_resp.rank,
    //         league_v4_resp.league_points,
    //         win_rate
    //     );
    //     data_vec.push(data_str);
    // }

    let data = vec![];
    Ok(data)
}