use crate::error::Error;

use crate::gml::parser::city_object_reader::read_city_objects;
use ecitygml_core::model::common::CityObjectClass;
use ecitygml_core::model::core::{AbstractFeature, CityModel};
use egml::model::base::Id;
use std::collections::HashSet;

extern crate quick_xml;
extern crate serde;

pub fn decode(file_content: Vec<u8>) -> Result<CityModel, Error> {
    let city_objects = read_city_objects(
        file_content.as_slice(),
        HashSet::from([
            CityObjectClass::Building,
            CityObjectClass::SolitaryVegetationObject,
            CityObjectClass::CityFurniture,
            CityObjectClass::ReliefFeature,
            CityObjectClass::Road,
        ]),
    )?;

    // TODO
    let abstract_feature = AbstractFeature::new(Id::generate_uuid_v4());

    let city_model = CityModel::new(abstract_feature, city_objects);
    Ok(city_model)
}
