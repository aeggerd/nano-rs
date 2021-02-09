#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub action: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub multiplier: String,
    #[serde(rename = "network_current")]
    pub network_current: String,
    #[serde(rename = "network_minimum")]
    pub network_minimum: String,
    #[serde(rename = "network_receive_current")]
    pub network_receive_current: String,
    #[serde(rename = "network_receive_minimum")]
    pub network_receive_minimum: String,
}

impl Request {
    pub fn as_vec(&self) -> Vec<u8> {
        let encoded: Vec<u8> = serde_json::to_vec(&self).unwrap();
        encoded
    }
}

impl Result {
    pub fn as_vec(&self) -> Vec<u8> {
        let encoded: Vec<u8> = serde_json::to_vec(&self).unwrap();
        encoded
    }
}