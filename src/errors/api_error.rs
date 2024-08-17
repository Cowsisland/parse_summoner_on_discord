use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub message: String,
    pub status_code: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiErrorResponse {
    pub status: Status,
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Request failed with status {status}: {message}")]
    RequestFailed { status: reqwest::StatusCode, message: String},
    
    #[error("Failed to deserialize response: {0}")]
    DeserializeError(#[from] serde_json::Error),
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
}

// impl From<reqwest::Error> for ApiError {
//     fn from(err: reqwest::Error) -> Self {
//         ApiError::Reqwest(err)
//     }
// }

// #[cfg(test)]
// mod test {
//     use serde::Deserialize;
//     use serde_json::Value;
//     use crate::errors::api_error::{ApiErrorResponse, ApiError};

//     #[test]
//     #[should_panic(expected = "API Error. Response is")]
//     fn test_error() {
//         #[derive(Deserialize, Debug)]
//         pub struct SummonerV4 {
//             pub id: String,
//             pub account_id: String,
//             pub puuid: String,
//             pub profile_icon_id: u32,
//             pub revision_date: i64,
//             pub summoner_level: u32,
//         }
        
//         #[derive(Deserialize, Debug)]
//         pub enum Response {
//             Success(SummonerV4),
//             Error(ApiErrorResponse),
//         }

//         let json_data = r#"
//         {
//             "status": {
//                 "message": "Forbidden",
//                 "status_code": 403
//             }
//         }
//         "#;
    
//         // let response: Result<SummonerV4, ApiError> = serde_json::from_str(json_data).expect("error");
//         // println!("{:#?}", response);
//         // let summoner_v4_response = match response {
//         //     Response::Success(data) => data,
//         //     Response::Error(api_error) => panic!("{}", api_error),
//         // };
//         // println!("{:#?}", summoner_v4_response);
//         //assert_eq!(summoner_v4_response, "api_error");
//     }
// }