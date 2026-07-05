use serde::{Deserialize, Deserializer, Serializer};

pub fn deserialize_space_separated_f64<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect())
}

pub fn deserialize_space_separated_f64_3<'de, D>(deserializer: D) -> Result<[f64; 3], D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let s = String::deserialize(deserializer)?;
    let values: Vec<f64> = s
        .split_whitespace()
        .filter_map(|v| v.parse().ok())
        .collect();
    values
        .try_into()
        .map_err(|_| D::Error::custom("expected exactly 3 f64 values"))
}

pub fn serialize_space_separated_f64_3<S>(
    values: &[f64; 3],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&values.map(|v| v.to_string()).join(" "))
}

pub fn deserialize_space_separated_f64_4<'de, D>(deserializer: D) -> Result<[f64; 4], D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let s = String::deserialize(deserializer)?;
    let values: Vec<f64> = s
        .split_whitespace()
        .filter_map(|v| v.parse().ok())
        .collect();
    values
        .try_into()
        .map_err(|_| D::Error::custom("expected exactly 4 f64 values"))
}

pub fn serialize_space_separated_f64_4<S>(
    values: &[f64; 4],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&values.map(|v| v.to_string()).join(" "))
}
