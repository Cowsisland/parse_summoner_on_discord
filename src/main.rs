extern crate url;
extern crate reqwest;

use std::env;
use dotenv::dotenv;

use parse_summoner_on_discord::{libs::discord_handler::Handler, logic::{calc_mastery, calc_rank}};
use once_cell::sync::Lazy;
use serenity::Client;

static RIOT_TOKEN: Lazy<String> = Lazy::new(|| {
    dotenv().ok();
    env::var("RIOT_TOKEN").expect("Please setting RIOT_TOKEN")
});

static DISCORD_TOKEN: Lazy<String> = Lazy::new(|| {
    dotenv().ok();
    env::var("DISCORD_TOKEN").expect("Please setting DISCORD_TOKEN")
});

fn main() {
    let sn = "HASAKI PTSD";
    let tag = "JP1";
    println!("{:?}", calc_mastery::resp_mastery(sn, tag, &RIOT_TOKEN));
    // println!("{:?}", calc_rank::resp_league(sn, tag, &RIOT_TOKEN));

    // まだ動かない
    // let mut client = Client::new(&*DISCORD_TOKEN, Handler).expect("Err creating client");

    // if let Err(why) = client.start() {
    //     println!("Client error: {:?}", why);
    // }
}
