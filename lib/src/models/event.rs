use crate::utils::parser::string_to_float;

/**
 * This is the Event struct which will be formatted from
 * the file.
 */

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Event {
    #[serde(rename(deserialize = "a"), deserialize_with = "string_to_float")]
    pub asking_price: f32,

    #[serde(rename(deserialize = "b"), deserialize_with = "string_to_float")]
    pub bidding_price: f32,

    #[serde(rename(deserialize = "T"))]
    pub timestamp: i64,

    #[serde(rename(deserialize = "s"))]
    pub symbol: String,
}
