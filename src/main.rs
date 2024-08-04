extern crate url;
extern crate reqwest;

use std::env;
use dotenv::dotenv;

use parse_summoner_on_discord::logic::calc_mastery;
use once_cell::sync::Lazy;

static RIOT_TOKEN: Lazy<String> = Lazy::new(|| {
    dotenv().ok();
    env::var("RIOT_TOKEN").expect("Please setting RIOT_TOKEN")
});

fn main() {
    let sn = "HASAKI PTSD";
    let tag = "JP1";
    println!("{:?}", calc_mastery::resp_mastery(sn, tag, &RIOT_TOKEN));

    // まだ動かない
    // println!("{:?}", resp_league(sn, tag));

    // let token = env::var("DISCORD_TOKEN")
    //     .expect("Expected a token in the environment");
    // let mut client = Client::new(&token, Handler).expect("Err creating client");

    // if let Err(why) = client.start() {
    //     println!("Client error: {:?}", why);
    // }
}
