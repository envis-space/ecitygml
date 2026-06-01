use serde::{Deserialize, Deserializer};

pub fn deserialize_space_separated_f64<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect())
}
