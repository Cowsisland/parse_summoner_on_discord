use reqwest::Error;

pub fn resp_league(sn: &str, tag: &str) -> Result<Vec<String>, Error> {
    todo!();
    // // to use env
    // dotenv().ok();

    // // Check Riot API
    // let api_key = &env::var("RIOT_API_KEY").expect("Please setting RIOT_API_KEY");

    // // let name = sn;
    // let name = sn;
    // let region = "jp";


    // let summoner_v4_resp = SummonerV4::fetch(region, name, api_key)?;
    // let league_v4_resp_vec = LeagueV4::fetch(region, &summoner_v4_resp.id, api_key)?;


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

    // Ok(data_vec)
}