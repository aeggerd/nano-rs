#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountHistoryRequest {
    pub action: String,
    pub account: String,
    pub count: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountHistoryResult {
    pub account: String,
    pub history: Vec<History>,
    pub previous: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct History {
    #[serde(rename = "type")]
    pub type_field: String,
    pub account: String,
    pub amount: String,
    #[serde(rename = "local_timestamp")]
    pub local_timestamp: String,
    pub height: String,
    pub hash: String,
}

impl AccountHistoryRequest {
    pub fn as_vec(&self) -> Vec<u8> {
        let encoded: Vec<u8> = serde_json::to_vec(&self).unwrap();
        encoded
    }
}

impl AccountHistoryResult {
    pub fn as_vec(&self) -> Vec<u8> {
        let encoded: Vec<u8> = serde_json::to_vec(&self).unwrap();
        encoded
    }
}