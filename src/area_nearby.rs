use derive_builder::Builder;
use serde::Deserialize;
use serde::Serialize;

use crate::errors::HttpError;
use crate::traits::Endpoint;
#[cfg(any(feature = "async", doc))]
use crate::traits::EndpointAsync;

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
pub struct AreasNearbyURL {
  latitude: f32,
  longitude: f32,
}

impl Endpoint for AreasNearbyURL {
  type Output = AreaNearby;

  fn endpoint(&self) -> std::borrow::Cow<'static, str> {
    std::borrow::Cow::Borrowed("https://developer.sepush.co.za/business/2.0/areas_nearby")
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
impl EndpointAsync for AreasNearbyURL {}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AreaNearby {
  /// Vec of areas nearby.
  pub areas: Vec<Area>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Area {
  pub count: i64,
  pub id: String,
  pub name: String,
  pub region: String,
}
