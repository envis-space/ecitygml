use crate::error::Error;

use crate::gml::codec::core::deserialize_city_model;
use ecitygml_core::model::core::CityModel;

extern crate quick_xml;
extern crate serde;

pub fn deserialize(file_content: Vec<u8>) -> Result<CityModel, Error> {
    let city_model = deserialize_city_model(&file_content)?;

    Ok(city_model)
}
