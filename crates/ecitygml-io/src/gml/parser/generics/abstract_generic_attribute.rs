use crate::Error;
use crate::gml::parser::generics::GmlStringAttribute;
use crate::gml::parser::generics::double_attribute::GmlDoubleAttribute;
use crate::gml::parser::generics::generic_attribute_set::GmlGenericAttributeSet;
use crate::gml::parser::generics::int_attribute::GmlIntAttribute;
use crate::gml::parser::generics::measure_attribute::GmlMeasureAttribute;
use ecitygml_core::model::generics::{
    DoubleAttribute, GenericAttributeKind, GenericAttributeSet, IntAttribute, MeasureAttribute,
    StringAttribute,
};
use quick_xml::{de, se};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GmlGenericAttributeKind {
    #[serde(rename(serialize = "gen:StringAttribute", deserialize = "StringAttribute"))]
    StringAttribute(GmlStringAttribute),
    #[serde(rename(serialize = "gen:IntAttribute", deserialize = "IntAttribute"))]
    IntAttribute(GmlIntAttribute),
    #[serde(rename(serialize = "gen:DoubleAttribute", deserialize = "DoubleAttribute"))]
    DoubleAttribute(GmlDoubleAttribute),
    #[serde(rename(serialize = "gen:MeasureAttribute", deserialize = "MeasureAttribute"))]
    MeasureAttribute(GmlMeasureAttribute),
    #[serde(rename(
        serialize = "gen:GenericAttributeSet",
        deserialize = "GenericAttributeSet"
    ))]
    GenericAttributeSet(GmlGenericAttributeSet),
}

impl TryFrom<GmlGenericAttributeKind> for GenericAttributeKind {
    type Error = Error;

    fn try_from(value: GmlGenericAttributeKind) -> Result<Self, Self::Error> {
        match value {
            GmlGenericAttributeKind::StringAttribute(x) => {
                let core_attr = StringAttribute::try_from(x)?;
                Ok(GenericAttributeKind::StringAttribute(core_attr))
            }
            GmlGenericAttributeKind::IntAttribute(x) => {
                let core_attr = IntAttribute::try_from(x)?;
                Ok(GenericAttributeKind::IntAttribute(core_attr))
            }
            GmlGenericAttributeKind::DoubleAttribute(x) => {
                let core_attr = DoubleAttribute::try_from(x)?;
                Ok(GenericAttributeKind::DoubleAttribute(core_attr))
            }
            GmlGenericAttributeKind::MeasureAttribute(x) => {
                let core_attr = MeasureAttribute::try_from(x)?;
                Ok(GenericAttributeKind::MeasureAttribute(core_attr))
            }
            GmlGenericAttributeKind::GenericAttributeSet(x) => {
                let core_attr = GenericAttributeSet::try_from(x)?;
                Ok(GenericAttributeKind::GenericAttributeSet(core_attr))
            }
        }
    }
}

impl From<&GenericAttributeKind> for GmlGenericAttributeKind {
    fn from(attr: &GenericAttributeKind) -> Self {
        match attr {
            GenericAttributeKind::StringAttribute(x) => {
                GmlGenericAttributeKind::StringAttribute(GmlStringAttribute::from(x))
            }
            GenericAttributeKind::IntAttribute(x) => {
                GmlGenericAttributeKind::IntAttribute(GmlIntAttribute::from(x))
            }
            GenericAttributeKind::DoubleAttribute(x) => {
                GmlGenericAttributeKind::DoubleAttribute(GmlDoubleAttribute::from(x))
            }
            GenericAttributeKind::MeasureAttribute(x) => {
                GmlGenericAttributeKind::MeasureAttribute(GmlMeasureAttribute::from(x))
            }
            GenericAttributeKind::GenericAttributeSet(x) => {
                GmlGenericAttributeKind::GenericAttributeSet(GmlGenericAttributeSet::from(x))
            }
        }
    }
}

pub fn deserialize_generic_attribute(xml_document: &[u8]) -> Result<GenericAttributeKind, Error> {
    let parsed_attribute: GmlGenericAttributeKind = de::from_reader(xml_document)?;
    let attribute = GenericAttributeKind::try_from(parsed_attribute)?;

    Ok(attribute)
}

