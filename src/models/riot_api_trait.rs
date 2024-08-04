use reqwest::Error;

pub trait V1Account {
    type T;
    fn fetch(server_region: &str, name: &str, tag: &str) -> Result<Self::T, Error>;
}

pub trait V4Summoner {
    type T;
    fn fetch(server_region: &str, name: &str, tag: &str) -> Result<Self::T, Error>;
}

pub trait V4UseSummoner {
    type T;
    fn fetch(user_region: &str, puuid: &str, count: &str) -> Result<Self::T, Error>;
}
