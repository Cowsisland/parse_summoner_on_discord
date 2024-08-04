use once_cell::sync::Lazy;

use std::env;
use dotenv::dotenv;

trait RiotToken {
    fn get_riot_token() -> Lazy<String> {
        Lazy::new(|| {
            dotenv().ok();
            env::var("RIOT_TOKEN").expect("Please setting RIOT_TOKEN")
        })
    }
}