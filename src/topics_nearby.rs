use serde::Deserialize;
use serde::Serialize;

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
