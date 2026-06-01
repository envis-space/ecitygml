use crate::Error;
use crate::gml::codec::transportation::abstract_transportation_space::deserialize_abstract_transportation_space;
use crate::gml::codec::transportation::intersection_property::deserialize_intersection_property;
use crate::gml::codec::transportation::section_property::deserialize_section_property;
use crate::gml::util::{XmlElement, collect_children, extract_xml_element_spans};
use ecitygml_core::model::transportation::Road;

pub fn deserialize_road(xml_document: &[u8]) -> Result<Road, Error> {
    let spans = extract_xml_element_spans(xml_document)?;

    let mut abstract_transportation_space_result = None;
    let mut sections_result = None;
    let mut intersections_result = None;

    rayon::scope(|s| {
        s.spawn(|_| {
            abstract_transportation_space_result = Some(deserialize_abstract_transportation_space(
                xml_document,
                &spans,
            ))
        });
        s.spawn(|_| {
            sections_result = Some(collect_children(
                xml_document,
                &spans,
                XmlElement::SectionProperty,
                deserialize_section_property,
            ));
        });
        s.spawn(|_| {
            intersections_result = Some(collect_children(
                xml_document,
                &spans,
                XmlElement::IntersectionProperty,
                deserialize_intersection_property,
            ));
        });
    });

    let abstract_transportation_space = abstract_transportation_space_result
        .expect("rayon::scope guarantees all spawns complete")?;
    let sections = sections_result.expect("rayon::scope guarantees all spawns complete")?;
    let intersections =
        intersections_result.expect("rayon::scope guarantees all spawns complete")?;

    let mut road = Road::new(abstract_transportation_space);
    road.sections = sections;
    road.intersections = intersections;
    Ok(road)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::{AsAbstractCityObject, AsAbstractFeature, AsAbstractSpace};
    use egml::model::base::Id;

    #[test]
    fn test_deserialize_basic_road() {
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

        let road = deserialize_road(xml_document).expect("should work");

        assert_eq!(
            road.id(),
            &Id::try_from("UUID_8a13804c-cbd7-3a2f-9d46-e4d528a4bb4f").expect("should work")
        );

        assert!(road.lod2_multi_surface().is_none());
        assert_eq!(road.generic_attributes().len(), 1);
        assert!(road.intersections.is_empty());
        assert_eq!(road.sections.len(), 1);
        let traffic_space = road.sections.first().unwrap().object.as_ref().unwrap();
        assert_eq!(
            traffic_space.id(),
            &Id::try_from("UUID_0950bfa5-204e-33e6-bdb7-c5c318d73a29").expect("should work")
        );
    }
}
