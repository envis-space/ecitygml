use crate::Error;
use crate::gml::codec::core::{
    deserialize_abstract_thematic_surface, serialize_abstract_thematic_surface,
};
use crate::gml::util::xml_element::XmlElement;
use crate::gml::util::{XmlNode, XmlNodeContent, extract_xml_element_spans, serialize_inner};
use crate::gml::write::Formatting;
use ecitygml_core::model::core::AsAbstractThematicSurface;
use ecitygml_core::model::transportation::AuxiliaryTrafficArea;
use egml::io::GmlCode;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_auxiliary_traffic_area(
    xml_document: &[u8],
) -> Result<AuxiliaryTrafficArea, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let (abstract_thematic_surface_result, parsed_result) = rayon::join(
        || deserialize_abstract_thematic_surface(xml_document, &spans),
        || de::from_reader::<_, GmlAuxiliaryTrafficArea>(xml_document).map_err(Error::from),
    );
    let abstract_thematic_surface = abstract_thematic_surface_result?;
    let parsed = parsed_result?;
    let mut auxiliary_traffic_area =
        AuxiliaryTrafficArea::from_abstract_thematic_surface(abstract_thematic_surface);

    auxiliary_traffic_area.set_class(parsed.class.map(Into::into));
    auxiliary_traffic_area.set_functions(parsed.functions.into_iter().map(Into::into).collect());
    auxiliary_traffic_area.set_usages(parsed.usages.into_iter().map(Into::into).collect());
    auxiliary_traffic_area.set_surface_material(parsed.surface_material.map(Into::into));

    Ok(auxiliary_traffic_area)
}

pub fn serialize_auxiliary_traffic_area(
    auxiliary_traffic_area: &AuxiliaryTrafficArea,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = serialize_abstract_thematic_surface(
        auxiliary_traffic_area.abstract_thematic_surface(),
        formatting,
    )?;

    if let Some(raw) = serialize_inner(
        GmlAuxiliaryTrafficArea::from(auxiliary_traffic_area),
        formatting,
    )? {
        parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(XmlNode::new(XmlElement::AuxiliaryTrafficArea, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAuxiliaryTrafficArea {
    #[serde(
        rename(serialize = "tran:class", deserialize = "class"),
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<GmlCode>,

    #[serde(rename(serialize = "tran:function", deserialize = "function"), default)]
    pub functions: Vec<GmlCode>,

    #[serde(rename(serialize = "tran:usage", deserialize = "usage"), default)]
    pub usages: Vec<GmlCode>,

    #[serde(
        rename(serialize = "tran:surfaceMaterial", deserialize = "surfaceMaterial"),
        skip_serializing_if = "Option::is_none"
    )]
    pub surface_material: Option<GmlCode>,
}

impl From<&AuxiliaryTrafficArea> for GmlAuxiliaryTrafficArea {
    fn from(item: &AuxiliaryTrafficArea) -> Self {
        Self {
            class: item.class().map(Into::into),
            functions: item.functions().iter().map(Into::into).collect(),
            usages: item.usages().iter().map(Into::into).collect(),
            surface_material: item.surface_material().map(Into::into),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::{
        AsAbstractCityObject, AsAbstractFeature, AsAbstractOccupiedSpace, AsAbstractThematicSurface,
    };
    use egml::model::base::Id;

    #[test]
    fn test_deserialize_basic_traffic_area() {
        let xml_document =
            b"<tran:AuxiliaryTrafficArea gml:id=\"UUID_312e01da-5c89-3f89-b8c0-3e0a8bafecbb\">
                  <genericAttribute>
                    <gen:StringAttribute>
                      <gen:name>opendrive_lane_type</gen:name>
                      <gen:value>BORDER</gen:value>
                    </gen:StringAttribute>
                  </genericAttribute>
                </tran:AuxiliaryTrafficArea>";

        let auxiliary_traffic_area =
            deserialize_auxiliary_traffic_area(xml_document).expect("should work");

        assert_eq!(
            auxiliary_traffic_area.id(),
            &Id::try_from("UUID_312e01da-5c89-3f89-b8c0-3e0a8bafecbb").expect("should work")
        );
        assert!(auxiliary_traffic_area.lod2_multi_surface().is_none());
        assert_eq!(auxiliary_traffic_area.generic_attributes().len(), 1);
    }
}
