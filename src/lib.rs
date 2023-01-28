use allowance::AllowanceCheck;
use area_info::AreaInfo;
use area_nearby::AreaNearby;
use area_search::AreaSearch;
use reqwest::Response;
use serde::de::DeserializeOwned;
use status::EskomStatus;
use topics_nearby::TopicsNearby;
use dotenv;
extern crate thiserror;

pub mod allowance;
pub mod area_info;
pub mod area_nearby;
pub mod area_search;
pub mod status;
pub mod topics_nearby;

pub fn add(left: usize, right: usize) -> usize {
  left + right
}

pub struct EskomAPIAsync {
  client: reqwest::Client,
}

pub struct EskomAPI {
  client: reqwest::blocking::Client,
}

#[derive(thiserror::Error, Debug)]
pub enum HttpError {
  #[error("Bad Request: {0}")]
  ResponseError(#[from] reqwest::Error), //400
  #[error("Timeout")]
  Timeout,
  #[error("No Internet")]
  NoInternet,
  #[error("UnknownError")]
  Unknown,
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
        },
        Err(_) => panic!("Environment variable: {} doesn't exist", key),
    }
  }

  /// The current and next loadshedding statuses for South Africa and (Optional) municipal overrides
  /// `eskom` is the National status
  /// Other keys in the `status` refer to different municipalities and potential overrides from the National status; most typically present is the key for `capetown`
  pub async fn status_async(&self) -> Result<EskomStatus, HttpError> {
    let t = self.client.get(Endpoints::Status.to_string()).send().await;
    return self.handle_response_async::<EskomStatus>(t).await;
  }

  /// Obtain the `id` from Area Find or Area Search and use with this request. This single request has everything you need to monitor upcoming loadshedding events for the chosen suburb.
  pub async fn area_info_async(&self, id: &str) -> Result<AreaInfo, HttpError> {
    let t = self
      .client
      .get(Endpoints::AreaInfo.to_string())
      .query(&[("id", id)])
      .send()
      .await;
    return self.handle_response_async::<AreaInfo>(t).await;
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
    return self.handle_response_async::<AreaNearby>(t).await;
  }

  /// Search area based on text
  pub async fn areas_search_async(&self, search_term: &str) -> Result<AreaSearch, HttpError> {
    let t = self
      .client
      .get(Endpoints::AreasSearch.to_string())
      .query(&[("text", search_term)])
      .send()
      .await;
    return self.handle_response_async::<AreaSearch>(t).await;
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
    return self.handle_response_async::<TopicsNearby>(t).await;
  }

  /// Check allowance allocated for token
  /// `NOTE`: This call doesn't count towards your quota.
  pub async fn check_allowance_async(&self) -> Result<AllowanceCheck, HttpError> {
    let t = self
      .client
      .get(Endpoints::CheckAllowace.to_string())
      .send()
      .await;
    return self.handle_response_async::<AllowanceCheck>(t).await;
  }

  async fn handle_response_async<T: DeserializeOwned>(
    &self,
    response: Result<Response, reqwest::Error>,
  ) -> Result<T, HttpError> {
    return match response {
      Ok(resp) => {
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
      Err(err) => {
        if err.is_timeout() {
          Err(HttpError::Timeout)
        } else if err.is_status() {
          Err(HttpError::ResponseError(err))
        } else {
          Err(HttpError::NoInternet)
        }
      }
    };
  }
}

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
        },
        Err(_) => panic!("Environment variable: {} doesn't exist", key),
    }
  }

  /// The current and next loadshedding statuses for South Africa and (Optional) municipal overrides
  /// `eskom` is the National status
  /// Other keys in the `status` refer to different municipalities and potential overrides from the National status; most typically present is the key for `capetown`
  pub fn status(&self) -> Result<EskomStatus, HttpError> {
    let t = self.client.get(Endpoints::Status.to_string()).send();
    return self.handle_response::<EskomStatus>(t);
  }

  /// Obtain the `id` from Area Find or Area Search and use with this request. This single request has everything you need to monitor upcoming loadshedding events for the chosen suburb.
  pub fn area_info(&self, id: &str) -> Result<AreaInfo, HttpError> {
    let t = self
      .client
      .get(Endpoints::AreaInfo.to_string())
      .query(&[("id", id)])
      .send();
    return self.handle_response::<AreaInfo>(t);
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
    return self.handle_response::<AreaNearby>(t);
  }

  /// Search area based on text
  pub fn areas_search(&self, search_term: &str) -> Result<AreaSearch, HttpError> {
    let t = self
      .client
      .get(Endpoints::AreasSearch.to_string())
      .query(&[("text", search_term)])
      .send();
    return self.handle_response::<AreaSearch>(t);
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
    return self.handle_response::<TopicsNearby>(t);
  }

  /// Check allowance allocated for token
  /// `NOTE`: This call doesn't count towards your quota.
  pub fn check_allowance(&self) -> Result<AllowanceCheck, HttpError> {
    let t = self.client.get(Endpoints::CheckAllowace.to_string()).send();
    return self.handle_response::<AllowanceCheck>(t);
  }

  fn handle_response<T: DeserializeOwned>(
    &self,
    response: Result<reqwest::blocking::Response, reqwest::Error>,
  ) -> Result<T, HttpError> {
    return match response {
      Ok(resp) => {
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
      Err(err) => {
        if err.is_timeout() {
          Err(HttpError::Timeout)
        } else if err.is_status() {
          Err(HttpError::ResponseError(err))
        } else {
          Err(HttpError::NoInternet)
        }
      }
    };
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
