extern crate url;
extern crate reqwest;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::collections::BTreeMap;
use dotenv::dotenv;
// use std::process;
use reqwest::Error;
use serde::{Deserialize, Serialize};



// "RESPONSE BODY"から構造体を作成
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct Summoner_v4 {
    id: String,
    accountId: String,
    puuid: String,
    name: String,
    profileIconId: i32,
    revisionDate: i64,
    summonerLevel: i64
}

// // defaultトレイトを使って構造体を初期化
impl Default for Summoner_v4 {
    fn default() -> Self {
        Self {
            id: String::default(),
            accountId: String::default(),
            puuid: String::default(),
            name: String::default(),
            profileIconId: i32::default(),
            revisionDate: i64::default(),
            summonerLevel: i64::default(),
        }
    }
}

//  Summoner_v4の構造体を取得
impl Summoner_v4 {
    fn fetch_summoner_v4(region: &str, name: &str, api_key: &str) -> Result<Self, Error> {
        let url = format! (
            "https://{}1.api.riotgames.com/lol/summoner/v4/summoners/by-name/{}?api_key={}",
            region,
            name,
            api_key
        );

        let resp: Self = reqwest::blocking::get(&url)?.json()?;

        Ok(resp)
    }
}



// response_bodyから構造体を作成
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct Champion_mastery_v4 {
    championId: i64,
    championLevel: i32,
    championPoints: i32,
    lastPlayTime: i64,
    championPointsSinceLastLevel: i64,
    championPointsUntilNextLevel: i64,
    chestGranted: bool,
    tokensEarned: i32,
    summonerId: String
}

// defaultトレイトを使って構造体を初期化
impl Default for Champion_mastery_v4 {
    fn default() -> Self {
        Self{
            championId: i64::default(),
            championLevel: i32::default(),
            championPoints: i32::default(),
            lastPlayTime: i64::default(),
            championPointsSinceLastLevel: i64::default(),
            championPointsUntilNextLevel: i64::default(),
            chestGranted: bool::default(),
            tokensEarned: i32::default(),
            summonerId: String::default(),
        }
    }
}

// Champion_mastery_v4の構造体を取得
impl Champion_mastery_v4 {
    fn fetch_champion_mastery_v4(region: &str, summoner_id: &str, api_key: &str) -> Result<Vec<Self>, Error> {
        let url = format! (
            "https://{}1.api.riotgames.com/lol/champion-mastery/v4/champion-masteries/by-summoner/{}?api_key={}",
            region,
            summoner_id,
            api_key
        );

        let resp: Vec<Self> = reqwest::blocking::get(&url)?.json()?;

        Ok(resp)
    }
}



fn get_champ_id_map() -> BTreeMap<i64, String> {
    let file = File::open("./data/champion_id.json").unwrap();
    let reader = BufReader::new(file);

    let champ_id_map: BTreeMap<i64, String> = serde_json::from_reader(reader).unwrap();
    
    champ_id_map
}



fn resp_lol_data(sn: &str) -> Result<Vec<String>, Error> {

    // to use env
    dotenv().ok();

    // Check Riot API
    let api_key = &env::var("RIOT_API_KEY").expect("Please setting RIOT_API_KEY");
    let champ_id_map = get_champ_id_map();
   
    // let name = sn;
    println!("SN = {}", sn);
    let name = sn;
    let region = "jp";


    let summoner_v4_resp = Summoner_v4::fetch_summoner_v4(region, name, api_key)?;

    let champion_mastery_v4_resp = Champion_mastery_v4::fetch_champion_mastery_v4(region, &summoner_v4_resp.id, api_key)?;


    let loop_num: usize;
    if champion_mastery_v4_resp.len() < 7 {
        loop_num = champion_mastery_v4_resp.len();
    } else {
        loop_num = 7;
    }


    let mut data_vec = Vec::new();
    for i in 0..loop_num {
        let data_str = format!("{: >4}) {: <9} {: >7} ({})",
            i+1,
            champ_id_map.get(&champion_mastery_v4_resp[i].championId).unwrap(),
            champion_mastery_v4_resp[i].championPoints,
            champion_mastery_v4_resp[i].championLevel
        );
        data_vec.push(data_str);
    }

    Ok(data_vec)
}



use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!") {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content.starts_with("!mastary") {
            let sn: Vec<&str> = msg.content.split_whitespace().collect();
            match sn.len() {
                2 => {
                    match resp_lol_data(sn[1]) {
                        Ok(data_vec) => { 
                            for data in data_vec {
                                if let Err(why) = msg.channel_id.say(&ctx.http, data) {
                                    println!("Error sending message: {:?}", why);
                                }
                            }
                        },
                        Err(err) => {
                            println!("{}", err);
                        }
                    };
                }
                _ => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, "!mastary SN を入力して下さい") {
                        println!("Error sending message: {:?}", why);
                    }
                }
            } 
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}



fn main() {

    // Check Discord Token    
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
