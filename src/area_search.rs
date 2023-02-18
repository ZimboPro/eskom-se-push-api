use derive_builder::Builder;
use serde::Deserialize;
use serde::Serialize;

use crate::errors::HttpError;
use crate::traits::Endpoint;
#[cfg(any(feature = "async", doc))]
use crate::traits::EndpointAsync;

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
pub struct AreaSearchURL {
  search_term: String,
}

impl Endpoint for AreaSearchURL {
  type Output = AreaSearch;

  fn endpoint(&self) -> std::borrow::Cow<'static, str> {
    std::borrow::Cow::Borrowed("https://developer.sepush.co.za/business/2.0/areas_search")
  }

  fn url(&self) -> Result<url::Url, crate::errors::HttpError> {
    let mut u = url::Url::parse(&self.endpoint()).unwrap();
    if self.search_term.trim().is_empty() {
      Err(HttpError::SearchTextNotSet)
    } else {
      u.set_query(Some(format!("text={}", self.search_term).as_str()));
      Ok(u)
    }
  }
}

#[cfg(any(feature = "async", doc))]
impl EndpointAsync for AreaSearchURL {}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AreaSearch {
  /// Vec of areas
  pub areas: Vec<Area>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Area {
  /// ID of the area. To be used when querying for the related info.
  pub id: String,
  /// Name of the area.
  pub name: String,
  /// Region the area is in.
  pub region: String,
}
