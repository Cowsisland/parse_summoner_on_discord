extern crate url;
extern crate reqwest;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::collections::BTreeMap;
use dotenv::dotenv;
use reqwest::Error;

use parse_summoner_on_discord::{models::{champion_mastery_v4::ChampionMasteryV4, summoner_v4::SummonerV4, v4_traits::{V4Summoner, V4UseSummoner}}};
use parse_summoner_on_discord::libs::get_champ_id_map;

fn resp_mastery(sn: &str, tag: &str) -> Result<Vec<String>, Error> {
    // to use env
    dotenv().ok();

    // Check Riot API
    let riot_token = &env::var("RIOT_TOKEN").expect("Please setting RIOT_TOKEN");
    let server_region: &str = "asia";

    let summoner_v4_resp = SummonerV4::fetch(server_region, sn, tag, riot_token)?;
    println!("{:?}", summoner_v4_resp);


    let puuid = &summoner_v4_resp.puuid;
    let user_region = "jp1";
    let count = "3";

    let champion_mastery_v4_resp: Vec<ChampionMasteryV4> = ChampionMasteryV4::fetch(user_region, puuid, count, riot_token)?;
    println!("{:?}", champion_mastery_v4_resp);

    // iterで回してマスタリーのランキングとポイントを取得する
    // マスタリーのランキングをcountの数だけ表示
    let champ_id_map = get_champ_id_map::get_champ_id_map();

    // let mut data_vec = Vec::new();
    // for i in 0..loop_num {
    //     let data_str = format!("{: <4} {: <12} {: <8} ({})",
    //         (i+1).to_string() +")",
    //         champ_id_map.get(&champion_mastery_v4_resp[i].champion_id).unwrap(),
    //         champion_mastery_v4_resp[i].champion_points,
    //         champion_mastery_v4_resp[i].champion_level
    //     );
    //     data_vec.push(data_str);
    // }

    let data_vec = vec![];
    Ok(data_vec)
}



fn resp_league(sn: &str, tag: &str) -> Result<Vec<String>, Error> {
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


// use serenity::{
//     model::{channel::Message, gateway::Ready},
//     prelude::*,
// };

// struct Handler;

// impl EventHandler for Handler {
//     fn message(&self, ctx: Context, msg: Message) {
//         if msg.content == "!ping" {
//             if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!") {
//                 println!("Error sending message: {:?}", why);
//             }
//         } else if msg.content.starts_with("mastery:") {
//             let sn: Vec<&str> = msg.content.split(":").collect();
//             if sn.len() >= 2 {
//                 match resp_mastery(sn[1]) {
//                     Ok(data_vec) => {
//                         let data = data_vec.join("\n");
//                         if let Err(why) = msg.channel_id.say(&ctx.http, data) {
//                             println!("Error sending message: {:?}", why);
//                         }
//                     },
//                     Err(err) => {
//                         println!("{}", err);
//                     }
//                 }
//             }
//         } else if  msg.content.starts_with("rate:") {
//             let sn: Vec<&str> = msg.content.split(":").collect();
//             if sn.len() >= 2 {
//                 match resp_league(sn[1]) {
//                     Ok(data_vec) => {
//                         let data = data_vec.join("\n");
//                         if let Err(why) = msg.channel_id.say(&ctx.http, data) {
//                             println!("Error sending message: {:?}", why);
//                         }
//                     },
//                     Err(err) => {
//                         println!("{}", err);
//                     }
//                 }
//             }
//         }
//     }

//     fn ready(&self, _: Context, ready: Ready) {
//         println!("{} is connected!", ready.user.name);
//     }
// }

fn main() {   
    // 環境変数取得のためのチェック
    dotenv().ok();
    let riot_token = &env::var("RIOT_TOKEN").expect("Please setting RIOT_TOKEN");
    println!("{}", riot_token);

    let sn = "HASAKI PTSD";
    let tag = "JP1";
    println!("{:?}", resp_mastery(sn, tag));

    // まだ動かない
    // println!("{:?}", resp_league(sn, tag));

    // let token = env::var("DISCORD_TOKEN")
    //     .expect("Expected a token in the environment");
    // let mut client = Client::new(&token, Handler).expect("Err creating client");

    // if let Err(why) = client.start() {
    //     println!("Client error: {:?}", why);
    // }
}
