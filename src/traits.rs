use std::borrow::Cow;

#[cfg(any(feature = "async", doc))]
use async_trait::async_trait;
#[cfg(any(feature = "reqwest", doc))]
use http::StatusCode;
use serde::de::DeserializeOwned;

#[cfg(any(feature = "ureq", doc))]
use crate::ureq_client::handle_ureq_response;

use crate::errors::{APIError, HttpError};

pub trait Endpoint {
  type Output: DeserializeOwned;

  /// Returns the method required for this endpoint `NOTE` Default is GET
  fn method(&self) -> &str {
    "GET"
  }

  /// Returns the endpoint Url BUT it won't have any queries attached to it
  fn endpoint(&self) -> Cow<'static, str>;

  /// Returns the built URL for this endpoint
  fn url(&self) -> Result<url::Url, HttpError> {
    Ok(url::Url::parse(&self.endpoint()).unwrap())
  }

  #[cfg(any(feature = "ureq", doc))]
  /// Uses a `ureq` client to make the API call and handle the response.
  /// The assumption is made that the token is part of the default headers
  /// Requires the `ureq` feature to be enabled
  fn ureq_client(&self, client: &ureq::Agent) -> Result<Self::Output, HttpError> {
    let url_endpoint = self.url()?;
    handle_ureq_response(client.request(self.method(), url_endpoint.as_str()).call())
  }

  #[cfg(any(feature = "ureq", doc))]
  /// Creates a `ureq` client to make the API call and handle the response
  /// Requires the `ureq` feature to be enabled
  fn ureq(&self, token: &str) -> Result<Self::Output, HttpError> {
    use crate::constants::TOKEN_KEY;

    let url_endpoint = self.url()?;
    handle_ureq_response(
      ureq::request(self.method(), url_endpoint.as_str())
        .set(TOKEN_KEY, token)
        .call(),
    )
  }

  #[cfg(any(all(feature = "reqwest", feature = "sync"), doc))]
  /// Uses a `reqwest::blocking` client to make the API call and handle the response.
  /// The assumption is made that the token is part of the default headers
  /// Requires the `reqwest` and `sync` features to be enabled
  fn reqwest_client(&self, client: &reqwest::blocking::Client) -> Result<Self::Output, HttpError> {
    let url_endpoint = self.url()?;
    crate::reqwest_blocking_client::handle_reqwest_response_blocking::<Self::Output>(
      client.get(url_endpoint.as_str()).send(),
    )
  }

  #[cfg(any(all(feature = "reqwest", feature = "sync"), doc))]
  /// Creates a `reqwest::blocking` client to make the API call and handle the response
  /// Requires the `reqwest` and `sync` features to be enabled
  fn reqwest(&self, token: &str) -> Result<Self::Output, HttpError> {
    use http::header;

    use crate::constants::TOKEN_KEY;

    let url_endpoint = self.url()?;
    let mut headers = header::HeaderMap::new();
    headers.insert(TOKEN_KEY, header::HeaderValue::from_str(token).unwrap());
    let client = reqwest::blocking::ClientBuilder::new()
      .default_headers(headers)
      .build()
      .unwrap();
    crate::reqwest_blocking_client::handle_reqwest_response_blocking::<Self::Output>(
      client.get(url_endpoint.as_str()).send(),
    )
  }
}

#[cfg(any(all(feature = "reqwest", feature = "async"), doc))]
#[async_trait]
pub trait EndpointAsync: Endpoint {
  #[cfg(any(all(feature = "reqwest", feature = "async"), doc))]
  /// Uses an async `reqwest` client to make the API call and handle the response.
  /// The assumption is made that the token is part of the default headers
  /// Requires the `reqwest` and `async` features to be enabled
  async fn reqwest_client_async(
    &self,
    client: &reqwest::Client,
  ) -> Result<Self::Output, HttpError> {
    let url_endpoint = self.url()?;
    crate::reqwest_async_client::handle_reqwest_response::<Self::Output>(
      client.get(url_endpoint.as_str()).send().await,
    )
    .await
  }

  #[cfg(any(all(feature = "reqwest", feature = "async"), doc))]
  /// Creates an async `reqwest` client to make the API call and handle the response
  /// Requires the `reqwest` and `async` features to be enabled
  async fn reqwest_async(&self, token: &str) -> Result<Self::Output, HttpError> {
    let url_endpoint = self.url()?;

    let mut headers = http::header::HeaderMap::new();
    headers.insert(
      crate::constants::TOKEN_KEY,
      http::header::HeaderValue::from_str(token).unwrap(),
    );
    let client = reqwest::ClientBuilder::new()
      .default_headers(headers)
      .build()
      .unwrap();
    crate::reqwest_async_client::handle_reqwest_response::<Self::Output>(
      client.get(url_endpoint.as_str()).send().await,
    )
    .await
  }
}
