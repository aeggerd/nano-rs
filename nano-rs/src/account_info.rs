#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfoRequest {
    pub action: String,
    pub account: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfoResult {
    pub frontier: String,
    #[serde(rename = "open_block")]
    pub open_block: String,
    #[serde(rename = "representative_block")]
    pub representative_block: String,
    pub balance: String,
    #[serde(rename = "modified_timestamp")]
    pub modified_timestamp: String,
    #[serde(rename = "block_count")]
    pub block_count: String,
    #[serde(rename = "confirmation_height")]
    pub confirmation_height: String,
    #[serde(rename = "confirmation_height_frontier")]
    pub confirmation_height_frontier: String,
    #[serde(rename = "account_version")]
    pub account_version: String,
}

impl AccountInfoRequest {
    pub fn as_vec(&self) -> Vec<u8> {
        let encoded: Vec<u8> = serde_json::to_vec(&self).unwrap();
        encoded
    }
}

impl AccountInfoResult {
    pub fn as_vec(&self) -> Vec<u8> {
        let encoded: Vec<u8> = serde_json::to_vec(&self).unwrap();
        encoded
    }
}
