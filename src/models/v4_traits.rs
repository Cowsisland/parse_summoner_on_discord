use reqwest::Error;

pub trait V4Summoner {
    type T;
    fn fetch(server_region: &str, name: &str, tag: &str, api_key: &str) -> Result<Self::T, Error>;
}

pub trait V4UseSummoner {
    type T;
    fn fetch(user_region: &str, puuid: &str, count: &str, riot_token: &str) -> Result<Self::T, Error>;
}
