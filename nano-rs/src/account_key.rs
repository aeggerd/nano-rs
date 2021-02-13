#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountKeyRequest {
    pub action: String,
    pub account: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountKeyResult {
    pub key: String,
}

impl AccountKeyRequest {
    pub fn as_vec(&self) -> Vec<u8> {
        let encoded: Vec<u8> = serde_json::to_vec(&self).unwrap();
        encoded
    }
}

impl AccountKeyResult {
    pub fn as_vec(&self) -> Vec<u8> {
        let encoded: Vec<u8> = serde_json::to_vec(&self).unwrap();
        encoded
    }
}