pub fn serialize_generic_attribute(attr: &GenericAttributeKind) -> Result<String, Error> {
    let gml = GmlGenericAttributeKind::from(attr);
    Ok(se::to_string(&gml)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use egml::model::basic::Measure;
    // --- Serialization tests ---

    #[test]
    fn test_serialize_string_attribute() {
        let attr = GenericAttributeKind::StringAttribute(StringAttribute {
            name: "material".to_string(),
            value: "concrete".to_string(),
        });

        let xml = serialize_generic_attribute(&attr).unwrap();

        assert!(
            xml.contains("<gen:StringAttribute>"),
            "missing root element: {xml}"
        );
        assert!(
            xml.contains("<gen:name>material</gen:name>"),
            "missing name: {xml}"
        );
        assert!(
            xml.contains("<gen:value>concrete</gen:value>"),
            "missing value: {xml}"
        );
    }

    #[test]
    fn test_serialize_int_attribute() {
        let attr = GenericAttributeKind::IntAttribute(IntAttribute {
            name: "storeys".to_string(),
            value: 4,
        });

        let xml = serialize_generic_attribute(&attr).unwrap();

        assert!(
            xml.contains("<gen:IntAttribute>"),
            "missing root element: {xml}"
        );
        assert!(
            xml.contains("<gen:name>storeys</gen:name>"),
            "missing name: {xml}"
        );
        assert!(
            xml.contains("<gen:value>4</gen:value>"),
            "missing value: {xml}"
        );
    }

    #[test]
    fn test_serialize_double_attribute() {
        let attr = GenericAttributeKind::DoubleAttribute(DoubleAttribute {
            name: "height".to_string(),
            value: 12.5,
        });

        let xml = serialize_generic_attribute(&attr).unwrap();

        assert!(
            xml.contains("<gen:DoubleAttribute>"),
            "missing root element: {xml}"
        );
        assert!(
            xml.contains("<gen:name>height</gen:name>"),
            "missing name: {xml}"
        );
        assert!(
            xml.contains("<gen:value>12.5</gen:value>"),
            "missing value: {xml}"
        );
    }

    #[test]
    fn test_serialize_measure_attribute() {
        let attr = GenericAttributeKind::MeasureAttribute(MeasureAttribute {
            name: "GrossFloorArea".to_string(),
            value: Measure {
                uom: "m2".to_string(),
                value: 250.0,
            },
        });

        let xml = serialize_generic_attribute(&attr).unwrap();

        assert!(
            xml.contains("<gen:MeasureAttribute>"),
            "missing root element: {xml}"
        );
        assert!(
            xml.contains("<gen:name>GrossFloorArea</gen:name>"),
            "missing name: {xml}"
        );
        assert!(xml.contains("uom=\"m2\""), "missing uom attribute: {xml}");
        assert!(xml.contains("250"), "missing value: {xml}");
    }

    #[test]
    fn test_serialize_generic_attribute_set() {
        let string_attribute: GenericAttributeKind =
            StringAttribute::new("GrossFloorArea".to_string(), "250".to_string()).into();
        let int_attribute: GenericAttributeKind = IntAttribute::new("floors".to_string(), 7).into();
        let generic_attribute_set: GenericAttributeKind = GenericAttributeSet::new(
            "generic_attribute_name".to_string(),
            vec![string_attribute, int_attribute],
        )
        .into();

        let xml = serialize_generic_attribute(&generic_attribute_set).unwrap();

        assert!(
            xml.contains("<gen:GenericAttributeSet>"),
            "missing root element: {xml}"
        );
        assert!(
            xml.contains("<gen:name>generic_attribute_name</gen:name>"),
            "missing name: {xml}"
        );
        assert!(
            xml.contains("<gen:StringAttribute>"),
            "missing element: {xml}"
        );
        assert!(xml.contains("250"), "missing value: {xml}");
    }

    #[test]
    fn test_serialize_string_attribute_round_trip() {
        let original = GenericAttributeKind::StringAttribute(StringAttribute {
            name: "construction_method".to_string(),
            value: "prefab".to_string(),
        });

        let xml = serialize_generic_attribute(&original).unwrap();
        let recovered = deserialize_generic_attribute(xml.as_bytes()).unwrap();

        assert_eq!(original, recovered);
    }

    #[test]
    fn test_serialize_int_attribute_round_trip() {
        let original = GenericAttributeKind::IntAttribute(IntAttribute {
            name: "floors".to_string(),
            value: 7,
        });

        let xml = serialize_generic_attribute(&original).unwrap();
        let recovered = deserialize_generic_attribute(xml.as_bytes()).unwrap();

        assert_eq!(original, recovered);
    }

    #[test]
    fn test_serialize_double_attribute_round_trip() {
        let original = GenericAttributeKind::DoubleAttribute(DoubleAttribute {
            name: "area".to_string(),
            value: 99.9,
        });

        let xml = serialize_generic_attribute(&original).unwrap();
        let recovered = deserialize_generic_attribute(xml.as_bytes()).unwrap();

        assert_eq!(original, recovered);
    }

    #[test]
    fn test_serialize_measure_attribute_round_trip() {
        let original = GenericAttributeKind::MeasureAttribute(MeasureAttribute {
            name: "volume".to_string(),
            value: Measure {
                uom: "m3".to_string(),
                value: 1500.0,
            },
        });

        let xml = serialize_generic_attribute(&original).unwrap();
        let recovered = deserialize_generic_attribute(xml.as_bytes()).unwrap();

        assert_eq!(original, recovered);
    }

    // --- Deserialization tests ---

    #[test]
    fn test_deserialize_string_attribute_via_generic_kind() {
        let xml_document = b"<gen:StringAttribute>
    <gen:name>attribute_name</gen:name>
    <gen:value>1100</gen:value>
  </gen:StringAttribute>";

        let generic_attribute = deserialize_generic_attribute(xml_document).expect("should work");

        assert_eq!(generic_attribute.name(), "attribute_name");
        assert_eq!(
            generic_attribute.as_string().expect("must be string").value,
            "1100"
        );
    }

    #[test]
    fn test_deserialize_string_attribute() {
        let xml_document = b"<gen:StringAttribute>
          <gen:name>attribute_name</gen:name>
          <gen:value>1100</gen:value>
        </gen:StringAttribute>";

        let generic_attribute = deserialize_generic_attribute(xml_document).expect("should work");

        match generic_attribute {
            GenericAttributeKind::StringAttribute(x) => {
                assert_eq!(x.name, "attribute_name");
                assert_eq!(x.value, "1100");
            }
            other => panic!("Expected Double attribute, got {:?}", other),
        }
    }

    #[test]
    fn test_deserialize_int_attribute() {
        let xml_document = b"<gen:IntAttribute>
          <gen:name>attribute_name</gen:name>
          <gen:value>1100</gen:value>
        </gen:IntAttribute>";

        let generic_attribute = deserialize_generic_attribute(xml_document).expect("should work");

        match generic_attribute {
            GenericAttributeKind::IntAttribute(x) => {
                assert_eq!(x.name, "attribute_name");
                assert_eq!(x.value, 1100);
            }
            other => panic!("Expected Double attribute, got {:?}", other),
        }
    }

    #[test]
    fn test_deserialize_double_attribute() {
        let xml_document = b"<gen:DoubleAttribute>
          <gen:name>attribute_name</gen:name>
          <gen:value>42.2</gen:value>
        </gen:DoubleAttribute>";

        let generic_attribute = deserialize_generic_attribute(xml_document).expect("should work");

        match generic_attribute {
            GenericAttributeKind::DoubleAttribute(x) => {
                assert_eq!(x.name, "attribute_name");
                assert_eq!(x.value, 42.2);
            }
            other => panic!("Expected Double attribute, got {:?}", other),
        }
    }

    #[test]
    fn test_deserialize_generic_attribute_set() {
        let xml_document = "<gen:GenericAttributeSet>
          <gen:name>generic_attribute_name</gen:name>
          <gen:genericAttribute>
            <gen:StringAttribute>
              <gen:name>MyStringAttributeName</gen:name>
              <gen:value>abc</gen:value>
            </gen:StringAttribute>
          </gen:genericAttribute>
          <gen:genericAttribute>
            <gen:IntAttribute>
              <gen:name>MyIntAttributeName</gen:name>
              <gen:value>-123</gen:value>
            </gen:IntAttribute>
          </gen:genericAttribute>
      </gen:GenericAttributeSet>";

        let generic_attribute_kind =
            deserialize_generic_attribute(xml_document.as_ref()).expect("should work");

        let GenericAttributeKind::GenericAttributeSet(generic_attribute_set) =
            generic_attribute_kind
        else {
            panic!("Expected GenericAttributeSet");
        };
        assert_eq!(generic_attribute_set.name, "generic_attribute_name");
        assert_eq!(generic_attribute_set.generic_attributes.len(), 2);

        let first_generic_attribute = &generic_attribute_set.generic_attributes[0];
        let GenericAttributeKind::StringAttribute(string_attribute) = first_generic_attribute
        else {
            panic!("Expected StringAttribute");
        };
        assert_eq!(string_attribute.name, "MyStringAttributeName");
        assert_eq!(string_attribute.value, "abc");

        let second_generic_attribute = &generic_attribute_set.generic_attributes[1];
        let GenericAttributeKind::IntAttribute(int_attribute) = second_generic_attribute else {
            panic!("Expected IntAttribute");
        };
        assert_eq!(int_attribute.name, "MyIntAttributeName");
        assert_eq!(int_attribute.value, -123);
    }
}
