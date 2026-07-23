use crate::error::Error;

use crate::gml::codec::core::deserialize_city_model;
use crate::gml::util::{CitygmlVersion, detect_citygml_version};
use ecitygml_core::model::core::CityModel;

extern crate quick_xml;
extern crate serde;

pub fn deserialize(file_content: Vec<u8>) -> Result<CityModel, Error> {
    match detect_citygml_version(&file_content)? {
        Some(CitygmlVersion::V3_0) => {}
        Some(version) => return Err(Error::UnsupportedCityGmlVersion(version)),
        None => return Err(Error::UnknownCityGmlVersion),
    }

    let city_model = deserialize_city_model(&file_content)?;

    Ok(city_model)
}
