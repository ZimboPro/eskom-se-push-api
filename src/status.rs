use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

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

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextStage {
  pub stage: String,
  #[serde(rename = "stage_start_timestamp")]
  pub stage_start_timestamp: String,
}
