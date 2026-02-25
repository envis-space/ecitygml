use crate::Error;
use crate::gml::parser::generics::GmlStringAttribute;
use crate::gml::parser::generics::double_attribute::GmlDoubleAttribute;
use crate::gml::parser::generics::int_attribute::GmlIntAttribute;
use crate::gml::parser::generics::measure_attribute::GmlMeasureAttribute;
use ecitygml_core::model::generics::{
    DoubleAttribute, GenericAttributeKind, IntAttribute, MeasureAttribute, StringAttribute,
};
use quick_xml::de;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GmlGenericAttributeKind {
    StringAttribute(GmlStringAttribute),
    IntAttribute(GmlIntAttribute),
    DoubleAttribute(GmlDoubleAttribute),
    MeasureAttribute(GmlMeasureAttribute),
}

impl TryFrom<GmlGenericAttributeKind> for GenericAttributeKind {
    type Error = Error;

    fn try_from(value: GmlGenericAttributeKind) -> Result<Self, Self::Error> {
        match value {
            GmlGenericAttributeKind::StringAttribute(string_attr) => {
                let core_attr = StringAttribute::try_from(string_attr)?;
                Ok(GenericAttributeKind::StringAttribute(core_attr))
            }
            GmlGenericAttributeKind::IntAttribute(int_attr) => {
                let core_attr = IntAttribute::try_from(int_attr)?;
                Ok(GenericAttributeKind::IntAttribute(core_attr))
            }
            GmlGenericAttributeKind::DoubleAttribute(double_attr) => {
                let core_attr = DoubleAttribute::try_from(double_attr)?;
                Ok(GenericAttributeKind::DoubleAttribute(core_attr))
            }
            GmlGenericAttributeKind::MeasureAttribute(measure_attr) => {
                let core_attr = MeasureAttribute::try_from(measure_attr)?;
                Ok(GenericAttributeKind::MeasureAttribute(core_attr))
            }
        }
    }
}

pub fn parse_generic_attribute(xml_document: &[u8]) -> Result<GenericAttributeKind, Error> {
    let parsed_attribute: GmlGenericAttributeKind = de::from_reader(xml_document)?;
    let attribute = GenericAttributeKind::try_from(parsed_attribute)?;

    Ok(attribute)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_generic_attribute_basic() {
        let xml_document = b"<gen:StringAttribute>
    <gen:name>attribute_name</gen:name>
    <gen:value>1100</gen:value>
  </gen:StringAttribute>";

        let generic_attribute = parse_generic_attribute(xml_document).expect("should work");

        assert_eq!(generic_attribute.name(), "attribute_name");
        assert_eq!(
            generic_attribute.as_string().expect("must be string").value,
            "1100"
        );
    }

    #[test]
    fn test_parse_string_attribute_basic() {
        let xml_document = b"<gen:StringAttribute>
          <gen:name>attribute_name</gen:name>
          <gen:value>1100</gen:value>
        </gen:StringAttribute>";

        let generic_attribute = parse_generic_attribute(xml_document).expect("should work");

        match generic_attribute {
            GenericAttributeKind::StringAttribute(x) => {
                assert_eq!(x.name, "attribute_name");
                assert_eq!(x.value, "1100");
            }
            other => panic!("Expected Double attribute, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_int_attribute_basic() {
        let xml_document = b"<gen:IntAttribute>
          <gen:name>attribute_name</gen:name>
          <gen:value>1100</gen:value>
        </gen:IntAttribute>";

        let generic_attribute = parse_generic_attribute(xml_document).expect("should work");

        match generic_attribute {
            GenericAttributeKind::IntAttribute(x) => {
                assert_eq!(x.name, "attribute_name");
                assert_eq!(x.value, 1100);
            }
            other => panic!("Expected Double attribute, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_double_attribute_basic() {
        let xml_document = b"<gen:DoubleAttribute>
          <gen:name>attribute_name</gen:name>
          <gen:value>42.2</gen:value>
        </gen:DoubleAttribute>";

        let generic_attribute = parse_generic_attribute(xml_document).expect("should work");

        match generic_attribute {
            GenericAttributeKind::DoubleAttribute(x) => {
                assert_eq!(x.name, "attribute_name");
                assert_eq!(x.value, 42.2);
            }
            other => panic!("Expected Double attribute, got {:?}", other),
        }
    }
}
