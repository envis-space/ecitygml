use crate::Error;
use crate::gml::codec::building::building_subdivision_kind::{
    deserialize_building_subdivision_kind, serialize_building_subdivision_kind,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use crate::gml::write::Formatting;
use ecitygml_core::model::building::BuildingSubdivisionProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_building_subdivision_property(
    xml_document: &[u8],
    spans: &XmlElementSpans,
) -> Result<BuildingSubdivisionProperty, Error> {
    let gml_building_subdivision_property: GmlBuildingSubdivisionProperty =
        de::from_reader(xml_document)?;
    let mut building_subdivision_property: BuildingSubdivisionProperty =
        gml_building_subdivision_property.into();

    building_subdivision_property.object =
        deserialize_building_subdivision_kind(xml_document, spans)?;

    Ok(building_subdivision_property)
}

pub fn serialize_building_subdivision_property(
    building_subdivision_property: &BuildingSubdivisionProperty,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();
    if let Some(href) = &building_subdivision_property.href {
        parts
            .attributes
            .push(("xlink:href".to_string(), href.clone()));
    }
    if let Some(object) = &building_subdivision_property.object {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_building_subdivision_kind(
                object, formatting,
            )?));
    }
    Ok(XmlNode::new(XmlElement::BuildingSubdivisionProperty, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlBuildingSubdivisionProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,
}

impl From<GmlBuildingSubdivisionProperty> for BuildingSubdivisionProperty {
    fn from(item: GmlBuildingSubdivisionProperty) -> Self {
        Self {
            object: None,
            href: item.href,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gml::codec::core::deserialize_abstract_thematic_surface;
    use ecitygml_core::model::core::{
        AsAbstractCityObject, AsAbstractFeature, AsAbstractThematicSurface,
    };
    use egml::model::base::Id;
    use quick_xml::{DeError, de};

    #[test]
    fn test_deserialize_basic_href_boundary() {
        use crate::gml::util::extract_xml_element_spans;
        let xml_document = b"<bldg:buildingSubdivision>
            <bldg:Storey>
              <gml:name>Second Floor</gml:name>
            </bldg:Storey>
          </bldg:buildingSubdivision>";

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let building_subdivision_property =
            deserialize_building_subdivision_property(xml_document, &spans).expect("should work");
        let building_subdivision_kind = building_subdivision_property.object.as_ref().unwrap();

        assert_eq!(
            building_subdivision_kind.name().first().unwrap().as_str(),
            "Second Floor"
        );
    }
}
