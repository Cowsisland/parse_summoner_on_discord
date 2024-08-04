extern crate url;
extern crate reqwest;

use std::{collections::BTreeMap, env};
use dotenv::dotenv;
use libs::get_champ_id_map;
use logic::{calc_mastery, calc_rank};
use once_cell::sync::Lazy;

mod libs;
mod logic;
mod models;

pub static RIOT_TOKEN: Lazy<String> = Lazy::new(|| {
    dotenv().ok();
    env::var("RIOT_TOKEN").expect("Please setting RIOT_TOKEN")
});

static DISCORD_TOKEN: Lazy<String> = Lazy::new(|| {
    dotenv().ok();
    env::var("DISCORD_TOKEN").expect("Please setting DISCORD_TOKEN")
});

pub static CHAMPION_HASHMAP: Lazy<BTreeMap<u32, String>> = Lazy::new(|| {
    get_champ_id_map::get_champ_id_map()
});

fn main() {
    let sn = "HASAKI PTSD";
    let tag = "JP1";
    println!("{:?}", calc_mastery::resp_mastery(sn, tag));
    // println!("{:?}", calc_rank::resp_league(sn, tag, &*RIOT_TOKEN));

    // まだ動かない
    // lazyの型は DISCORD_TOKEN の実体を見る必要がある
    println!("DISCORD_TOKEN: {}", *DISCORD_TOKEN);
    // let mut client = Client::new(&*DISCORD_TOKEN, Handler).expect("Err creating client");

    // if let Err(why) = client.start() {
    //     println!("Client error: {:?}", why);
    // }
}
