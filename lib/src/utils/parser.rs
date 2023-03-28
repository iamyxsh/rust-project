use serde::{de, Deserialize, Deserializer, Serializer};
use serde_json::Value;

use super::MinNonNan;

/**
 * Parsers used to serialize and deserialize data
 * accordingly.
 */

pub fn string_to_float<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f32, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().map_err(de::Error::custom)?,

        _ => return Err(de::Error::custom("wrong type")),
    })
}

pub fn string_to_non_min<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<MinNonNan, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => MinNonNan(
            format!("{:.6}", s.parse::<f32>().unwrap())
                .parse::<f32>()
                .map_err(de::Error::custom)?,
        ),

        _ => return Err(de::Error::custom("wrong type")),
    })
}

pub fn non_min_to_string<S>(x: &MinNonNan, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&format!("{:.6}", x.0))
}

pub fn float_to_string<S>(x: &f32, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&format!("{:.6}", x))
}
