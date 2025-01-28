use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub fn default_true() -> bool {
    true
}

pub fn default_scale() -> String {
    "1".to_string()
}

pub fn int_or_string_as_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::String(s) => Ok(s),
        Value::Number(n) => Ok(n.to_string()),
        _ => Err(serde::de::Error::custom("Expected a string or an integer")),
    }
}
