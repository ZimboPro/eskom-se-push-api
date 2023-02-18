//! A blocking client using the `reqwest` http client.
//!
//! # Optional
//! Requires the `reqwest` and `sync` features to be enabled

use http::header;
use serde::de::DeserializeOwned;

use crate::{
  allowance::{AllowanceCheck, AllowanceCheckURL},
  area_info::{AreaInfo, AreaInfoURLBuilder},
  area_nearby::{AreaNearby, AreasNearbyURLBuilder},
  area_search::{AreaSearch, AreaSearchURLBuilder},
  constants::TOKEN_KEY,
  errors::HttpError,
  get_token_from_env,
  status::{EskomStatus, EskomStatusUrl},
  topics_nearby::{TopicsNearby, TopicsNearbyUrlBuilder},
  Endpoint,
};

pub struct ReqwestBlockingCLient {
  client: reqwest::blocking::Client,
}

impl ReqwestBlockingCLient {
  /// Create new client using the `reqwest::blocking` Http client
  /// `token` is the Eskom API token
  pub fn new(token: String) -> Self {
    let mut headers = header::HeaderMap::new();
    headers.insert(
      TOKEN_KEY,
      header::HeaderValue::from_str(token.as_str()).unwrap(),
    );

    ReqwestBlockingCLient {
      client: reqwest::blocking::ClientBuilder::new()
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
    match get_token_from_env(var_name) {
      Ok(val) => {
        let mut headers = header::HeaderMap::new();
        headers.insert(
          TOKEN_KEY,
          header::HeaderValue::from_str(val.as_str()).unwrap(),
        );

        ReqwestBlockingCLient {
          client: reqwest::blocking::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap(),
        }
      }
      Err(e) => panic!("Error: {}", e),
    }
  }

  /// The current and next loadshedding statuses for South Africa and (Optional) municipal overrides
  /// `eskom` is the National status
  /// Other keys in the `status` refer to different municipalities and potential overrides from the National status; most typically present is the key for `capetown`
  pub fn get_load_shedding_status(&self) -> Result<EskomStatus, HttpError> {
    let c = EskomStatusUrl::default();
    c.reqwest_client(&self.client)
  }

  /// Obtain the `area_id` from Area Find or Area Search and use with this request. This single request has everything you need to monitor upcoming loadshedding events for the chosen suburb.
  pub fn get_area_info(&self, area_id: &str) -> Result<AreaInfo, HttpError> {
    let t = AreaInfoURLBuilder::default()
      .area_id(area_id.to_owned())
      .build()
      .map_err(|_| HttpError::AreaIdNotSet)?;
    t.reqwest_client(&self.client)
  }

  /// Find areas based on GPS coordinates (latitude and longitude).
  /// The first area returned is typically the best choice for the coordinates - as it's closest to the GPS coordinates provided. However it could be that you are in the second or third area.
  pub fn areas_nearby(&self, lat: f32, long: f32) -> Result<AreaNearby, HttpError> {
    let t = AreasNearbyURLBuilder::default()
      .latitude(lat)
      .longitude(long)
      .build()
      .map_err(|_| HttpError::LongitudeOrLatitudeNotSet {
        longitude: lat,
        latitude: long,
      })?;
    t.reqwest_client(&self.client)
  }

  /// Search area based on text
  pub fn areas_search(&self, search_term: &str) -> Result<AreaSearch, HttpError> {
    let t = AreaSearchURLBuilder::default()
      .search_term(search_term)
      .build()
      .map_err(|_| HttpError::SearchTextNotSet)?;
    t.reqwest_client(&self.client)
  }

  /// Find topics created by users based on GPS coordinates (latitude and longitude). Can use this to detect if there is a potential outage/problem nearby
  pub fn topics_nearby(&self, lat: f32, long: f32) -> Result<TopicsNearby, HttpError> {
    let t = TopicsNearbyUrlBuilder::default()
      .latitude(lat)
      .longitude(long)
      .build()
      .map_err(|_| HttpError::LongitudeOrLatitudeNotSet {
        longitude: lat,
        latitude: long,
      })?;
    t.reqwest_client(&self.client)
  }

  /// Check allowance allocated for token
  /// `NOTE`: This call doesn't count towards your quota.
  pub fn check_allowance(&self) -> Result<AllowanceCheck, HttpError> {
    let t = AllowanceCheckURL::default();
    t.reqwest_client(&self.client)
  }
}

/// A response handler for `reqwest::blocking` to map the response to the given structure or relevant error
/// ```rust
/// let statusUrl = EskomStatusUrlBuilder::default().build().unwrap();
///
/// let mut headers = header::HeaderMap::new();
/// headers.insert(TOKEN_KEY, header::HeaderValue::from_str(token).unwrap());
/// let client = reqwest::blocking::ClientBuilder::new().
///   default_headers(headers).build().unwrap();
///
/// let api_response = client.get(url_endpoint.as_str()).send();
/// let response = handle_ureq_response::<EskomStatus>(api_response);
/// ```
/// `response` is the ureq API response
/// NOTE
/// Requires the `reqwest` and `sync` features to be enabled
pub fn handle_reqwest_response_blocking<T: DeserializeOwned>(
  response: Result<reqwest::blocking::Response, reqwest::Error>,
) -> Result<T, HttpError> {
  use http::StatusCode;

  use crate::errors::APIError;

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
