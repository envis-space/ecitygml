use crate::gml::codec::core::qualified_area::GmlQualifiedArea;
use ecitygml_core::model::core::QualifiedArea;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAreaProperty {
    #[serde(rename = "QualifiedArea")]
    pub qualified_area: GmlQualifiedArea,
}

impl From<GmlAreaProperty> for QualifiedArea {
    fn from(item: GmlAreaProperty) -> Self {
        Self::from(item.qualified_area)
    }
}

impl From<&QualifiedArea> for GmlAreaProperty {
    fn from(item: &QualifiedArea) -> Self {
        Self {
            qualified_area: GmlQualifiedArea::from(item),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::core::values::QualifiedAreaTypeValue;
    use egml::model::basic_types::Code;
    use egml::model::measures::Area;
    use quick_xml::{de, se};

    #[test]
    fn test_deserialize_area_property() {
        let xml_document = b"<area>
    <QualifiedArea>
        <area uom=\"m2\">5.0</area>
        <typeOfArea>RoadArea</typeOfArea>
    </QualifiedArea>
</area>";

        let gml_area_property: GmlAreaProperty =
            de::from_reader(xml_document.as_ref()).expect("should work");
        let qualified_area = QualifiedArea::from(gml_area_property);

        assert_eq!(qualified_area.area().value(), 5.0);
        assert_eq!(qualified_area.area().uom(), "m2");
        assert_eq!(qualified_area.type_of_area().code().value(), "RoadArea");
    }

    #[test]
    fn test_serialize_area_property() {
        let qualified_area = QualifiedArea::new(
            Area::new(5.0, "m2"),
            QualifiedAreaTypeValue::from(Code::new("RoadArea")),
        );

        let xml = se::to_string_with_root("area", &GmlAreaProperty::from(&qualified_area))
            .expect("should serialize");

        assert!(xml.contains("QualifiedArea"));
        assert!(xml.contains("uom=\"m2\""));
        assert!(xml.contains("typeOfArea"));
        assert!(xml.contains("RoadArea"));
    }

    #[test]
    fn test_round_trip_area_property() {
        let xml_document = b"<area>
    <QualifiedArea>
        <area uom=\"m2\">5.0</area>
        <typeOfArea>RoadArea</typeOfArea>
    </QualifiedArea>
</area>";

        let gml_area_property: GmlAreaProperty =
            de::from_reader(xml_document.as_ref()).expect("should work");
        let qualified_area = QualifiedArea::from(gml_area_property);

        let xml = se::to_string_with_root("area", &GmlAreaProperty::from(&qualified_area))
            .expect("should serialize");

        let round_tripped_gml: GmlAreaProperty = de::from_str(&xml).expect("should work");
        let round_tripped = QualifiedArea::from(round_tripped_gml);

        assert_eq!(qualified_area, round_tripped);
    }
}
