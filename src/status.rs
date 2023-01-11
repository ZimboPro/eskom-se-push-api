use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EskomStatus {
    pub status: Status,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub capetown: Capetown,
    pub eskom: Eskom,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capetown {
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

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Eskom {
    pub name: String,
    #[serde(rename = "next_stages")]
    pub next_stages: Vec<NextStage2>,
    pub stage: String,
    #[serde(rename = "stage_updated")]
    pub stage_updated: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextStage2 {
    pub stage: String,
    #[serde(rename = "stage_start_timestamp")]
    pub stage_start_timestamp: String,
}
