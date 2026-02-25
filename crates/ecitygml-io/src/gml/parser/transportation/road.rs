use crate::Error;
use crate::gml::parser::city_object_reader::read_city_objects;
use crate::gml::parser::core::parse_abstract_space;
use ecitygml_core::model::common::CityObjectClass;
use ecitygml_core::model::core::CityObjectKind;
use ecitygml_core::model::transportation::Road;
use std::collections::HashSet;

pub fn parse_road(xml_document: &[u8]) -> Result<Road, Error> {
    let space = parse_abstract_space(xml_document)?;
    let mut road = Road::new(space);

    let parsed_city_objects = read_city_objects(
        xml_document,
        HashSet::from([CityObjectClass::Intersection, CityObjectClass::Section]),
    )?;
    for city_object in parsed_city_objects {
        match city_object {
            CityObjectKind::Intersection(x) => {
                road.intersection.push(x);
            }
            CityObjectKind::Section(x) => {
                road.section.push(x);
            }
            _ => {
                panic!("Unexpected city object kind: {:?}", city_object);
            }
        }
    }

    Ok(road)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::{AsAbstractCityObject, AsAbstractFeature, AsAbstractSpace};
    use egml::model::base::Id;

    #[test]
    fn test_parse_basic_road() {
        let xml_document = b"
    <tran:Road gml:id=\"UUID_8a13804c-cbd7-3a2f-9d46-e4d528a4bb4f\">
      <genericAttribute>
        <gen:StringAttribute>
          <gen:name>identifier_new_section</gen:name>
          <gen:value>abc</gen:value>
        </gen:StringAttribute>
      </genericAttribute>
      <tran:section>
        <tran:Section gml:id=\"UUID_0950bfa5-204e-33e6-bdb7-c5c318d73a29\">
        </tran:Section>
      </tran:section>
    </tran:Road>";

        let road = parse_road(xml_document).expect("should work");

        assert_eq!(
            road.id(),
            &Id::try_from("UUID_8a13804c-cbd7-3a2f-9d46-e4d528a4bb4f").expect("should work")
        );

        assert!(road.lod2_multi_surface().is_none());
        assert_eq!(road.generic_attributes().len(), 1);
        assert!(road.intersection.is_empty());
        assert_eq!(road.section.len(), 1);
        let traffic_space = road.section.first().unwrap();
        assert_eq!(
            traffic_space.id(),
            &Id::try_from("UUID_0950bfa5-204e-33e6-bdb7-c5c318d73a29").expect("should work")
        );
    }
}
