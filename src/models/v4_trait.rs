use reqwest::Error;

pub trait V4Summoner: Default {
    type T;
    fn fetch(region: &str, name: &str, tag: &str, api_key: &str) -> Result<Self::T, Error>;
}

pub trait V4UseSummoner: Default {
    type T;
    fn fetch(region: &str, summoner_id: &str, api_key: &str) -> Result<Self::T, Error>;
}