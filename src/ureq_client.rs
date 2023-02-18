//! A blocking client using the `ureq` http client.
//!
//! # Optional
//! Requires the `ureq` feature to be enabled

use serde::de::DeserializeOwned;

use crate::{
  allowance::{AllowanceCheck, AllowanceCheckURL},
  area_info::{AreaInfo, AreaInfoURLBuilder},
  area_nearby::{AreaNearby, AreasNearbyURLBuilder},
  area_search::{AreaSearch, AreaSearchURLBuilder},
  errors::{APIError, HttpError},
  get_token_from_env,
  status::{EskomStatus, EskomStatusUrl},
  topics_nearby::{TopicsNearby, TopicsNearbyUrlBuilder},
  Endpoint,
};

pub struct UreqClient {
  token: String,
}

impl UreqClient {
  /// Create new client using the `ureq` Http client
  /// `token` is the Eskom API token
  pub fn new(token: String) -> Self {
    UreqClient { token }
  }

  /// Creates new instance of Eskom API using token as a env variable.
  /// Uses the [dotenv](https://crates.io/crates/dotenv) crate so it will load .env files if available.
  /// `Note`: The default variable name is `ESKOMSEPUSH_API_KEY` if var_name is set to `None`.
  /// `Note`: It will panic the env variable doesn't exist.
  pub fn new_with_env(var_name: Option<&str>) -> Self {
    match get_token_from_env(var_name) {
      Ok(val) => UreqClient { token: val },
      Err(e) => panic!("Error: {}", e),
    }
  }

  /// The current and next loadshedding statuses for South Africa and (Optional) municipal overrides
  /// `eskom` is the National status
  /// Other keys in the `status` refer to different municipalities and potential overrides from the National status; most typically present is the key for `capetown`
  pub fn get_load_shedding_status(&self) -> Result<EskomStatus, HttpError> {
    let c = EskomStatusUrl::default();
    c.ureq(&self.token)
  }

  /// Obtain the `area_id` from Area Find or Area Search and use with this request. This single request has everything you need to monitor upcoming loadshedding events for the chosen suburb.
  pub fn get_area_info(&self, area_id: &str) -> Result<AreaInfo, HttpError> {
    let t = AreaInfoURLBuilder::default()
      .area_id(area_id.to_owned())
      .build()
      .map_err(|_| HttpError::AreaIdNotSet)?;
    t.ureq(&self.token)
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
    t.ureq(&self.token)
  }

  /// Search area based on text
  pub fn areas_search(&self, search_term: &str) -> Result<AreaSearch, HttpError> {
    let t = AreaSearchURLBuilder::default()
      .search_term(search_term)
      .build()
      .map_err(|_| HttpError::SearchTextNotSet)?;
    t.ureq(&self.token)
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
    t.ureq(&self.token)
  }

  /// Check allowance allocated for token
  /// `NOTE`: This call doesn't count towards your quota.
  pub fn check_allowance(&self) -> Result<AllowanceCheck, HttpError> {
    let t = AllowanceCheckURL::default();
    t.ureq(&self.token)
  }
}

/// A response handler for `ureq` to map the response to the given structure or relevant error
/// ```rust
/// let statusUrl = EskomStatusUrlBuilder::default().build().unwrap();
///
/// let api_response  = ureq::get(statusUrl.url().unwrap().as_str()).set(TOKEN_KEY, "YOUR-TOKEN").call();
/// let response = handle_ureq_response::<EskomStatus>(api_response);
/// ```
/// `response` is the ureq API response
/// NOTE
/// Requires the `ureq` feature to be enabled
pub fn handle_ureq_response<T: DeserializeOwned>(
  response: Result<ureq::Response, ureq::Error>,
) -> Result<T, HttpError> {
  match response {
    Ok(resp) => resp
      .into_json::<T>()
      .map_err(|e| HttpError::UnknownError(e.to_string())),
    Err(ureq::Error::Status(code, response)) => match code {
      400 => Err(HttpError::APIError(APIError::BadRequest)),
      403 => Err(HttpError::APIError(APIError::Forbidden)),
      404 => Err(HttpError::APIError(APIError::NotFound)),
      429 => Err(HttpError::APIError(APIError::TooManyRequests)),
      500..=509 => Err(HttpError::APIError(APIError::ServerError(
        response.into_string().unwrap(),
      ))),
      _a => {
        Err(HttpError::Unknown)
      }
    },
    Err(_) => Err(HttpError::NoInternet),
  }
}
