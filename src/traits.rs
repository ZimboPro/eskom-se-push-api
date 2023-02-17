use std::{borrow::Cow, error::Error};

use async_trait::async_trait;
use http::StatusCode;
use reqwest::Url;
use serde::de::DeserializeOwned;

use crate::errors::{HttpError, APIError};


pub trait Endpoint {
    type Output: DeserializeOwned;
    fn method(&self) -> &str {
        "GET"
    }

    fn endpoint(&self) -> Cow<'static, str>;

    fn url(&self) -> Result<Url, HttpError>;

    #[cfg(feature = "ureq")]
    fn ureq_client(&self, client: &ureq::Agent) -> Result<Self::Output, HttpError> {
        let url_endpoint = self.url()?;
        handle_ureq_response(client.request(self.method(), url_endpoint.as_str()).call())
    }

    #[cfg(feature = "ureq")]
    fn ureq(&self) -> Result<Self::Output, HttpError> {
        let url_endpoint = self.url()?;
        handle_ureq_response(ureq::request(self.method(), url_endpoint.as_str()).call())
    }

    #[cfg(all(feature = "reqwest", feature = "sync"))]
    fn reqwest_client(&self, client: &reqwest::blocking::Client) -> Result<Self::Output, HttpError> {
        let url_endpoint = self.url()?;
        handle_reqwest_response_blocking::<Self::Output>(client.get(url_endpoint.as_str()).send())
    }

    #[cfg(all(feature = "reqwest", feature = "sync"))]
    fn reqwest(&self) -> Result<Self::Output, HttpError> {
        let url_endpoint = self.url()?;
        handle_reqwest_response_blocking::<Self::Output>(reqwest::blocking::get(url_endpoint.as_str()))
    }

    
}

#[cfg(all(feature = "reqwest", feature = "async"))]
#[async_trait]
pub trait EndpointAsync {
    #[cfg(all(feature = "reqwest", feature = "async"))]
    async fn reqwest_client_async(&self, client: &reqwest::Client) -> Result<Self::Output, HttpError> {
        let url_endpoint = self.url()?;
        handle_reqwest_response::<Self::Output>(client.get(url_endpoint.as_str()).send().await).await
    }

    #[cfg(all(feature = "reqwest", feature = "async"))]
    async fn reqwest_async(&self) -> Result<Self::Output, HttpError> {
        let url_endpoint = self.url()?;
        handle_reqwest_response::<Self::Output>(reqwest::get(url_endpoint.as_str()).await).await
    }
}

#[cfg(feature = "ureq")]
fn handle_ureq_response<T: DeserializeOwned>(response: Result<ureq::Response, ureq::Error>) -> Result<T, HttpError> {
    match response {
        Ok(response) => {
            response.into_json::<T>().map_err(|e| HttpError::UnknownError(e.to_string()))
        },
        Err(ureq::Error::Status(code, response)) => {
            match code {
                400 => Err(HttpError::APIError(APIError::BadRequest)),
                403 => Err(HttpError::APIError(APIError::Forbidden)),
                404 => Err(HttpError::APIError(APIError::NotFound)),
                429 => Err(HttpError::APIError(APIError::TooManyRequests)),
                500..=509 => Err(HttpError::APIError(APIError::ServerError(response.into_string().unwrap()))),
                _ => Err(HttpError::Unknown)
            }
        },
        Err(_) => { Err(HttpError::NoInternet) }
    }
}

#[cfg(all(feature = "reqwest", feature = "sync"))]
fn handle_reqwest_response_blocking<T: DeserializeOwned>(response: Result<reqwest::blocking::Response, reqwest::Error>) -> Result<T, HttpError> {
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

#[cfg(all(feature = "reqwest", feature = "async"))]
async fn handle_reqwest_response<T: DeserializeOwned>(response: Result<reqwest::Response, reqwest::Error>) -> Result<T, HttpError> {
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

trait Client {
    type Error: Error + Send + Sync + 'static;
    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, HttpError>;
}

