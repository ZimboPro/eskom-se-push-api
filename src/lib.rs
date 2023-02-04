//! # Eskom-se-Push API
//! 
//! The library is an unofficial lib and is not maintained by the API developers.
//! 
//! This library is an API binding to the [EskomSePush](https://sepush.co.za) API.
//! 
//! It does have a few small helper functions to assist.
//! 
//! To get up and running, you can either pass in the API key or use environment variables.
//! 
//! ## API key as a variable
//! 
//! ```rust
//! use eskom_se_push_api::EskomAPI;
//!
//! fn main() {
//!   let api = EskomAPI::new("XXXXXXXXXXXXXXXXXXXXXXXXX");
//!   let resp = api.check_allowance();
//!   match resp {
//!     Ok(allowance) => {
//!       println!(
//!         "You have made {} API calls today",
//!         allowance.allowance.count
//!       );
//!     }
//!     Err(e) => {
//!       eprintln!("Error: {}", e);
//!     }
//!   }
//! }
//! ```
//! 
//! ## API key as an env variable
//! 
//! The default env variable is `ESKOMSEPUSH_API_KEY`
//! ```rust
//! use eskom_se_push_api::EskomAPI;
//!
//! fn main() {
//!   let api = EskomAPI::new_with_env(None);
//!   let resp = api.check_allowance();
//!   match resp {
//!     Ok(allowance) => {
//!       println!(
//!         "You have made {} API calls today",
//!         allowance.allowance.count
//!       );
//!     }
//!     Err(e) => {
//!       eprintln!("Error: {}", e);
//!     }
//!   }
//! }
//! ```
//! 
//! //! 
//! ## API key as an custom env variable
//! 
//! Able to use custom env keys such as `MY_CUSTOM_KEY`
//! ```rust
//! use eskom_se_push_api::EskomAPI;
//!
//! fn main() {
//!   let api = EskomAPI::new_with_env(Some("MY_CUSTOM_KEY"));
//!   let resp = api.check_allowance();
//!   match resp {
//!     Ok(allowance) => {
//!       println!(
//!         "You have made {} API calls today",
//!         allowance.allowance.count
//!       );
//!     }
//!     Err(e) => {
//!       eprintln!("Error: {}", e);
//!     }
//!   }
//! }
//! ```

use allowance::AllowanceCheck;
use area_info::AreaInfo;
use area_nearby::AreaNearby;
use area_search::AreaSearch;

use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;
use status::EskomStatus;
use topics_nearby::TopicsNearby;
extern crate thiserror;

pub mod allowance;
pub mod area_info;
pub mod area_nearby;
pub mod area_search;
pub mod status;
pub mod topics_nearby;

#[cfg(feature = "async")]
pub struct EskomAPIAsync {
  client: reqwest::Client,
}

#[cfg(feature = "sync")]
pub struct EskomAPI {
  client: reqwest::blocking::Client,
}

#[derive(thiserror::Error, Debug)]
pub enum HttpError {
  #[error("API Error: {0}")]
  APIError(#[from] APIError), //400
  #[error("Timeout")]
  Timeout,
  #[error("No Internet")]
  NoInternet,
  #[error("UnknownError")]
  Unknown,
  #[error("Response Error: {0}")]
  ResponseError(#[from] reqwest::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum APIError {
  #[error("Bad Request (You sent something bad)")]
  BadRequest,
  #[error("Not Authenticated (Token Invalid / Disabled)")]
  Forbidden,
  #[error("Not found")]
  NotFound,
  #[error("Too Many Requests (Token quota exceeded)")]
  TooManyRequests,
}

enum Endpoints {
  Status,
  AreaInfo,
  AreasNearby,
  AreasSearch,
  TopicsNearby,
  CheckAllowace,
}

impl ToString for Endpoints {
  fn to_string(&self) -> String {
    match self {
      Endpoints::Status => "https://developer.sepush.co.za/business/2.0/status".to_string(),
      Endpoints::AreaInfo => "https://developer.sepush.co.za/business/2.0/area".to_string(),
      Endpoints::AreasNearby => {
        "https://developer.sepush.co.za/business/2.0/areas_nearby".to_string()
      }
      Endpoints::AreasSearch => {
        "https://developer.sepush.co.za/business/2.0/areas_search".to_string()
      }
      Endpoints::TopicsNearby => {
        "https://developer.sepush.co.za/business/2.0/topics_nearby".to_string()
      }
      Endpoints::CheckAllowace => {
        "https://developer.sepush.co.za/business/2.0/api_allowance".to_string()
      }
    }
  }
}

#[cfg(feature = "async")]
impl EskomAPIAsync {
  pub fn new(token: &str) -> Self {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
      "token",
      reqwest::header::HeaderValue::from_str(token).unwrap(),
    );
    EskomAPIAsync {
      client: reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap(),
    }
  }

  /// Creates new instance of Eskom API using token as a env variable.
  /// Uses the [dotenv](https://crates.io/crates/dotenv) crate so it will load .env files if available.
  /// `Note`: The default variable name is `ESKOMSEPUSH_API_KEY` if var_name is set to `None`.
  /// `Note`: It will panic the env variable doesn't exist.
  pub fn new_with_env(var_name: Option<&str>) -> Self {
    dotenv::dotenv().ok();
    let key = var_name.unwrap_or("ESKOMSEPUSH_API_KEY");
    match std::env::var(key) {
      Ok(val) => {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
          "token",
          reqwest::header::HeaderValue::from_str(&val).unwrap(),
        );
        EskomAPIAsync {
          client: reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap(),
        }
      }
      Err(_) => panic!("Environment variable: {} doesn't exist", key),
    }
  }

