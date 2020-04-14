extern crate url;
extern crate reqwest;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::collections::BTreeMap;
use dotenv::dotenv;
use reqwest::Error;
use serde::{Deserialize, Serialize};



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




#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct League_v4 {
    leagueId: String,
    queueType: String,
    tier: String,
    rank: String,
    summonerId: String,
    summonerName: String,
    leaguePoints: i32,
    wins: i32,
    losses: i32,
    veteran: bool,
    inactive: bool,
    freshBlood: bool,
    hotStreak: bool
}

impl Default for League_v4 {
    fn default() -> Self {
        Self {
            leagueId: String::default(),
            queueType: String::default(),
            tier: String::default(),
            rank: String::default(),
            summonerId: String::default(),
            summonerName: String::default(),
            leaguePoints: i32::default(),
            wins: i32::default(),
            losses: i32::default(),
            veteran: bool::default(),
            inactive: bool::default(),
            freshBlood: bool::default(),
            hotStreak: bool::default()
        }
    }
}

impl League_v4 {
    fn fetch_league_v4(region: &str, id: &str, api_key: &str) -> Result<Vec<Self>, Error> {
        let url = format!("https://{}1.api.riotgames.com/lol/league/v4/entries/by-summoner/{}?api_key={}",
            region,
            id,
            api_key
        );
    
        let resp: Vec<Self> = reqwest::blocking::get(&url)?.json()?;
        Ok(resp)
    }
}



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



fn resp_mastery(sn: &str) -> Result<Vec<String>, Error> {

    // to use env
    dotenv().ok();

    // Check Riot API
    let api_key = &env::var("RIOT_API_KEY").expect("Please setting RIOT_API_KEY");
    let champ_id_map = get_champ_id_map();
   
    let name = sn;
    let region = "jp";


    let summoner_v4_resp = Summoner_v4::fetch_summoner_v4(region, name, api_key)?;

    let champion_mastery_v4_resp = Champion_mastery_v4::fetch_champion_mastery_v4(region, &summoner_v4_resp.id, api_key)?;

    let loop_num: usize;
    if champion_mastery_v4_resp.len() < 5 {
        loop_num = champion_mastery_v4_resp.len();
    } else {
        loop_num = 5;
    }


    let mut data_vec = Vec::new();
    for i in 0..loop_num {
        let data_str = format!("{: <4} {: <12} {: <8} ({})",
            (i+1).to_string() +")",
            champ_id_map.get(&champion_mastery_v4_resp[i].championId).unwrap(),
            champion_mastery_v4_resp[i].championPoints,
            champion_mastery_v4_resp[i].championLevel
        );
        data_vec.push(data_str);
    }

    Ok(data_vec)
}



fn resp_league(sn: &str) -> Result<Vec<String>, Error> {

    // to use env
    dotenv().ok();

    // Check Riot API
    let api_key = &env::var("RIOT_API_KEY").expect("Please setting RIOT_API_KEY");
   
    // let name = sn;
    let name = sn;
    let region = "jp";


    let summoner_v4_resp = Summoner_v4::fetch_summoner_v4(region, name, api_key)?;
    let league_v4_resp_vec = League_v4::fetch_league_v4(region, &summoner_v4_resp.id, api_key)?;


    let mut data_vec = Vec::new();
    data_vec.push(
        format!("{}    {}  {}    {}    {}",
            "Queue", "Tier", "Division", "LP", "Win Rate"
        )    
    );

    for league_v4_resp in league_v4_resp_vec {
        let win_rate: i32 = league_v4_resp.wins*100 / (league_v4_resp.wins+league_v4_resp.losses);
        let data_str = format!("{}    {}  {}    {}LP    {}%",
            league_v4_resp.queueType,
            league_v4_resp.tier,
            league_v4_resp.rank,
            league_v4_resp.leaguePoints,
            win_rate
        );
        data_vec.push(data_str);
    }

    Ok(data_vec)
}



// To use Discord

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
        } else if msg.content.starts_with("mastery:") {
            let sn: Vec<&str> = msg.content.split(":").collect();
            if sn.len() >= 2 {
                match resp_mastery(sn[1]) {
                    Ok(data_vec) => {
                        let data = data_vec.join("\n");
                        if let Err(why) = msg.channel_id.say(&ctx.http, data) {
                            println!("Error sending message: {:?}", why);
                        }
                    },
                    Err(err) => {
                        println!("{}", err);
                    }
                }
            }
        } else if  msg.content.starts_with("rate:") {
            let sn: Vec<&str> = msg.content.split(":").collect();
            if sn.len() >= 2 {
                match resp_league(sn[1]) {
                    Ok(data_vec) => {
                        let data = data_vec.join("\n");
                        if let Err(why) = msg.channel_id.say(&ctx.http, data) {
                            println!("Error sending message: {:?}", why);
                        }
                    },
                    Err(err) => {
                        println!("{}", err);
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
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
