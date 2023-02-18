use derive_builder::Builder;
use serde::Deserialize;
use serde::Serialize;

use crate::traits::Endpoint;
#[cfg(any(feature = "async", doc))]
use crate::traits::EndpointAsync;

/// The URL builder for the Allowance Check endpoint
/// ```rust
/// let t = AllowanceCheckURL::default()
/// // returns the url for built the endpoint
/// t.url().unwrap()
/// ```
#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
pub struct AllowanceCheckURL {}

impl Endpoint for AllowanceCheckURL {
  type Output = AllowanceCheck;

  fn endpoint(&self) -> std::borrow::Cow<'static, str> {
    std::borrow::Cow::Borrowed("https://developer.sepush.co.za/business/2.0/api_allowance")
  }

  fn url(&self) -> Result<url::Url, crate::errors::HttpError> {
    Ok(url::Url::parse(&self.endpoint()).unwrap())
  }
}
#[cfg(any(feature = "async", doc))]
impl EndpointAsync for AllowanceCheckURL {}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowanceCheck {
  /// What is the status of your allowance.
  pub allowance: Allowance,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Allowance {
  /// How many API calls have already been made excluding the allowance API.
  pub count: i64,
  /// The max amount of allowed API calls.
  pub limit: i64,
  /// The account type.
  #[serde(rename = "type")]
  pub type_field: String,
}
