use crate::Error;
use crate::gml::codec::transportation::abstract_transportation_space::deserialize_abstract_transportation_space;
use crate::gml::util::extract_xml_element_spans;
use ecitygml_core::model::transportation::Intersection;

pub fn deserialize_intersection(xml_document: &[u8]) -> Result<Intersection, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_transportation_space =
        deserialize_abstract_transportation_space(xml_document, &spans)?;
    let intersection = Intersection::new(abstract_transportation_space);

    Ok(intersection)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::{
        AsAbstractCityObject, AsAbstractFeature, AsAbstractSpace, SpaceBoundaryKind,
        ThematicSurfaceKind,
    };
    use ecitygml_core::model::transportation::{
        AsAbstractTransportationSpace, AuxiliaryTrafficArea, GranularityValue, TrafficArea,
    };
    use egml::model::base::Id;

    #[test]
    fn test_deserialize_basic_intersection() {
        let xml_document = b"
        <tran:Intersection gml:id=\"UUID_9a9fc5a0-b252-3d63-ac79-b3141175f152\">
          <genericAttribute>
            <gen:StringAttribute>
              <gen:name>identifier_new_section</gen:name>
              <gen:value>abc</gen:value>
            </gen:StringAttribute>
          </genericAttribute>
          <tran:trafficSpace>
            <tran:TrafficSpace gml:id=\"UUID_ff91145b-98e8-388b-b4d1-b94624f806db\">
              <genericAttribute>
                <gen:StringAttribute>
                  <gen:name>identifier_roadId</gen:name>
                  <gen:value>4516050</gen:value>
                </gen:StringAttribute>
              </genericAttribute>
              <tran:function>2</tran:function>
              <tran:usage>1</tran:usage>
              <tran:granularity>lane</tran:granularity>
              <tran:trafficDirection>forwards</tran:trafficDirection>
              <tran:predecessor xlink:href=\"#UUID_35e15191-e911-320b-bbe7-004b237a024a\"/>
              <tran:successor xlink:href=\"#UUID_efb6f11e-0d92-3d1f-8208-7ddbea02e829\"/>
            </tran:TrafficSpace>
          </tran:trafficSpace>
        </tran:Intersection>";

        let intersection = deserialize_intersection(xml_document).expect("should work");

        assert_eq!(
            intersection.id(),
            &Id::try_from("UUID_9a9fc5a0-b252-3d63-ac79-b3141175f152").expect("should work")
        );

        assert!(intersection.lod2_multi_surface().is_none());
        assert_eq!(intersection.generic_attributes().len(), 1);
        assert!(intersection.auxiliary_traffic_spaces().is_empty());
        assert_eq!(intersection.traffic_spaces().len(), 1);
        let traffic_space = intersection
            .traffic_spaces()
            .first()
            .unwrap()
            .object
            .as_ref()
            .unwrap();

        assert_eq!(
            traffic_space.id(),
            &Id::try_from("UUID_ff91145b-98e8-388b-b4d1-b94624f806db").expect("should work")
        );
        assert_eq!(traffic_space.granularity(), &GranularityValue::Lane);

        let traffic_areas: Vec<&TrafficArea> = traffic_space
            .boundaries()
            .iter()
            .flat_map(|x| &x.object)
            .filter_map(|x| match x {
                SpaceBoundaryKind::ThematicSurfaceKind(ThematicSurfaceKind::TrafficArea(x)) => {
                    Some(x)
                }
                _ => None,
            })
            .collect();
        assert!(traffic_areas.is_empty());
    }
}
