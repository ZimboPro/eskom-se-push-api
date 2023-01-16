use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EskomStatus {
    pub status: HashMap<String, LoadsheddingStatus>,
}

impl EskomStatus {
    pub fn eskom(&self) -> &LoadsheddingStatus {
        self.status.get("eskom").unwrap()
    }
    
    pub fn area(&self, area: &str) -> Option<&LoadsheddingStatus> {
        self.status.get(area.to_lowercase().as_str())
    }
    
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

