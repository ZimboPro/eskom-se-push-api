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
  #[cfg(any(feature= "reqwest", doc))]
  ResponseError(#[from] reqwest::Error),
  #[cfg(any(feature = "ureq", doc))]
  #[error("Response Error: {0}")]
  UreqResponseError(String),
  #[error("Search text not set")]
  SearchTextNotSet,
  #[error("Area ID not set")]
  AreaIdNotSet,
  #[error(
    "Longitude and/or latitude has not been set: Long: {longitude:?} latitude: {latitude:?}"
  )]
  LongitudeOrLatitudeNotSet { longitude: f32, latitude: f32 },
  #[error("Unknown error: {0}")]
  UnknownError(String),
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
  #[error("Server Error: {0}")]
  ServerError(String),
}
