// use serde_json::{Map};

// #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Request {
//     pub action: String,
//     pub accounts: Vec<String>,
// }


// #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Result {
//     balances: Map<String, Balance>,
// }


// #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Balance {
//     pub balance: String,
//     pub pending: String,
// }

// impl Request {
//     pub fn as_vec(&self) -> Vec<u8> {
//         let encoded: Vec<u8> = serde_json::to_vec(&self).unwrap();
//         encoded
//     }
// }

// impl Result {
//     pub fn as_vec(&self) -> Vec<u8> {
//         let encoded: Vec<u8> = serde_json::to_vec(&self).unwrap();
//         encoded
//     }
// }