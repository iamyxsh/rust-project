use crate::utils::parser::{float_to_string, string_to_float};
use crate::utils::MinNonNan;
use serde::{Deserialize, Serialize};

/**
 * This is the OHLC struct.
 * This is the struct that will be serialized
 * to a file.
 */

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OHLC {
    #[serde(
        serialize_with = "float_to_string",
        deserialize_with = "string_to_float"
    )]
    pub open: f32,
    #[serde(
        serialize_with = "float_to_string",
        deserialize_with = "string_to_float"
    )]
    pub high: f32,
    #[serde(
        serialize_with = "float_to_string",
        deserialize_with = "string_to_float"
    )]
    pub low: f32,
    #[serde(
        serialize_with = "float_to_string",
        deserialize_with = "string_to_float"
    )]
    pub close: f32,
    #[serde(skip_serializing, skip_deserializing)]
    pub price: MinNonNan,
    pub timestamp: i64,
    pub symbol: String,
}

impl OHLC {
    // Init
    pub fn new(
        open: f32,
        close: f32,
        high: f32,
        low: f32,
        timestamp: i64,
        symbol: String,
        price: f32,
    ) -> Self {
        OHLC {
            open,
            high,
            low,
            close,
            timestamp,
            symbol,
            price: MinNonNan(price),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ohlc = OHLC::new(
            0_f32,
            0_f32,
            0_f32,
            0_f32,
            0_i64,
            "SYMBOL".to_string(),
            0_f32,
        );

        assert_eq!(ohlc.open, 0_f32);
        assert_eq!(ohlc.close, 0_f32);
        assert_eq!(ohlc.high, 0_f32);
        assert_eq!(ohlc.low, 0_f32);
        assert_eq!(ohlc.symbol, "SYMBOL".to_string());
        assert_eq!(ohlc.timestamp, 0_i64);
        assert_eq!(ohlc.price, MinNonNan(0_f32));
    }
}
