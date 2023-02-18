use std::collections::HashMap;

use chrono::DateTime;
use chrono::Utc;
use derive_builder::Builder;
use serde::Deserialize;
use serde::Serialize;

use crate::traits::Endpoint;
#[cfg(any(feature = "async", doc))]
use crate::traits::EndpointAsync;

pub enum Stage {
  NoLoadShedding,
  Stage1,
  Stage2,
  Stage3,
  Stage4,
  Stage5,
  Stage6,
  Stage7,
  Stage8,
  /// Able to check any stage (Also future proofing). String should be a whole number.
  /// "0" is no loadshedding. "1" is Stage 1 etc.
  Stage(String),
}

impl PartialEq<String> for Stage {
  fn eq(&self, other: &String) -> bool {
    match self {
      Stage::NoLoadShedding => "0" == other,
      Stage::Stage1 => "1" == other,
      Stage::Stage2 => "2" == other,
      Stage::Stage3 => "3" == other,
      Stage::Stage4 => "4" == other,
      Stage::Stage5 => "5" == other,
      Stage::Stage6 => "6" == other,
      Stage::Stage7 => "7" == other,
      Stage::Stage8 => "8" == other,
      Stage::Stage(stage) => stage == other,
    }
  }
}

impl From<String> for Stage {
  fn from(stage: String) -> Self {
    match stage.as_str() {
      "0" => Self::NoLoadShedding,
      "1" => Self::Stage1,
      "2" => Self::Stage2,
      "3" => Self::Stage3,
      "4" => Self::Stage4,
      "5" => Self::Stage5,
      "6" => Self::Stage6,
      "7" => Self::Stage7,
      "8" => Self::Stage8,
      new_stage => Self::Stage(new_stage.to_string()),
    }
  }
}

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
pub struct EskomStatusUrl {}

impl Endpoint for EskomStatusUrl {
  type Output = EskomStatus;

  fn endpoint(&self) -> std::borrow::Cow<'static, str> {
    std::borrow::Cow::Borrowed("https://developer.sepush.co.za/business/2.0/status")
  }

  fn url(&self) -> Result<url::Url, crate::errors::HttpError> {
    Ok(url::Url::parse(&self.endpoint()).unwrap())
  }
}

#[cfg(any(feature = "async", doc))]
impl EndpointAsync for EskomStatusUrl {}

/// The status of load shedding nation wide and certain areas if they don't follow the
/// nation wide status
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EskomStatus {
  pub status: HashMap<String, LoadsheddingStatus>,
}

impl EskomStatus {
  /// Gets the nation-wide load shedding status
  pub fn eskom(&self) -> &LoadsheddingStatus {
    self.status.get("eskom").unwrap()
  }

  /// Gets the status for a specific area
  /// `Note` the area needs to match the case of key
  pub fn area(&self, area: &str) -> Option<&LoadsheddingStatus> {
    self.status.get(area.to_lowercase().as_str())
  }

  /// Returns all the area keys
  pub fn keys(&mut self) -> Vec<String> {
    self.status.clone().into_keys().collect()
  }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadsheddingStatus {
  pub name: String,
  #[serde(rename = "next_stages")]
  pub next_stages: Vec<NextStage>,
  pub stage: String,
  #[serde(rename = "stage_updated")]
  pub stage_updated: String,
}

impl LoadsheddingStatus {
  pub fn is_it_stage(&self, stage: Stage) -> bool {
    stage == self.stage
  }

  pub fn get_stage(&self) -> Stage {
    self.stage.clone().into()
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextStage {
  pub stage: String,
  #[serde(rename = "stage_start_timestamp")]
  pub stage_start_timestamp: DateTime<Utc>,
}

impl NextStage {
  pub fn is_it_stage(&self, stage: Stage) -> bool {
    stage == self.stage
  }

  pub fn get_stage(&self) -> Stage {
    self.stage.clone().into()
  }
}
