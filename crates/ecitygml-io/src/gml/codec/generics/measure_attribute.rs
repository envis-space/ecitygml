use crate::Error;
use crate::Error::AttributeWithoutName;
use ecitygml_core::model::generics::MeasureAttribute;
use egml::io::codec::basic::GmlMeasure;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GmlMeasureAttribute {
    #[serde(rename(serialize = "gen:name", deserialize = "name"))]
    pub name: String,
    #[serde(rename(serialize = "gen:value", deserialize = "value"))]
    pub value: GmlMeasure,
}

impl TryFrom<GmlMeasureAttribute> for MeasureAttribute {
    type Error = Error;

    fn try_from(item: GmlMeasureAttribute) -> Result<Self, Self::Error> {
        if item.name.is_empty() {
            return Err(AttributeWithoutName("string attribute".to_string()));
        }

        Ok(Self {
            name: item.name,
            value: item.value.into(),
        })
    }
}

impl From<&MeasureAttribute> for GmlMeasureAttribute {
    fn from(attr: &MeasureAttribute) -> Self {
        Self {
            name: attr.name.clone(),
            value: GmlMeasure {
                uom: attr.value.uom.clone(),
                value: attr.value.value,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de;

    #[test]
    fn test_deserialize_measure_attribute() {
        let xml_document = b"
        <gen:MeasureAttribute>
          <gen:name>GrossPlannedArea</gen:name>
          <gen:value uom=\"m2\">120.0</gen:value>
        </gen:MeasureAttribute>";

        let gml_measure_attribute: GmlMeasureAttribute =
            de::from_reader(xml_document.as_ref()).expect("should work");
        let measure_attribute = MeasureAttribute::try_from(gml_measure_attribute).unwrap();

        assert_eq!(measure_attribute.name, "GrossPlannedArea");
        assert_eq!(measure_attribute.value.uom, "m2");
        assert_eq!(measure_attribute.value.value, 120.0);
    }
}
