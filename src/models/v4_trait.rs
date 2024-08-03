use reqwest::Error;

pub trait V4: Default {
    type T;
    fn fetch(region: &str, name: &str, api_key: &str) -> Result<Self::T, Error>;
}