use derive_builder::Builder;
use serde::Deserialize;
use serde::Serialize;

use crate::errors::HttpError;
use crate::traits::Endpoint;
#[cfg(any(feature= "async", doc))]
use crate::traits::EndpointAsync;

/// The URL builder for the Allowance Check endpoint
/// ```rust
/// let t = AllowanceCheckURL::default()
/// // returns the url for built the endpoint
/// t.url().unwrap()
/// ```
#[derive(Default, Builder, Debug)]
#[builder()]
pub struct AreaInfoURL {
  area_id: String,
}

impl Endpoint for AreaInfoURL {
  type Output = AreaInfo;

  fn endpoint(&self) -> std::borrow::Cow<'static, str> {
    std::borrow::Cow::Borrowed("https://developer.sepush.co.za/business/2.0/area")
  }

  fn url(&self) -> Result<url::Url, crate::errors::HttpError> {
    let mut u = url::Url::parse(&self.endpoint()).unwrap();
    if self.area_id.trim().is_empty() {
      Err(HttpError::AreaIdNotSet)
    } else {
      u.set_query(Some(format!("id={}", self.area_id).as_str()));
      Ok(u)
    }
  }
}
#[cfg(any(feature = "async", doc))]
impl EndpointAsync for AreaInfoURL {}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AreaInfo {
  /// List of sorted events. Will be an empty list if not impacted
  pub events: Vec<Event>,
  /// Info of the region requested for
  pub info: Info,
  /// Raw loadshedding schedule, per stage (1-8)
  /// `Note`: An empty list means no events for that stage
  /// `Note`: Some Municipalities/Regions don't have Stage 5-8 schedules (and there will be 4 records instead of 8 in this list. Stage 5 upwards you can assume Stage 4 schedule impact.
  pub schedule: Schedule,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
  // TODO convert to Date
  /// End time of the event eg `2022-08-08T22:30:00+02:00`
  pub end: String,
  // TODO convert to enum
  /// The stage of the event eg `Stage 2`
  pub note: String,
  // TODO convert to Date
  /// Start time of the event eg `2022-08-08T20:00:00+02:00`
  pub start: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
  pub name: String,
  pub region: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
  /// Vec of the days and there stages
  pub days: Vec<Day>,
  /// Where the data was retrieved from.
  pub source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Day {
  // TODO convert to Date
  /// Date the stages are relevant to eg `2022-08-08`
  pub date: String,
  // TODO convert to enum
  /// Day of week eg `Monday`
  pub name: String,
  /// Raw loadshedding schedule, per stage (1-8).
  /// Index 0 refers to `Stage 1`, index 1 is `Stage 2` and so on and so forth.
  /// Formatted for display purposes `(i.e. 20:00-22:30)`.
  /// Any adjacent events have been merged into a single event `(e.g. 12:00-14:30 & 14:00-16:30 become 12:00-16:30)`.
  ///  * `Note`: An empty list means no events for that stage
  ///  * `Note`: Some Municipalities/Regions don't have Stage 5-8 schedules (and there will be 4 records instead of 8 in this list. Stage 5 upwards you can assume Stage 4 schedule impact.
  pub stages: Vec<Vec<String>>,
}
