extern crate url;
extern crate reqwest;

use std::env;
use dotenv::dotenv;
use logic::{calc_mastery, calc_rank};
use once_cell::sync::Lazy;

mod libs;
mod logic;
mod models;
mod errors;

pub static RIOT_TOKEN: Lazy<String> = Lazy::new(|| {
    dotenv().ok();
    env::var("RIOT_TOKEN").expect("Please setting RIOT_TOKEN")
});

static DISCORD_TOKEN: Lazy<String> = Lazy::new(|| {
    dotenv().ok();
    env::var("DISCORD_TOKEN").expect("Please setting DISCORD_TOKEN")
});

fn main() {
    let sn = "The Runic Blade";
    let tag = "JP1";
    println!("{:?}", calc_mastery::resp_mastery(sn, tag));
    println!("{:?}", calc_rank::resp_league(sn, tag));

    // まだ動かない
    // lazyの型は DISCORD_TOKEN の実体を見る必要がある
    println!("DISCORD_TOKEN: {}", *DISCORD_TOKEN);
    // let mut client = Client::new(&*DISCORD_TOKEN, Handler).expect("Err creating client");

    // if let Err(why) = client.start() {
    //     println!("Client error: {:?}", why);
    // }
}
