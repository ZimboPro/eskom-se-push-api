use derive_builder::Builder;
use serde::Deserialize;
use serde::Serialize;

use crate::errors::HttpError;
use crate::traits::Endpoint;
#[cfg(any(feature = "async", doc))]
use crate::traits::EndpointAsync;

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
pub struct TopicsNearbyUrl {
  latitude: f32,
  longitude: f32,
}

impl Endpoint for TopicsNearbyUrl {
  type Output = TopicsNearby;

  fn endpoint(&self) -> std::borrow::Cow<'static, str> {
    std::borrow::Cow::Borrowed("https://developer.sepush.co.za/business/2.0/topics_nearby")
  }

  fn url(&self) -> Result<url::Url, crate::errors::HttpError> {
    let mut u = url::Url::parse(&self.endpoint()).unwrap();
    if self.latitude == 0. || self.longitude == 0. {
      Err(HttpError::LongitudeOrLatitudeNotSet {
        longitude: self.longitude,
        latitude: self.latitude,
      })
    } else {
      u.set_query(Some(format!("lat={}", self.latitude).as_str()));
      u.set_query(Some(format!("long={}", self.longitude).as_str()));
      Ok(u)
    }
  }
}

#[cfg(any(feature = "async", doc))]
impl EndpointAsync for TopicsNearbyUrl {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicsNearby {
  /// Vec of topics in the area
  pub topics: Vec<Topic>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Topic {
  pub active: String,
  pub body: String,
  pub category: String,
  pub distance: f64,
  pub followers: i64,
  pub timestamp: String,
}
