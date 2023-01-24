use serde::Deserialize;
use serde::Serialize;

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
