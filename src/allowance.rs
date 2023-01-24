use serde::Deserialize;
use serde::Serialize;

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
