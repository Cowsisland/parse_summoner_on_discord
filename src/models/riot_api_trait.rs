use crate::errors::api_error::ApiError;

pub trait V1Account {
    type T;
    fn fetch(server_region: &str, name: &str, tag: &str) -> Result<Self::T, ApiError>;
}

pub trait V4Summoner {
    type T;
    fn fetch(user_region: &str, puuid: &str) -> Result<Self::T, ApiError>;
}

pub trait V4UseSummoner {
    type T;
    fn fetch(user_region: &str, puuid: &str, count: &str) -> Result<Self::T, ApiError>;
}

// トークン取得用のトレイト（必要なければ消す）
use once_cell::sync::Lazy;

use std::env;
use dotenv::dotenv;

pub trait RiotToken {
    fn get_riot_token() -> Lazy<String> {
        Lazy::new(|| {
            dotenv().ok();
            env::var("RIOT_TOKEN").expect("Please setting RIOT_TOKEN")
        })
    }
}
