use crate::Error;
use crate::gml::codec::core::{
    deserialize_abstract_thematic_surface, serialize_abstract_thematic_surface,
};
use crate::gml::util::CityGmlElement;
use ecitygml_core::model::core::AsAbstractThematicSurface;
use ecitygml_core::model::transportation::Marking;
use ecitygml_core::model::transportation::values::MarkingClassValue;
use egml::io::codec::basic::GmlCode;
use egml::io::util::{
    Formatting, XmlNode, XmlNodeContent, extract_xml_element_spans, serialize_inner,
};
use egml::model::basic_types::Code;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_marking(xml_document: &[u8]) -> Result<Marking, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let (abstract_thematic_surface_result, parsed_result) = rayon::join(
        || deserialize_abstract_thematic_surface(xml_document, &spans),
        || de::from_reader::<_, GmlMarking>(xml_document).map_err(Error::from),
    );
    let abstract_thematic_surface = abstract_thematic_surface_result?;
    let parsed = parsed_result?;

    let mut marking = Marking::from_abstract_thematic_surface(abstract_thematic_surface);
    marking.set_class_opt(parsed.class.map(Code::from).map(MarkingClassValue::from));

    Ok(marking)
}

pub fn serialize_marking(marking: &Marking, formatting: Formatting) -> Result<XmlNode, Error> {
    let mut parts =
        serialize_abstract_thematic_surface(marking.abstract_thematic_surface(), formatting)?;

    if let Some(raw) = serialize_inner(GmlMarking::from(marking), formatting)? {
        parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(XmlNode::new(CityGmlElement::Marking.into(), parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlMarking {
    #[serde(
        rename(serialize = "tran:class", deserialize = "class"),
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<GmlCode>,
}

impl From<&Marking> for GmlMarking {
    fn from(item: &Marking) -> Self {
        Self {
            class: item.class().map(MarkingClassValue::code).map(Into::into),
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
        let xml_document = b"<tran:Marking gml:id=\"UUID_312e01da-5c89-3f89-b8c0-3e0a8bafecbb\">
                  <genericAttribute>
                    <gen:StringAttribute>
                      <gen:name>opendrive_lane_type</gen:name>
                      <gen:value>BORDER</gen:value>
                    </gen:StringAttribute>
                  </genericAttribute>
                </tran:Marking>";

        let marking = deserialize_marking(xml_document).expect("should work");

        assert_eq!(
            marking.feature_id(),
            &Id::try_from("UUID_312e01da-5c89-3f89-b8c0-3e0a8bafecbb").expect("should work")
        );
        assert!(marking.lod2_multi_surface().is_none());
        assert_eq!(marking.generic_attributes().len(), 1);
    }
}