  /// The current and next loadshedding statuses for South Africa and (Optional) municipal overrides
  /// `eskom` is the National status
  /// Other keys in the `status` refer to different municipalities and potential overrides from the National status; most typically present is the key for `capetown`
  pub async fn status_async(&self) -> Result<EskomStatus, HttpError> {
    let t = self.client.get(Endpoints::Status.to_string()).send().await;
    self.handle_response_async::<EskomStatus>(t).await
  }

  /// Obtain the `id` from Area Find or Area Search and use with this request. This single request has everything you need to monitor upcoming loadshedding events for the chosen suburb.
  pub async fn area_info_async(&self, id: &str) -> Result<AreaInfo, HttpError> {
    let t = self
      .client
      .get(Endpoints::AreaInfo.to_string())
      .query(&[("id", id)])
      .send()
      .await;
    self.handle_response_async::<AreaInfo>(t).await
  }

  /// Find areas based on GPS coordinates (latitude and longitude).
  /// The first area returned is typically the best choice for the coordinates - as it's closest to the GPS coordinates provided. However it could be that you are in the second or third area.
  pub async fn areas_nearby_async(
    &self,
    lat: impl ToString,
    long: impl ToString,
  ) -> Result<AreaNearby, HttpError> {
    let t = self
      .client
      .get(Endpoints::AreasNearby.to_string())
      .query(&[("lat", lat.to_string()), ("lon", long.to_string())])
      .send()
      .await;
    self.handle_response_async::<AreaNearby>(t).await
  }

  /// Search area based on text
  pub async fn areas_search_async(&self, search_term: &str) -> Result<AreaSearch, HttpError> {
    let t = self
      .client
      .get(Endpoints::AreasSearch.to_string())
      .query(&[("text", search_term)])
      .send()
      .await;
    self.handle_response_async::<AreaSearch>(t).await
  }

  /// Find topics created by users based on GPS coordinates (latitude and longitude). Can use this to detect if there is a potential outage/problem nearby
  pub async fn topics_nearby_async(
    &self,
    lat: impl ToString,
    long: impl ToString,
  ) -> Result<TopicsNearby, HttpError> {
    let t = self
      .client
      .get(Endpoints::TopicsNearby.to_string())
      .query(&[("lat", lat.to_string()), ("lon", long.to_string())])
      .send()
      .await;
    self.handle_response_async::<TopicsNearby>(t).await
  }

  /// Check allowance allocated for token
  /// `NOTE`: This call doesn't count towards your quota.
  pub async fn check_allowance_async(&self) -> Result<AllowanceCheck, HttpError> {
    let t = self
      .client
      .get(Endpoints::CheckAllowace.to_string())
      .send()
      .await;
    self.handle_response_async::<AllowanceCheck>(t).await
  }

  async fn handle_response_async<T: DeserializeOwned>(
    &self,
    response: Result<Response, reqwest::Error>,
  ) -> Result<T, HttpError> {
    match response {
      Ok(resp) => {
        let status_code = resp.status();
        if status_code.is_server_error() {
          Err(HttpError::ResponseError(
            resp.error_for_status().unwrap_err(),
          ))
        } else {
          match status_code {
            StatusCode::BAD_REQUEST => Err(HttpError::APIError(APIError::BadRequest)),
            StatusCode::FORBIDDEN => Err(HttpError::APIError(APIError::Forbidden)),
            StatusCode::NOT_FOUND => Err(HttpError::APIError(APIError::NotFound)),
            StatusCode::TOO_MANY_REQUESTS => Err(HttpError::APIError(APIError::TooManyRequests)),
            _ => {
              let r = resp.json::<T>().await;
              match r {
                Ok(r) => Ok(r),
                Err(e) => {
                  if e.is_decode() {
                    Err(HttpError::ResponseError(e))
                  } else {
                    Err(HttpError::Unknown)
                  }
                }
              }
            }
          }
        }
      }
      Err(err) => {
        if err.is_timeout() {
          Err(HttpError::Timeout)
        } else if err.is_status() {
          Err(HttpError::ResponseError(err))
        } else {
          Err(HttpError::NoInternet)
        }
      }
    }
  }
}

