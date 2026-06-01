use crate::Error;
use crate::gml::codec::core::deserialize_abstract_space_boundary;
use crate::gml::codec::relief::relief_component_property::deserialize_relief_component_property;
use crate::gml::util::{XmlElement, collect_children, extract_xml_element_spans};
use ecitygml_core::model::relief::ReliefFeature;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_relief_feature(xml_document: &[u8]) -> Result<ReliefFeature, Error> {
    let spans = extract_xml_element_spans(xml_document)?;

    let mut abstract_space_boundary_result = None;
    let mut parsed_result = None;
    let mut relief_components_result = None;

    rayon::scope(|s| {
        s.spawn(|_| {
            abstract_space_boundary_result =
                Some(deserialize_abstract_space_boundary(xml_document, &spans))
        });
        s.spawn(|_| {
            parsed_result =
                Some(de::from_reader::<_, GmlReliefFeature>(xml_document).map_err(Error::from));
        });
        s.spawn(|_| {
            relief_components_result = Some(collect_children(
                xml_document,
                &spans,
                XmlElement::ReliefComponentProperty,
                deserialize_relief_component_property,
            ));
        });
    });

    let abstract_space_boundary =
        abstract_space_boundary_result.expect("rayon::scope guarantees all spawns complete")?;
    let parsed: GmlReliefFeature =
        parsed_result.expect("rayon::scope guarantees all spawns complete")?;
    let relief_components =
        relief_components_result.expect("rayon::scope guarantees all spawns complete")?;

    let mut relief_feature = ReliefFeature::new(abstract_space_boundary, parsed.lod.try_into()?);
    relief_feature.set_relief_components(relief_components);
    Ok(relief_feature)
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GmlReliefFeature {
    pub lod: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::common::LevelOfDetail;
    use ecitygml_core::model::core::AsAbstractFeature;
    use egml::model::base::Id;

    #[test]
    fn test_deserialize_tin_relief() {
        let xml_document = "
            <dem:ReliefFeature gml:id=\"ID_7a8b707e-f87c-35f3-8e3c-254427e59493\">
              <dem:lod>2</dem:lod>
              <dem:reliefComponent>
                <dem:TINRelief gml:id=\"abc\">
                  <dem:lod>3</dem:lod>
                    <dem:tin>
                    <gml:TriangulatedSurface gml:id=\"ground\">
                      <gml:patches>
                        <gml:Triangle>
                          <gml:exterior>
                            <gml:LinearRing gml:id=\"Geo28109128\">
                              <gml:posList>513037.352708 5403284.890495 247.27 513034.424098 5403297.424959 247.19 513034.423981 5403287.429362 247.3 513037.352708 5403284.890495 247.27</gml:posList>
                            </gml:LinearRing>
                          </gml:exterior>
                        </gml:Triangle>
                      </gml:patches>
                    </gml:TriangulatedSurface>
                    </dem:tin>
                </dem:TINRelief>
              </dem:reliefComponent>
            </dem:ReliefFeature>";

        let relief_feature =
            deserialize_relief_feature(xml_document.as_bytes()).expect("should work");

        assert_eq!(
            relief_feature.id(),
            &Id::try_from("ID_7a8b707e-f87c-35f3-8e3c-254427e59493").unwrap()
        );
        assert_eq!(relief_feature.lod(), LevelOfDetail::Two);
        assert_eq!(relief_feature.num_relief_components(), 1);

        let relief_component_property = relief_feature
            .relief_components()
            .first()
            .to_owned()
            .unwrap();
        assert!(relief_component_property.object.is_some());
        let relief_component = relief_component_property.object.as_ref().unwrap();
        assert_eq!(relief_component.id(), &Id::try_from("abc").unwrap());
    }
}
