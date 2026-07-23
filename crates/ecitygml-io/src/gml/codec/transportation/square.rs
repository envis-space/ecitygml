use crate::Error;
use crate::gml::codec::transportation::abstract_transportation_space::{
    deserialize_abstract_transportation_space, serialize_abstract_transportation_space,
};
use crate::gml::util::CityGmlElement;
use ecitygml_core::model::transportation::values::{
    SquareClassValue, SquareFunctionValue, SquareUsageValue,
};
use ecitygml_core::model::transportation::{AsAbstractTransportationSpace, Square};
use egml::io::codec::basic::GmlCode;
use egml::io::util::{
    Formatting, XmlNode, XmlNodeContent, extract_xml_element_spans, serialize_inner,
};
use egml::model::basic_types::Code;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_square(xml_document: &[u8]) -> Result<Square, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let (abstract_transportation_space_result, parsed_result) = rayon::join(
        || deserialize_abstract_transportation_space(xml_document, &spans),
        || de::from_reader::<_, GmlSquare>(xml_document).map_err(Error::from),
    );
    let abstract_transportation_space = abstract_transportation_space_result?;
    let parsed = parsed_result?;

    let mut square = Square::from_abstract_transportation_space(abstract_transportation_space);
    square.set_class_opt(parsed.class.map(Code::from).map(SquareClassValue::from));
    square.set_functions(
        parsed
            .functions
            .into_iter()
            .map(Code::from)
            .map(SquareFunctionValue::from)
            .collect(),
    );
    square.set_usages(
        parsed
            .usages
            .into_iter()
            .map(Code::from)
            .map(SquareUsageValue::from)
            .collect(),
    );

    Ok(square)
}

pub fn serialize_square(square: &Square, formatting: Formatting) -> Result<XmlNode, Error> {
    let mut xml_node_parts = serialize_abstract_transportation_space(
        square.abstract_transportation_space(),
        formatting,
    )?;

    if let Some(raw) = serialize_inner(GmlSquare::from(square), formatting)? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(XmlNode::new(CityGmlElement::Square.into(), xml_node_parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlSquare {
    #[serde(
        rename(serialize = "tran:class", deserialize = "class"),
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<GmlCode>,

    #[serde(rename(serialize = "tran:function", deserialize = "function"), default)]
    pub functions: Vec<GmlCode>,

    #[serde(rename(serialize = "tran:usage", deserialize = "usage"), default)]
    pub usages: Vec<GmlCode>,
}

impl From<&Square> for GmlSquare {
    fn from(item: &Square) -> Self {
        Self {
            class: item.class().map(SquareClassValue::code).map(Into::into),
            functions: item
                .functions()
                .iter()
                .map(SquareFunctionValue::code)
                .map(Into::into)
                .collect(),
            usages: item
                .usages()
                .iter()
                .map(SquareUsageValue::code)
                .map(Into::into)
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::{AsAbstractCityObject, AsAbstractFeature, AsAbstractSpace};
    use egml::model::base::Id;

    #[test]
    fn test_deserialize_basic_square() {
        let xml_document = b"
    <tran:Square gml:id=\"UUID_8a13804c-cbd7-3a2f-9d46-e4d528a4bb4f\">
      <genericAttribute>
        <gen:StringAttribute>
          <gen:name>identifier_new_section</gen:name>
          <gen:value>abc</gen:value>
        </gen:StringAttribute>
      </genericAttribute>
      <tran:class>1000</tran:class>
    </tran:Square>";

        let square = deserialize_square(xml_document).expect("should work");

        assert_eq!(
            square.feature_id(),
            &Id::try_from("UUID_8a13804c-cbd7-3a2f-9d46-e4d528a4bb4f").expect("should work")
        );

        assert!(square.lod2_multi_surface().is_none());
        assert_eq!(square.generic_attributes().len(), 1);
        assert_eq!(square.class().unwrap().code().value(), "1000");
    }
}
