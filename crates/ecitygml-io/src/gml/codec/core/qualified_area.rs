use ecitygml_core::model::core::QualifiedArea;
use ecitygml_core::model::core::values::QualifiedAreaTypeValue;
use egml::io::codec::basic::GmlCode;
use egml::io::codec::measures::GmlArea;
use egml::model::basic_types::Code;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlQualifiedArea {
    #[serde(rename(serialize = "area", deserialize = "area"))]
    pub area: GmlArea,

    #[serde(rename = "typeOfArea")]
    pub type_of_area: GmlCode,
}

impl From<GmlQualifiedArea> for QualifiedArea {
    fn from(item: GmlQualifiedArea) -> Self {
        Self::new(
            item.area.into(),
            QualifiedAreaTypeValue::from(Code::from(item.type_of_area)),
        )
    }
}

impl From<&QualifiedArea> for GmlQualifiedArea {
    fn from(item: &QualifiedArea) -> Self {
        Self {
            area: item.area().into(),
            type_of_area: item.type_of_area().code().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egml::model::basic_types::Code;
    use egml::model::measures::Area;
    use quick_xml::{de, se};

    #[test]
    fn test_deserialize_qualified_area() {
        let xml_document = b"<QualifiedArea>
    <area uom=\"m2\">5.0</area>
    <typeOfArea>RoadArea</typeOfArea>
</QualifiedArea>";

        let gml_qualified_area: GmlQualifiedArea =
            de::from_reader(xml_document.as_ref()).expect("should work");
        let qualified_area = QualifiedArea::from(gml_qualified_area);

        assert_eq!(qualified_area.area().value(), 5.0);
        assert_eq!(qualified_area.area().uom(), "m2");
        assert_eq!(qualified_area.type_of_area().code().value(), "RoadArea");
    }

    #[test]
    fn test_serialize_qualified_area() {
        let qualified_area = QualifiedArea::new(
            Area::new(5.0, "m2"),
            QualifiedAreaTypeValue::from(Code::new("RoadArea")),
        );

        let xml =
            se::to_string_with_root("QualifiedArea", &GmlQualifiedArea::from(&qualified_area))
                .expect("should serialize");

        assert!(xml.contains("QualifiedArea"));
        assert!(xml.contains("area"));
        assert!(xml.contains("uom=\"m2\""));
        assert!(xml.contains('5'));
        assert!(xml.contains("typeOfArea"));
        assert!(xml.contains("RoadArea"));
    }

    #[test]
    fn test_round_trip_qualified_area() {
        let xml_document = b"<QualifiedArea>
    <area uom=\"m2\">5.0</area>
    <typeOfArea>RoadArea</typeOfArea>
</QualifiedArea>";

        let gml_qualified_area: GmlQualifiedArea =
            de::from_reader(xml_document.as_ref()).expect("should work");
        let qualified_area = QualifiedArea::from(gml_qualified_area);

        let xml =
            se::to_string_with_root("QualifiedArea", &GmlQualifiedArea::from(&qualified_area))
                .expect("should serialize");

        let round_tripped_gml: GmlQualifiedArea = de::from_str(&xml).expect("should work");
        let round_tripped = QualifiedArea::from(round_tripped_gml);

        assert_eq!(qualified_area, round_tripped);
    }
}