#[cfg(feature = "sync")]
impl EskomAPI {
  pub fn new(token: &str) -> Self {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
      "token",
      reqwest::header::HeaderValue::from_str(token).unwrap(),
    );
    EskomAPI {
      client: reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap(),
    }
  }

  /// Creates new instance of Eskom API using token as a env variable.
  /// Uses the [dotenv](https://crates.io/crates/dotenv) crate so it will load .env files if available.
  /// `Note`: The default variable name is `ESKOMSEPUSH_API_KEY` if var_name is set to `None`.
  /// `Note`: It will panic the env variable doesn't exist.
  pub fn new_with_env(var_name: Option<&str>) -> Self {
    dotenv::dotenv().ok();
    let key = var_name.unwrap_or("ESKOMSEPUSH_API_KEY");
    match std::env::var(key) {
      Ok(val) => {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
          "token",
          reqwest::header::HeaderValue::from_str(&val).unwrap(),
        );
        EskomAPI {
          client: reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap(),
        }
      }
      Err(_) => panic!("Environment variable: {} doesn't exist", key),
    }
  }

  /// The current and next loadshedding statuses for South Africa and (Optional) municipal overrides
  /// `eskom` is the National status
  /// Other keys in the `status` refer to different municipalities and potential overrides from the National status; most typically present is the key for `capetown`
  pub fn status(&self) -> Result<EskomStatus, HttpError> {
    let t = self.client.get(Endpoints::Status.to_string()).send();
    self.handle_response::<EskomStatus>(t)
  }

  /// Obtain the `id` from Area Find or Area Search and use with this request. This single request has everything you need to monitor upcoming loadshedding events for the chosen suburb.
  pub fn area_info(&self, id: &str) -> Result<AreaInfo, HttpError> {
    let t = self
      .client
      .get(Endpoints::AreaInfo.to_string())
      .query(&[("id", id)])
      .send();
    self.handle_response::<AreaInfo>(t)
  }

  /// Find areas based on GPS coordinates (latitude and longitude).
  /// The first area returned is typically the best choice for the coordinates - as it's closest to the GPS coordinates provided. However it could be that you are in the second or third area.
  pub fn areas_nearby_async(
    &self,
    lat: impl ToString,
    long: impl ToString,
  ) -> Result<AreaNearby, HttpError> {
    let t = self
      .client
      .get(Endpoints::AreasNearby.to_string())
      .query(&[("lat", lat.to_string()), ("lon", long.to_string())])
      .send();
    self.handle_response::<AreaNearby>(t)
  }

  /// Search area based on text
  pub fn areas_search(&self, search_term: &str) -> Result<AreaSearch, HttpError> {
    let t = self
      .client
      .get(Endpoints::AreasSearch.to_string())
      .query(&[("text", search_term)])
      .send();
    self.handle_response::<AreaSearch>(t)
  }

  /// Find topics created by users based on GPS coordinates (latitude and longitude). Can use this to detect if there is a potential outage/problem nearby
  pub fn topics_nearby(
    &self,
    lat: impl ToString,
    long: impl ToString,
  ) -> Result<TopicsNearby, HttpError> {
    let t = self
      .client
      .get(Endpoints::TopicsNearby.to_string())
      .query(&[("lat", lat.to_string()), ("lon", long.to_string())])
      .send();
    self.handle_response::<TopicsNearby>(t)
  }

  /// Check allowance allocated for token
  /// `NOTE`: This call doesn't count towards your quota.
  pub fn check_allowance(&self) -> Result<AllowanceCheck, HttpError> {
    let t = self.client.get(Endpoints::CheckAllowace.to_string()).send();
    self.handle_response::<AllowanceCheck>(t)
  }

  fn handle_response<T: DeserializeOwned>(
    &self,
    response: Result<reqwest::blocking::Response, reqwest::Error>,
  ) -> Result<T, HttpError> {
    match response {
      Ok(resp) => {
        let status_code = resp.status();
        if status_code.is_server_error() {
          Err(HttpError::ResponseError(
            resp.error_for_status().unwrap_err(),
          ))
        } else {
          match status_code {
            StatusCode::BAD_REQUEST => Err(HttpError::APIError(APIError::BadRequest)),
            StatusCode::FORBIDDEN => Err(HttpError::APIError(APIError::Forbidden)),
            StatusCode::NOT_FOUND => Err(HttpError::APIError(APIError::NotFound)),
            StatusCode::TOO_MANY_REQUESTS => Err(HttpError::APIError(APIError::TooManyRequests)),
            _ => {
              let r = resp.json::<T>();
              match r {
                Ok(r) => Ok(r),
                Err(e) => {
                  if e.is_decode() {
                    Err(HttpError::ResponseError(e))
                  } else {
                    Err(HttpError::Unknown)
                  }
                }
              }
            }
          }
        }
      }
      Err(err) => {
        if err.is_timeout() {
          Err(HttpError::Timeout)
        } else if err.is_status() {
          Err(HttpError::ResponseError(err))
        } else {
          Err(HttpError::NoInternet)
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    // let result = add(2, 2);
    // assert_eq!(result, 4);
  }
}
